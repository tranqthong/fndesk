use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType},
};

pub const CURRENT_DIR_STYLE: Style = Style::new().fg(Color::Magenta);
pub const SELECTED_ENTRY_STYLE: Style = Style::new().bg(Color::Cyan);
pub const CLIPBOARD_SELECTED_STYLE: Style = Style::new().bg(Color::LightRed);
pub const STATUS_BAR_STYLE: Style = Style::new().bg(Color::DarkGray).fg(Color::White);

pub const ROUNDED_BLOCK: Block = Block::bordered().border_type(BorderType::Rounded);
