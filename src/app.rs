use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
    process::Command,
};

use crossterm::event::{KeyCode, KeyEvent};
use log::{debug, error};
use ratatui::widgets::ListState;

use crate::{entry, path, status_bar::status_string};

#[derive(Debug, PartialEq)]
pub enum AppState {
    Running,
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
    pub fn new<T: AsRef<Path>>(init_dir: T) -> Self {
        App {
            app_state: AppState::Running,
            current_dir: init_dir.as_ref().to_path_buf(),
            parent_dir: path::get_parent_dir(&init_dir),
            dir_items: DirListState::new(path::get_dir_items(&init_dir, &false)),
            show_hidden: false,
            status_text: String::from("Hello There"),
            clipboard: None,
        }
    }

    pub fn handle_keypress(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit_app(),
            KeyCode::Char('h') => self.toggle_hidden(),
            KeyCode::Char('c') | KeyCode::Char('y') => self.add_selected_to_clipboard(),
            KeyCode::Char('p') => self.copy_from_clipboard(),
            KeyCode::Char('x') => self.move_from_clipboard(),
            KeyCode::Delete => self.trash_selected(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Tab | KeyCode::BackTab => self.switch_panes(),
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right => self.open_selected(),
            KeyCode::Esc | KeyCode::Backspace | KeyCode::Left => self.nav_up_dir(),
            _ => {}
        }
        self.update_status_bar();
    }

    pub fn refresh_dirlist(&mut self) {
        self.dir_items
            .set_items(path::get_dir_items(&self.current_dir, &self.show_hidden));
        self.auto_select_first();
        self.update_status_bar();
    }

    fn auto_select_first(&mut self) {
        match self.dir_items.state.selected() {
            Some(_) => (),
            None => {
                self.dir_items.state.select_first();
            }
        }
    }

    fn update_status_bar(&mut self) {
        if let Some(idx) = self.dir_items.state.selected() {
            if idx < self.dir_items.items.len() {
                self.status_text = status_string(self.dir_items.items[idx].path());
            }
        }
    }

    fn quit_app(&mut self) {
        self.app_state = AppState::Exit;
    }

    fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.dir_items
            .set_items(path::get_dir_items(&self.current_dir, &self.show_hidden));
        self.auto_select_first();
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

    fn add_selected_to_clipboard(&mut self) {
        match self.dir_items.state.selected() {
            Some(idx) => {
                // we replace whatever is currently in the clipboard
                // considering maybe making it a stack/list in the future
                self.clipboard = Some(self.dir_items.items[idx].path());
                debug!("Added {:?} to clipboard", &self.dir_items.items[idx])
            }
            None => debug!("No item selected to be added to clipboard."),
        }
    }

    fn move_from_clipboard(&mut self) {
        if self.clipboard.is_some() {
            let src_path = self.clipboard.as_ref().unwrap();
            if src_path.is_file() {
                let dest_path = entry::append_duplicates(src_path, &self.current_dir);

                entry::copy_file(src_path, &dest_path, true);
            } else if src_path.is_dir() {
                // for directories we will attempt to merge
                // later we can give the user the option on whether or not to merge
                match entry::copy_dir(src_path, &self.current_dir, true) {
                    Ok(_) => (),
                    Err(e) => error!("{e:}"),
                }
            }
        }
        self.refresh_dirlist();
    }

    fn copy_from_clipboard(&mut self) {
        if self.clipboard.is_some() {
            let src_path = self.clipboard.as_ref().unwrap();
            if src_path.is_file() {
                let dest_path = entry::append_duplicates(src_path, &self.current_dir);

                entry::copy_file(src_path, &dest_path, false);
            } else if src_path.is_dir() {
                // for directories we will attempt to merge
                // later we can give the user the option on whether or not to merge
                match entry::copy_dir(src_path, &self.current_dir, false) {
                    Ok(_) => (),
                    Err(e) => error!("{e:}"),
                }
            }
        }
        self.refresh_dirlist();
    }

    fn trash_selected(&mut self) {
        if let Some(idx) = self.dir_items.state.selected() {
            let selected_entry = &self.dir_items.items[idx];
            entry::trash_entry(selected_entry);
            self.refresh_dirlist();
        }
    }

    fn nav_up_dir(&mut self) {
        let new_current_dirpath = self.parent_dir.clone();
        let new_parent_dirpath = path::get_parent_dir(&new_current_dirpath);

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
                Err(e) => debug!("{e:?}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

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
    fn test_keypress_q() {
        let mut test_app = setup();
        test_app.app.handle_keypress(KeyCode::Char('q').into());
        assert_eq!(test_app.app.app_state, AppState::Exit);
    }

    #[test]
    fn test_keypress_space_dir() {}

    #[test]
    fn test_keypress_space_file() {}

    #[test]
    fn test_keypress_del() {
        let test_dir = tempdir().unwrap();
        let test_dirpath = test_dir.path();
        let test_filename = "test_file.txt";
        let test_filepath = test_dir.path().join(test_filename);
        let _test_file = fs::File::create(&test_filepath).unwrap();

        let mut test_app = setup();
        test_app.app.current_dir = test_dirpath.to_path_buf();
        test_app.app.parent_dir = test_dirpath.parent().unwrap().to_path_buf();
        test_app.app.refresh_dirlist();

        loop {
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_filename {
                break;
            }
            test_app.app.handle_keypress(KeyCode::Down.into());
        }
        test_app.app.handle_keypress(KeyCode::Delete.into());

        assert!(fs::File::open(test_filename).is_err());
        test_dir.close().unwrap();
    }

    #[test]
    fn test_keypress_copy() {
        let test_filename = "copy_test.txt";
        let test_dir = tempdir().unwrap();
        let test_dirpath = test_dir.path();
        let test_filepath = test_dir.path().join(test_filename);
        let _test_file = fs::File::create(&test_filepath).unwrap();

        let mut test_app = setup();
        test_app.app.current_dir = test_dirpath.to_path_buf();
        test_app.app.parent_dir = test_dirpath.parent().unwrap().to_path_buf();
        test_app.app.refresh_dirlist();

        loop {
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_filename {
                break;
            }
            test_app.app.handle_keypress(KeyCode::Down.into());
        }

        test_app.app.handle_keypress(KeyCode::Char('c').into());

        let result = test_app.app.clipboard.clone();

        assert_eq!(result.unwrap(), test_filepath);
        test_dir.close().unwrap();
    }

    #[test]
    fn test_keypress_paste() {
        let test_filename = "paste_file.txt";
        let test_dir = tempdir().unwrap();
        let test_dirpath = test_dir.path();
        let test_filepath = test_dir.path().join(test_filename);
        let _test_file = fs::File::create(&test_filepath).unwrap();

        let mut test_app = setup();
        test_app.app.current_dir = test_dirpath.to_path_buf();
        test_app.app.parent_dir = test_dirpath.parent().unwrap().to_path_buf();
        test_app.app.refresh_dirlist();

        loop {
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == test_filename {
                break;
            }
            test_app.app.handle_keypress(KeyCode::Down.into());
        }

        test_app.app.handle_keypress(KeyCode::Char('c').into());

        let dest_dir = tempdir().unwrap();
        let dest_dirpath = dest_dir.path();
        let dest_filepath = dest_dir.path().join(test_filename);
        test_app.app.refresh_dirlist();

        test_app.app.handle_keypress(KeyCode::Left.into());

        // tempdir are hidden so to find our dest dir, we need to turn on hidden dir/files
        test_app.app.handle_keypress(KeyCode::Char('h').into());
        loop {
            let selected_idx = test_app.app.dir_items.state.selected().unwrap();
            let selected_entry = &test_app.app.dir_items.items[selected_idx];
            if selected_entry.file_name() == dest_dirpath.file_name().unwrap() {
                test_app.app.handle_keypress(KeyCode::Right.into());
                break;
            }
            test_app.app.handle_keypress(KeyCode::Down.into());
        }
        test_app.app.handle_keypress(KeyCode::Char('p').into());

        assert!(fs::File::open(dest_filepath).is_ok());
        test_dir.close().unwrap();
        dest_dir.close().unwrap();
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
