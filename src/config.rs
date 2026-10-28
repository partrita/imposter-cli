use crate::cli::Args;
use crate::style::Style;
use crate::types::{Complexity, DevelopmentType, JargonLevel, Speed, Theme};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub style: Style,
    pub theme: Theme,
    pub dev_type: DevelopmentType,
    pub jargon: JargonLevel,
    pub complexity: Complexity,
    pub duration: u64,
    pub alerts: bool,
    pub speed: Speed,
    pub model: Option<String>,
}

impl Default for Config {
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
        }
    }
}

impl Config {
    pub fn from_args(a: &Args) -> Self {
        Self {
            style: a.style,
            theme: a.theme,
            dev_type: a.dev_type,
            jargon: a.jargon,
            complexity: a.complexity,
            duration: a.duration,
            alerts: a.alerts,
            speed: a.speed,
            model: a.model.clone(),
        }
    }

    pub fn path() -> Option<PathBuf> {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("imposter").join("config.toml"))
    }

    /// Hand-rolled `key = value` parser. Saves ~10 crates versus serde + toml.
    pub fn load() -> Option<Self> {
        let text = fs::read_to_string(Self::path()?).ok()?;
        let mut cfg = Config::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                // `mode` is the pre-unification key; keep reading it so old
                // config files keep working.
                "mode" | "style" => {
                    cfg.style = Style::parse(value).unwrap_or(cfg.style);
                }
                "theme" => cfg.theme = Theme::parse(value).unwrap_or(cfg.theme),
                "dev_type" => cfg.dev_type = DevelopmentType::parse(value).unwrap_or(cfg.dev_type),
                "jargon" => cfg.jargon = JargonLevel::parse(value).unwrap_or(cfg.jargon),
                "complexity" => cfg.complexity = Complexity::parse(value).unwrap_or(cfg.complexity),
                "duration" => {
                    cfg.duration = value
                        .parse::<u64>()
                        .map(|n| n.min(crate::cli::MAX_MINUTES))
                        .unwrap_or(cfg.duration);
                }
                "alerts" => cfg.alerts = value == "true",
                "speed" => cfg.speed = Speed::parse(value).unwrap_or(cfg.speed),
                "model" => {
                    let m = value.trim();
                    if !m.is_empty() {
                        cfg.model = Some(m.to_string());
                    }
                }
                _ => {}
            }
        }
        Some(cfg)
    }

    pub fn save(&self) -> std::io::Result<PathBuf> {
        let path = Self::path().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "no config directory")
        })?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let model_line = match &self.model {
            Some(m) if !m.trim().is_empty() => format!("model = \"{}\"\n", m.trim()),
            _ => String::new(),
        };
        let body = format!(
            "# imposter settings\n\
             style = {}\n\
             theme = {}\n\
             dev_type = {}\n\
             jargon = {}\n\
             complexity = {}\n\
             duration = {}\n\
             alerts = {}\n\
             speed = {}\n\
             {}",
            self.style.as_str(),
            self.theme.as_str(),
            self.dev_type.as_str(),
            self.jargon.as_str(),
            self.complexity.as_str(),
            self.duration,
            self.alerts,
            self.speed.as_str(),
            model_line,
        );
        fs::write(&path, body)?;
        Ok(path)
    }
}
