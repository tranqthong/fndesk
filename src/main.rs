use std::{env, error::Error};

mod app;
mod cli;
#[cfg(feature = "crossterm")]
mod crossterm;
mod entries;
mod status_bar;
mod ui;
mod ui_styles;
mod dir;

fn main() -> Result<(), Box<dyn Error>> {
    let env_log = env_logger::Env::default();
    env_logger::init_from_env(env_log);

    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let init_dir = crate::cli::parse_args(args);

    #[cfg(feature = "crossterm")]
    crate::crossterm::run(init_dir)?;
    Ok(())
}
