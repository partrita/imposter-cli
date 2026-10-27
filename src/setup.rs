use crate::cli::Args;
use crate::config::Config;
use crate::style::Style;
use crate::types::{Complexity, DevelopmentType, JargonLevel, Theme};
use std::io::{self, Write};

/// A tiny stdin questionnaire. No raw mode, no alternate screen, no deps.
pub fn run(args: &Args) -> Config {
    let mut cfg = Config::load().unwrap_or_else(|| Config::from_args(args));

    println!("imposter \u{00b7} setup");
    println!("press enter to keep the value in brackets\n");

    let answer = ask(&format!(
        "style [claude/codex/opencode/pi/antigravity/classic] ({})",
        cfg.style.as_str()
    ));
    if let Some(style) = Style::parse(&answer) {
        cfg.style = style;
    }

    let answer = ask(&format!("theme [dark/light] ({})", cfg.theme.as_str()));
    if let Some(theme) = Theme::parse(&answer) {
        cfg.theme = theme;
    }

    let answer = ask(&format!("dev type ({})", cfg.dev_type.as_str()));
    if let Some(dev) = DevelopmentType::parse(&answer) {
        cfg.dev_type = dev;
    }

    let answer = ask(&format!(
        "duration minutes, 0-999, 0 = forever ({})",
        cfg.duration
    ));
    if let Ok(minutes) = answer.parse::<u64>() {
        cfg.duration = minutes.min(crate::cli::MAX_MINUTES);
    }

    let answer = ask(&format!(
        "jargon [low/medium/high/extreme] ({})",
        cfg.jargon.as_str()
    ));
    if let Some(jargon) = JargonLevel::parse(&answer) {
        cfg.jargon = jargon;
    }

    let answer = ask(&format!(
        "complexity [low/medium/high/extreme] ({})",
        cfg.complexity.as_str()
    ));
    if let Some(complexity) = Complexity::parse(&answer) {
        cfg.complexity = complexity;
    }

    let answer = ask(&format!(
        "fake alerts? [y/N] ({})",
        if cfg.alerts { "y" } else { "n" }
    ));
    if !answer.is_empty() {
        cfg.alerts = answer.starts_with('y');
    }

    let answer = ask("save these settings? [Y/n]");
    if answer.is_empty() || answer.starts_with('y') {
        match cfg.save() {
            Ok(path) => println!("saved to {}", path.display()),
            Err(e) => eprintln!("could not save settings: {e}"),
        }
    }

    println!();
    cfg
}

fn ask(label: &str) -> String {
    print!("{label}: ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return String::new();
    }
    line.trim().to_string()
}
