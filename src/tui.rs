use std::borrow::Borrow;

use ratatui::prelude::Constraint;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem};
use ratatui::{layout::Layout, Frame};

use crate::app::{App, DirItemList};
use crate::utils;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn tui(frame: &mut Frame, app: &mut App) {
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left_area_block, right_area_block] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    let left_block = Block::bordered();

    let left_items: Vec<ListItem> = app
        .file_list
        .items
        .iter()
        .enumerate()
        .map(|(i, dir_item)| {
            // let color = utils::alternate_colors(i);
            ListItem::from(dir_item)
        })
        .collect();

    let left_dir_list = List::new(left_items)
        .block(left_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol("->")
        .highlight_spacing(HighlightSpacing::Always);

    // let left_area_files = List::new(app.file_list.items.iter().cloned())
    //     .highlight_style(SELECTED_STYLE)
    //     .highlight_symbol("->")
    //     .highlight_spacing(HighlightSpacing::Always)
    //     .block(Block::bordered())
    //     .style(Style::default());

    let right_area_files = List::new(["Placeholder"])
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol("->")
        .highlight_spacing(HighlightSpacing::Always)
        .block(Block::bordered())
        .style(Style::default());

    
    frame.render_widget(Block::bordered().title(app.title), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(left_dir_list, left_area_block);
    frame.render_widget(right_area_files, right_area_block);
}
