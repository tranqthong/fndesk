use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::Backend;
use ratatui::Terminal;

use crate::app::{App, AppState};
use crate::{ui, utils};

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let init_dir = utils::get_init_dirpath();
    // should always be able grab the current directory from which the program is started
    let app = App::new(init_dir.into_string().unwrap());
    let app_result = run_app(&mut terminal, app, tick_rate);

    ratatui::restore();

    if let Err(err) = app_result {
        println!("{err:?}"); // TODO replace with logging later
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    while app.app_state == AppState::Running {
        terminal.draw(|f| {
            ui::draw(f, &mut app);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.quit_app(),
                        KeyCode::Char('h') => app.toggle_hidden(),
                        KeyCode::Enter | KeyCode::Char(' ') => app.open_selected(),
                        KeyCode::Up => app.move_cursor_up(),
                        KeyCode::Down => app.move_cursor_down(),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Tab | KeyCode::BackTab => app.switch_panes(),
                        KeyCode::Esc | KeyCode::Backspace => app.nav_up_dir(),
                        KeyCode::Delete => app.delete_selected(),
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    Ok(())
}
