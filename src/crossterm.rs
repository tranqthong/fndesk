use std::{error::Error, io};

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};

use crate::{app::App, tui::tui};

pub fn run(enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    // manual way to set up terminal, will use default, leaving this in for now
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Drawer FM", enhanced_graphics);
    let app_result = run_app(&mut terminal, app);

    //manual way of restoring terminal, leaving this in for now
    // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    ratatui::restore();

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    while !app.exit_program {
        terminal.draw(|f| tui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => app.on_q(),
                KeyCode::Enter | KeyCode::Char(' ') => app.open_selection(),
                KeyCode::Up => app.on_up(),
                KeyCode::Down => app.on_down(),
                KeyCode::Left => app.on_left(),
                KeyCode::Right => app.on_right(),
                KeyCode::Tab | KeyCode::BackTab => app.on_tab(),
                KeyCode::Esc | KeyCode::Backspace => app.on_backspace(),
                KeyCode::Delete => app.on_del(),
                _ => {}
            }
        }
    }
    Ok(())
}
