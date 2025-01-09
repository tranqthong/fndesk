use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    process::Command,
    rc::Rc,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

use crate::utils::{get_dir_items, get_parent_dir};

#[derive(Debug, PartialEq)]
pub enum AppState {
    Running,
    Copying,
    Moving,
    Exit,
}

pub struct DirListState {
    pub state: ListState,
    pub items: Vec<DirEntry>,
}

impl DirListState {
    fn new(items: Vec<DirEntry>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn set_items(&mut self, items: Vec<DirEntry>) {
        self.items = items;
        self.state = ListState::default();
    }
}

pub struct App {
    pub app_state: AppState,
    pub current_dir: PathBuf,
    pub parent_dir: PathBuf,
    pub dir_items: DirListState,
    pub show_hidden: bool,
    pub status_text: String,
    pub clipboard: Option<PathBuf>,
}

impl App {
    pub fn new(init_dir: PathBuf) -> Self {
        let init_dir_ref = Rc::new(init_dir);
        App {
            app_state: AppState::Running,
            current_dir: init_dir_ref.to_path_buf(),
            parent_dir: get_parent_dir(&Rc::clone(&init_dir_ref)),
            dir_items: DirListState::new(get_dir_items(&Rc::clone(&init_dir_ref), &false)),
            show_hidden: false,
            status_text: String::from("Status Text Placeholder"),
            clipboard: None,
        }
    }

    pub fn handle_keypress(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit_app(),
            KeyCode::Char('h') => self.toggle_hidden(),
            KeyCode::Char('c') => self.copy_selected(),
            KeyCode::Enter | KeyCode::Char(' ') => self.open_selected(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Tab | KeyCode::BackTab => self.switch_panes(),
            KeyCode::Esc | KeyCode::Backspace => self.nav_up_dir(),
            KeyCode::Delete => self.delete_selected(),
            _ => {}
        }
    }

    pub fn refresh_dirlist(&mut self) {
        self.dir_items
            .set_items(get_dir_items(&self.current_dir, &self.show_hidden));
        self.auto_select_first();
    }

    fn auto_select_first(&mut self) {
        match self.dir_items.state.selected() {
            Some(_) => (),
            None => {
                self.dir_items.state.select_first();
            }
        }
    }

    fn quit_app(&mut self) {
        self.app_state = AppState::Exit;
    }

    fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.dir_items
            .set_items(get_dir_items(&self.current_dir, &self.show_hidden));
    }

    fn move_cursor_left(&mut self) {
        // TODO implement when two column pane is implemented
    }

    fn move_cursor_right(&mut self) {
        // TODO implement when two column pane is implemented
    }

    fn move_cursor_up(&mut self) {
        self.dir_items.state.select_previous();
    }

    fn move_cursor_down(&mut self) {
        self.dir_items.state.select_next();
    }

    fn switch_panes(&mut self) {
        // TODO implement when two column pane is implemented
    }

    fn copy_selected(&mut self) {
        match self.dir_items.state.selected() {
            Some(idx) => {
                self.clipboard = Some(self.dir_items.items[idx].path());
                self.app_state = AppState::Copying;
                self.status_text = format!("Copying {:?}", &self.dir_items.items[idx])
            }
            None => self.status_text = "No file/directory selected!".to_string(),
        }
    }

    fn move_selected(&mut self) {
        match self.dir_items.state.selected() {
            Some(idx) => {
                self.clipboard = Some(self.dir_items.items[idx].path());
                self.app_state = AppState::Moving;
                self.status_text = format!("Cut {:?}", &self.dir_items.items[idx])
            }
            None => self.status_text = "No file/directory selected!".to_string(),
        }
    }

    fn paste_item(&mut self) {
        // paste the file/dir
        if self.clipboard.is_some() {
            let source_path = self.clipboard.clone().unwrap();
            let file_name = source_path.file_name();

            let mut target_path = PathBuf::new();
            target_path.push(&self.current_dir);
            target_path.push(file_name.unwrap());

            if target_path.exists() {
                // TODO ask if user wants to overwrite
                self.status_text = "Overwrite file Y/n?".to_string();
            }

            let file_copy = fs::copy(source_path, target_path);
            match file_copy {
                Ok(_) => {
                    self.status_text = "Pasted from clipboard".to_string();
                    self.clipboard = None;
                }
                Err(e) => self.status_text = format!("Unable to paste: {e:?}"),
            }

            if self.app_state == AppState::Moving {
                let file_move = fs::remove_file(self.clipboard.clone().unwrap());
                match file_move {
                    Ok(_) => {}
                    Err(e) => {
                        self.status_text =
                            format!("Unable to remove old file or old file no longer exists: {e:?}")
                    }
                }
            }
            self.app_state = AppState::Running;
            self.refresh_dirlist();
        }
    }

    fn delete_selected(&mut self) {
        match self.dir_items.state.selected() {
            Some(idx) => {
                let selected_entry = &self.dir_items.items[idx];
                match trash::delete(selected_entry.path()) {
                    Ok(_) => {
                        self.status_text = format!("Deleted {:?}", selected_entry.file_name());
                    }
                    Err(e) => {
                        self.status_text = format!("Error {e:?}");
                    }
                };
                self.refresh_dirlist();
            }
            None => self.status_text = "No file/directory selected!".to_string(),
        }
    }

    fn nav_up_dir(&mut self) {
        let new_current_dirpath = self.parent_dir.clone();
        let new_parent_dirpath = get_parent_dir(&new_current_dirpath);

        // we only want to be able to navigate up to a parent if we're not already in the root directory
        if new_parent_dirpath != self.current_dir {
            self.parent_dir = new_parent_dirpath;
            self.current_dir = new_current_dirpath;

            self.refresh_dirlist();
        }
    }

    fn open_selected(&mut self) {
        let selected_idx = self.dir_items.state.selected().unwrap_or(0);
        let selected_entry = &self.dir_items.items[selected_idx];

        if selected_entry.metadata().unwrap().is_dir() {
            let new_parent_dir = self.current_dir.clone();
            self.current_dir = selected_entry.path();
            self.parent_dir = new_parent_dir.to_owned();
            self.refresh_dirlist();
        } else {
            let selected_entry_path = selected_entry.path().to_str().unwrap().to_owned();

            // TODO need to handle opening files on Windows/Mac in the future
            match Command::new("xdg-open").arg(selected_entry_path).output() {
                Ok(_) => (),
                Err(_) => self.status_text = "Unable to open file, check permissions".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        fs::{self, File},
    };

    struct TestContext {
        app: App,
    }

    // TODO
    // move the test file creation and removal out of the individual unit tests and
    // into the setup and teardown constructs
    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown...");
        }
    }

    fn setup() -> TestContext {
        println!("Test setup...");
        let init_dir =
            env::current_dir().expect("Invalid permissions or currenty directory doesn't exists.");
        TestContext {
            app: App::new(init_dir),
        }
    }

    #[test]
    fn test_app_state() {
        let test_app = setup();
        assert_eq!(test_app.app.app_state, AppState::Running)
    }

    #[test]
    fn test_app_exit() {
        let mut test_app = setup();
        test_app.app.quit_app();
        assert_eq!(test_app.app.app_state, AppState::Exit);
    }

    #[test]
    fn test_keypress_space_dir() {}

    #[test]
    fn test_keypress_space_file() {}

    #[test]
    fn test_keypress_del() {
        let test_file_name = "test_file.txt";
        File::create_new(test_file_name).unwrap();

        let mut test_app = setup();
        loop {
            test_app.app.handle_keypress(KeyCode::Down.into());
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_file_name {
                break;
            }
        }
        test_app.app.handle_keypress(KeyCode::Delete.into());

        assert!(File::open(test_file_name).is_err());
    }

    #[test]
    fn test_keypress_copy() {
        let test_file_name = "copy_test.txt";
        File::create_new(test_file_name).unwrap();

        let mut test_app = setup();

        loop {
            test_app.app.handle_keypress(KeyCode::Down.into());
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_file_name {
                break;
            }
        }

        test_app.app.handle_keypress(KeyCode::Char('c').into());

        let result = test_app.app.clipboard.clone();
        let mut expected = PathBuf::new();
        expected.push(env::current_dir().unwrap());
        expected.push(test_file_name);

        assert_eq!(result.unwrap(), expected);

        match fs::remove_file(expected) {
            Ok(_) => println!("Deleted copy_test.txt"),
            Err(e) => println!("{e:?}"),
        };
    }

    #[test]
    fn test_keypress_paste() {
        let test_file_name = "paste_file.txt";
        File::create_new(test_file_name).unwrap();

        let mut test_app = setup();
        loop {
            test_app.app.handle_keypress(KeyCode::Down.into());
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_file_name {
                break;
            }
        }

        test_app.app.handle_keypress(KeyCode::Char('c').into());

        let test_dir = "test_temp";
        fs::create_dir(test_dir).expect("Unable to create temp directory for unit testing");

        let mut test_path = PathBuf::new();
        test_path.push(env::current_dir().unwrap());
        test_path.push(test_dir);

        test_app.app.paste_item();
    }

    #[test]
    fn test_keypress_up() {
        let mut test_app = setup();
        test_app.app.handle_keypress(KeyCode::Up.into());
        let result = test_app.app.dir_items.state.selected();

        assert_ne!(result, None);
    }

    #[test]
    fn test_keypress_down() {
        let mut test_app = setup();
        test_app.app.handle_keypress(KeyCode::Down.into());
        let result = test_app.app.dir_items.state.selected();

        assert_ne!(result, None);
    }

    #[test]
    fn test_keypress_esc() {
        let mut test_app = setup();
        test_app.app.handle_keypress(KeyCode::Esc.into());
        let result = &test_app.app.current_dir;
        let expected = fs::canonicalize("../").unwrap();

        assert_eq!(result, &expected);
    }

    #[test]
    fn test_keypress_backspace() {
        let mut test_app = setup();
        test_app.app.handle_keypress(KeyCode::Backspace.into());
        let result = &test_app.app.current_dir;
        let expected = fs::canonicalize("../").unwrap();

        assert_eq!(result, &expected);
    }
}
