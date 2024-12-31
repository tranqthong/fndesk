use log::error;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::prelude::Backend;
use ratatui::Terminal;

use crate::app::{App, AppState};
use crate::{ui, utils};

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let init_dir = utils::get_init_dirpath();
    // should always be able grab the current directory from which the program is started
    let app = App::new(init_dir);
    let app_result = run_app(&mut terminal, app, tick_rate);

    ratatui::restore();

    if let Err(err) = app_result {
        error!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    // need to find a better way to detect file system changes besides
    // constant refreshing
    app.refresh_dirlist();
    while app.app_state != AppState::Exit {
        terminal.draw(|f| {
            ui::draw(f, &mut app);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_keypress(key);
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    Ok(())
}
