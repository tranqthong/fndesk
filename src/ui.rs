use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
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
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(frame.area());

    let title_block = Block::default().style(Style::default());

    let title = Paragraph::new(Text::styled(
        &app.current_dir,
        Style::default().fg(Color::White).bg(Color::Magenta),
    ))
    .block(title_block);

    let item_list: Vec<ListItem> = app
        .dir_items
        .items
        .iter()
        .map(|x| ListItem::new(x.file_name().into_string().unwrap()))
        .collect();

    // let mut dir_item_state = DirListState::new(item_list);
    let dir_items_list = List::new(item_list).highlight_style(Style::default().bg(Color::Cyan));

    frame.render_widget(title, rect_sections[0]);
    frame.render_stateful_widget(dir_items_list, rect_sections[1], &mut app.dir_items.state);
}
