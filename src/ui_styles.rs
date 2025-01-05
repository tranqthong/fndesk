use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType},
};

pub const SELECTED_DIR_STYLE: Style = Style::new().fg(Color::Magenta);
pub const SELECTED_ENTRY_STYLE: Style = Style::new().bg(Color::Cyan);
pub const POPUP_BLOCK_STYLE: Style = Style::new().bg(Color::DarkGray);
pub const OVERWRITE_STYLE: Style = Style::new().bg(Color::Red);

pub const ROUNDED_BLOCK: Block = Block::new().border_type(BorderType::Rounded);
