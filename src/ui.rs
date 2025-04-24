use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::App,
    ui_styles::{CURRENT_DIR_STYLE, ROUNDED_BLOCK, SELECTED_ENTRY_STYLE, STATUS_BAR_STYLE},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    // This is a potential UI usage when I decide to add two column panes
    // let vertical_block = Layout::vertical([
    //     Constraint::Length(1),
    //     Constraint::Min(0),
    //     Constraint::Length(1),
    // ]);

    // let [title_area, main_area, status_area] = vertical_block.areas(frame.area());
    // let horizontal_block = Layout::horizontal([Constraint::Fill(1); 2]);
    // let [left_area, right_area] = horizontal_block.areas(main_area);

    // let title_str = app.current_dir.clone();
    // let item_list = handle_listing_dir_items(&app.current_dir_items);

    // frame.render_widget(Block::new().title(title_str), title_area);
    // frame.render_widget(Block::new().title("Status Placeholder"), status_area);
    // frame.render_widget(Block::bordered(), left_area);
    // frame.render_widget(Block::bordered(), right_area);

    let rect_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let title_block = Block::default().style(Style::default());
    let current_dir_path = app.current_dir.clone().into_os_string().into_string();
    let title = Paragraph::new(Text::styled(current_dir_path.unwrap(), CURRENT_DIR_STYLE))
        .block(title_block);

    let item_list: Vec<ListItem> = app
        .dir_items
        .items
        .iter()
        .map(|x| ListItem::new(x.file_name().into_string().unwrap()))
        .collect();

    let dir_items_list = List::new(item_list)
        .highlight_style(SELECTED_ENTRY_STYLE)
        .block(ROUNDED_BLOCK);

    let status_contents = Paragraph::new(app.status_text.clone());
    let status_bar = Paragraph::left_aligned(status_contents).style(STATUS_BAR_STYLE);

    frame.render_widget(title, rect_sections[0]);
    frame.render_stateful_widget(dir_items_list, rect_sections[1], &mut app.dir_items.state);
    frame.render_widget(status_bar, rect_sections[2]);
}
