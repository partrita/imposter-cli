mod ansi;
mod classic;
mod cli;
mod config;
mod content;
mod jargon;
mod rng;
mod setup;
mod style;
mod transcript;
#[cfg(feature = "tui")]
mod tui;
mod types;

use ansi::Ui;
use std::time::{Duration, Instant};

pub fn expired(deadline: Option<Instant>) -> bool {
    match deadline {
        Some(at) => Instant::now() >= at,
        None => false,
    }
}

fn interactive(args: &cli::Args) -> config::Config {
    #[cfg(feature = "tui")]
    {
        if Ui::stdout_is_tty() {
            if let Some(cfg) = tui::run(config::Config::from_args(args)) {
                return cfg;
            }
            std::process::exit(0);
        }
    }
    setup::run(args)
}

/// First launch with no arguments and no saved config: open the setup TUI
/// instead of dropping straight into a session. The result is saved so the
/// next run starts immediately.
fn no_args() -> bool {
    std::env::args_os().len() == 1
}

fn first_run() -> bool {
    no_args() && Ui::stdout_is_tty() && !config::Config::path().map(|p| p.exists()).unwrap_or(false)
}

fn main() {
    let args = cli::parse();

    let setup_now = args.interactive || first_run();
    let cfg = if setup_now {
        let cfg = interactive(&args);
        if !args.interactive {
            let _ = cfg.save();
        }
        cfg
    } else if no_args() {
        // A bare `imposter` uses the saved settings; any explicit flag wins.
        config::Config::load().unwrap_or_else(|| config::Config::from_args(&args))
    } else {
        config::Config::from_args(&args)
    };

    if args.save && !setup_now {
        match cfg.save() {
            Ok(path) => eprintln!("saved settings to {}", path.display()),
            Err(e) => eprintln!("could not save settings: {e}"),
        }
    }

    // No color or animation when piped: output stays grep-able and cheap.
    let color = std::env::var_os("NO_COLOR").is_none() && Ui::stdout_is_tty();
    if color {
        ansi::enable_ansi();
    }
    let animate = color && !args.fast;
    let mut ui = Ui::new(color, animate, cfg.theme);
    let mut rng = rng::Rng::new(args.seed);

    let deadline = if cfg.duration > 0 {
        Some(Instant::now() + Duration::from_secs(cfg.duration * 60))
    } else {
        None
    };

    match cfg.style {
        style::Style::Classic => classic::run(&mut ui, &cfg, &args, &mut rng, deadline),
        _ => transcript::run(&mut ui, &cfg, &args, &mut rng, deadline),
    }

    ui.flush();
}
