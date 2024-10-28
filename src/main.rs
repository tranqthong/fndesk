use argh::FromArgs;
use std::error::Error;

mod app;
#[cfg(feature = "crossterm")]
mod crossterm;
mod tui;
mod utils;

#[derive(Debug, FromArgs)]
#[argh(description = "foo")]
struct Cli {
    #[argh(option, description = "enable better graphics", default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    #[cfg(feature = "crossterm")]
    crate::crossterm::run(cli.enhanced_graphics)?;

    Ok(())
}
