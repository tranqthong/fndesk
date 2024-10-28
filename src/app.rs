use std::fs::Metadata;
use std::path::PathBuf;
use std::{fs, path};

use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::ListState;

use crate::utils::get_file_name;

pub struct App<'a> {
    pub title: &'a str,
    pub enhanced_graphics: bool,
    pub file_list: DirItemList,
    // pub filelist_state: FileListState<&'a str>,
    pub exit_program: bool,
}

pub struct DirItemList {
    pub items: Vec<DirItem>,
    pub state: ListState,
}

#[derive(Debug, Clone)]
pub struct DirItem {
    pub filename: String,
    pub file_path: PathBuf,
    pub is_dir: bool,
}

impl DirItem {
    fn new(filename: &str, file_path: PathBuf, is_dir: bool) -> Self {
        Self {
            filename: filename.to_string(),
            file_path: file_path,
            is_dir: is_dir,
        }
    }
}

impl From<&DirItem> for Text<'_> {
    fn from(dir_item: &DirItem) -> Self {
        let text = if dir_item.file_path.is_dir() {
            format!("{} (directory)", dir_item.filename)
        } else {
            dir_item.filename.clone()
        };
        Text::from(Span::styled(
            text,
            if dir_item.file_path.is_dir() {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        ))
    }
}

impl Into<Text<'_>> for DirItem {
    fn into(self) -> Text<'static> {
        let text = if self.is_dir {
            format!("{} (directory)", self.filename)
        } else {
            self.filename.clone()
        };
        Text::from(Span::styled(
            text,
            if self.is_dir {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        ))
    }
}

// impl DirItemList {
//     pub fn with_items(items: Vec<DirItem>) -> Self {
//         Self {
//             state: ListState::default(),
//             items,
//         }
//     }

//     pub fn next(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }

//     pub fn previous(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }
// }

impl FromIterator<DirItem> for DirItemList {
    fn from_iter<T: IntoIterator<Item = (DirItem)>>(iter: T) -> Self {
        let items = iter.into_iter().collect();

        let state = ListState::default();
        Self { items, state }
    }
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> Self {
        let mut cur_dir: Vec<DirItem> = vec![];

        if let Ok(current_dir) = fs::read_dir(".") {
            for item in current_dir {
                if let Ok(item) = item {
                    let current_file = DirItem::new(
                        &get_file_name(&item.file_name()),
                        item.path(),
                        item.metadata().unwrap().is_dir(),
                    );
                    cur_dir.push(current_file);
                }
            }
        } else {
            // TODO replace with logging
            eprintln!("Error reading directory");
        }

        App {
            title,
            file_list: DirItemList::from_iter(cur_dir),
            enhanced_graphics,
            exit_program: false,
        }
    }

    pub fn on_q(&mut self) {
        self.exit_program = true
    }

    pub fn on_left(&mut self) {
        // TODO switch between left and right
    }

    pub fn on_right(&mut self) {
        // TODO switch between left and right
    }

    pub fn on_up(&mut self) {
        self.file_list.state.select_previous();
    }

    pub fn on_down(&mut self) {
        self.file_list.state.select_next();
    }

    pub fn on_tab(&mut self) {}

    pub fn on_del(&mut self) {
        // TODO move file to user's trash
    }
    pub fn on_backspace(&mut self) {}

    pub fn open_selection(&mut self) {}
}
