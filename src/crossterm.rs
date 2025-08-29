use log::error;
use std::error::Error;
use std::io;
use std::path::Path;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::prelude::Backend;
use ratatui::Terminal;

use crate::app::{App, AppState};
use crate::ui;

pub fn run<T: AsRef<Path>>(init_dir: T) -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let app = App::new(init_dir);
    let app_result = run_app(&mut terminal, app);

    ratatui::restore();

    if let Err(err) = app_result {
        error!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // TODO find a better way to detect file system changes if possible
    app.refresh_dirlist();
    while app.app_state != AppState::Exit {
        terminal.draw(|f| {
            ui::draw(f, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_keypress(key);
            }
        }
    }
    Ok(())
}
