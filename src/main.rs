use argh::FromArgs;
use std::{error::Error, time::Duration};

mod app;
#[cfg(feature = "crossterm")]
mod crossterm;
mod ui;
mod utils;

#[derive(Debug, FromArgs)]
///Optional Args
struct Cli {
    #[argh(option, description = "time in ms between two ticks", default = "250")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    #[cfg(feature = "crossterm")]
    crate::crossterm::run(tick_rate)?;

    // TODO add windows/mac support soon, maybe, eventually, whenever
    Ok(())
}
