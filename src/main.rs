mod ansi;
mod classic;
mod cli;
mod config;
mod content;
mod hollywood;
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
            let initial = config::Config::load().unwrap_or_else(|| config::Config::from_args(args));
            if let Some(cfg) = tui::run(initial) {
                return cfg;
            }
            std::process::exit(0);
        }
    }
    setup::run(args)
}

fn no_args() -> bool {
    std::env::args_os().len() == 1
}

fn main() {
    let args = cli::parse();

    // In a TTY environment:
    // - Bare `imposter` or explicit `-i` starts the interactive TUI (using saved config as baseline).
    // - `-c` / `--continue` bypasses TUI and directly runs the saved settings.
    // - Any other explicit flags run the session directly.
    let is_tty = Ui::stdout_is_tty();
    let setup_now = is_tty && (args.interactive || (no_args() && !args.r#continue));

    let cfg = if setup_now {
        let cfg = interactive(&args);
        let _ = cfg.save();
        cfg
    } else if args.r#continue || no_args() {
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
        style::Style::Hollywood => hollywood::run(&mut ui, &cfg, &args, &mut rng, deadline),
        _ => transcript::run(&mut ui, &cfg, &args, &mut rng, deadline),
    }

    ui.flush();
}
