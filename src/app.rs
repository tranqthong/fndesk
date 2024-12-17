use std::{fs::DirEntry, process::Command};

use ratatui::widgets::ListState;

use crate::utils::{get_dir_items, get_parent_dir};

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
    pub current_dir: String,
    pub parent_dir: String,
    pub dir_items: DirListState,
    pub show_hidden: bool,
}

impl App {
    pub fn new(init_dir: String) -> Self {
        App {
            app_state: AppState::Running,
            current_dir: init_dir.clone(),
            parent_dir: get_parent_dir(&init_dir.clone()),
            dir_items: DirListState::new(get_dir_items(&init_dir.clone(), &false)),
            show_hidden: false,
        }
    }

    pub fn quit_app(&mut self) {
        self.app_state = AppState::Exit;
    }

    pub fn toggle_hidden(&mut self) {
        if self.show_hidden {
            self.show_hidden = false;
        } else {
            self.show_hidden = true;
        }
        self.dir_items
            .set_items(get_dir_items(&self.current_dir, &self.show_hidden));
    }

    pub fn move_cursor_left(&mut self) {
        // TODO navigate through the table
    }

    pub fn move_cursor_right(&mut self) {
        // TODO navigate through the table
    }

    pub fn move_cursor_up(&mut self) {
        self.dir_items.state.select_previous();
    }

    pub fn move_cursor_down(&mut self) {
        self.dir_items.state.select_next();
    }

    pub fn switch_panes(&mut self) {
        // TODO switch between left and right
    }

    pub fn delete_selected(&mut self) {
        // TODO move file to user's trash
    }

    pub fn nav_up_dir(&mut self) {
        self.current_dir = self.parent_dir.clone();
        self.parent_dir = get_parent_dir(&self.current_dir);
        self.dir_items
            .set_items(get_dir_items(&self.current_dir, &self.show_hidden));
    }

    pub fn open_selected(&mut self) {
        let selected_index = self.dir_items.state.selected().unwrap();
        let selected_entry = &self.dir_items.items[selected_index];
        if selected_entry.metadata().unwrap().is_dir() {
            self.current_dir = selected_entry.path().to_str().unwrap().to_owned();
            self.parent_dir = selected_entry
                .path()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            self.dir_items
                .set_items(get_dir_items(&self.current_dir, &self.show_hidden));
        } else {
            let selected_entry_path = selected_entry.path().to_str().unwrap().to_owned();
            let _open_file = Command::new("xdg-open")
                .arg(selected_entry_path)
                // .arg("&")
                .output()
                .expect("Failed to open file {selected_entry_path}");
            // TODO need to handle opening files on Windows/Mac in the future
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    struct TestContext {
        app: App,
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown...");
        }
    }

    fn setup() -> TestContext {
        println!("Test setup...");
        let init_dir = env::current_dir()
            .unwrap()
            .as_os_str()
            .to_os_string()
            .into_string();
        TestContext {
            app: App::new(init_dir.unwrap()),
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
    fn test_open_another_dir() {}

    #[test]
    fn test_open_file() {}

    #[test]
    fn test_moving_cursor_up() {}

    #[test]
    fn test_moving_cursor_down() {}

    #[test]
    fn test_nav_parent_dir() {}
}
