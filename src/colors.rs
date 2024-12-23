use ratatui::style::{Color, Style};

pub const SELECTED_DIR_STYLE: ratatui::prelude::Style =
    Style::new().fg(Color::White).bg(Color::Magenta);
pub const SELECTED_ENTRY_STYLE: ratatui::prelude::Style = Style::new().bg(Color::Cyan);
pub const POPUP_BLOCK_STYLE: ratatui::prelude::Style = Style::new().bg(Color::DarkGray);
pub const OVERWRITE_STYLE: ratatui::prelude::Style = Style::new().bg(Color::Red);
