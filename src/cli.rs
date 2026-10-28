use crate::style::Style;
use crate::types::{Complexity, DevelopmentType, JargonLevel, Speed, Theme};

/// Longest session the CLI accepts, in minutes (about 16.6 hours).
pub const MAX_MINUTES: u64 = 999;

pub struct Args {
    pub style: Style,
    pub theme: Theme,
    pub dev_type: DevelopmentType,
    pub jargon: JargonLevel,
    pub complexity: Complexity,
    pub duration: u64,
    pub alerts: bool,
    pub speed: Speed,
    pub model: Option<String>,
    pub interactive: bool,
    pub r#continue: bool,
    pub save: bool,
    pub fast: bool,
    pub seed: Option<u64>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            style: Style::Claude,
            theme: Theme::Dark,
            dev_type: DevelopmentType::Backend,
            jargon: JargonLevel::Medium,
            complexity: Complexity::Medium,
            duration: 0,
            alerts: false,
            speed: Speed::Normal,
            model: None,
            interactive: false,
            r#continue: false,
            save: false,
            fast: false,
            seed: None,
        }
    }
}

pub fn parse() -> Args {
    let mut args = Args::default();
    let mut it = std::env::args().skip(1);

    while let Some(raw) = it.next() {
        let (key, inline) = match raw.split_once('=') {
            Some((k, v)) => (k.to_string(), Some(v.to_string())),
            None => (raw, None),
        };

        match key.as_str() {
            "-h" | "--help" => {
                print!("{HELP}");
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("imposter {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--claude" => args.style = Style::Claude,
            "--classic" => args.style = Style::Classic,
            "--hollywood" | "--hacker" => args.style = Style::Hollywood,
            "-s" | "--style" => {
                let v = take(&mut it, inline, &key);
                args.style = Style::parse(&v).unwrap_or_else(|| {
                    fail(&format!(
                        "unknown style `{v}` (claude, codex, opencode, pi, antigravity, hollywood, classic)"
                    ))
                });
            }
            "--dark" => args.theme = Theme::Dark,
            "--light" => args.theme = Theme::Light,
            "-t" | "--theme" => {
                let v = take(&mut it, inline, &key);
                args.theme = Theme::parse(&v)
                    .unwrap_or_else(|| fail(&format!("unknown theme `{v}` (dark, light)")));
            }
            "-d" | "--dev-type" | "--dev" => {
                let v = take(&mut it, inline, &key);
                args.dev_type = DevelopmentType::parse(&v)
                    .unwrap_or_else(|| fail(&format!("unknown dev type `{v}`")));
            }
            "-j" | "--jargon" => {
                let v = take(&mut it, inline, &key);
                args.jargon = JargonLevel::parse(&v)
                    .unwrap_or_else(|| fail(&format!("unknown jargon `{v}`")));
            }
            "--complexity" => {
                let v = take(&mut it, inline, &key);
                args.complexity = Complexity::parse(&v)
                    .unwrap_or_else(|| fail(&format!("unknown complexity `{v}`")));
            }
            "-c" | "--continue" => args.r#continue = true,
            "--speed" => {
                let v = take(&mut it, inline, &key);
                args.speed = Speed::parse(&v).unwrap_or_else(|| {
                    fail(&format!("unknown speed `{v}` (slow, normal, fast, turbo)"))
                });
            }
            "-m" | "--model" => {
                let v = take(&mut it, inline, &key);
                if !v.trim().is_empty() {
                    args.model = Some(v);
                }
            }
            "-T" | "--duration" => {
                let v = take(&mut it, inline, &key);
                args.duration = parse_minutes(&v);
            }
            "-a" | "--alerts" => args.alerts = true,
            "-i" | "--interactive" | "setup" => args.interactive = true,
            "--save" => args.save = true,
            "--fast" => args.fast = true,
            "--seed" => {
                let v = take(&mut it, inline, &key);
                args.seed = Some(
                    v.parse::<u64>()
                        .unwrap_or_else(|_| fail(&format!("seed must be an integer, got `{v}`"))),
                );
            }
            "--" => break,
            other => fail(&format!("unknown argument `{other}`")),
        }
    }

    args
}

fn take(it: &mut impl Iterator<Item = String>, inline: Option<String>, key: &str) -> String {
    inline.unwrap_or_else(|| {
        it.next()
            .unwrap_or_else(|| fail(&format!("missing value for `{key}`")))
    })
}

fn parse_minutes(v: &str) -> u64 {
    match v.parse::<u64>() {
        Ok(n) if n <= MAX_MINUTES => n,
        Ok(_) => fail(&format!(
            "duration must be 0-{MAX_MINUTES} minutes, got `{v}`"
        )),
        Err(_) => fail(&format!("duration must be minutes, got `{v}`")),
    }
}

pub fn fail(msg: &str) -> ! {
    eprintln!("imposter: {msg}");
    eprintln!("\ntry `imposter --help`");
    std::process::exit(2);
}

pub const HELP: &str = "\
imposter - look busy without doing anything

USAGE:
    imposter [OPTIONS]
    imposter --interactive

OPTIONS:
    -s, --style <NAME>            AI CLI: claude codex opencode pi antigravity hollywood, or classic
                                  [default: claude]
        --claude                  Shortcut for --style claude
        --classic                 Shortcut for --style classic
        --hollywood               Shortcut for --style hollywood
    -t, --theme <MODE>            Color theme: dark or light [default: dark]
        --dark                    Shortcut for --theme dark
        --light                   Shortcut for --theme light
    -d, --dev-type <TYPE>         backend frontend fullstack data-science devops
                                  blockchain machine-learning systems game-dev security
    -T, --duration <MINUTES>      Stop after N minutes, 0-999 (0 = until Ctrl-C)
                                  [default: 0]
    -a, --alerts                  Sprinkle in fake errors and fixes
    -j, --jargon <LEVEL>          low medium high extreme
        --complexity <LEVEL>      low medium high extreme
    -c, --continue                Continue directly with previous settings without TUI
        --speed <SPEED>           Playback speed: slow normal fast turbo [default: normal]
    -m, --model <NAME>            Custom model name (e.g. claude-3-7-sonnet, gpt-4o)
        --fast                    No animation (useful for demos, GIFs, CI)
        --seed <N>                Deterministic output
        --save                    Save the current settings
    -i, --interactive             Open the setup TUI explicitly
    -h, --help                    Show this help
    -V, --version                 Show the version

EXAMPLES:
    imposter
    imposter --style antigravity --light
    imposter --dev-type systems --duration 45   # 45 minutes
    imposter --classic --complexity extreme --alerts
    imposter --fast --duration 1 --seed 42
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_claude_and_infinite() {
        let a = Args::default();
        assert_eq!(a.style, Style::Claude);
        assert_eq!(a.theme, Theme::Dark);
        assert_eq!(a.duration, 0);
    }

    #[test]
    fn classic_is_a_style() {
        assert_eq!(Style::parse("classic"), Some(Style::Classic));
        assert_eq!(Style::parse("classic").map(Style::as_str), Some("classic"));
    }

    #[test]
    fn theme_parses_dark_and_light() {
        assert_eq!(Theme::parse("dark"), Some(Theme::Dark));
        assert_eq!(Theme::parse("light"), Some(Theme::Light));
    }

    #[test]
    fn speed_parses_all_variants() {
        assert_eq!(Speed::parse("slow"), Some(Speed::Slow));
        assert_eq!(Speed::parse("normal"), Some(Speed::Normal));
        assert_eq!(Speed::parse("fast"), Some(Speed::Fast));
        assert_eq!(Speed::parse("turbo"), Some(Speed::Turbo));
    }
}
