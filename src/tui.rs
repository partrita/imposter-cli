use crate::config::Config;
use crate::content;
use crate::jargon;
use crate::style::Style;
use crate::types::{Complexity, Theme};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    MouseButton, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style as TuiStyle};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io::stdout;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
    Style,
    Model,
    Speed,
    Theme,
    Duration,
    Start,
    Save,
}

const FIELDS: [Field; 7] = [
    Field::Style,
    Field::Model,
    Field::Speed,
    Field::Theme,
    Field::Duration,
    Field::Start,
    Field::Save,
];

impl Field {
    fn label(self) -> &'static str {
        match self {
            Field::Style => "Style",
            Field::Model => "Model",
            Field::Speed => "Speed",
            Field::Theme => "Theme",
            Field::Duration => "Duration",
            Field::Start => "",
            Field::Save => "",
        }
    }

    #[allow(dead_code)]
    fn editable(self) -> bool {
        matches!(self, Field::Duration | Field::Model)
    }
}

struct Palette {
    bg: Color,
    fg: Color,
    muted: Color,
    dim: Color,
    border: Color,
    green: Color,
    red: Color,
    accent: Color,
    badge_fg: Color,
}

struct App {
    cfg: Config,
    field: usize,
    editing: bool,
    buf: String,
    status: Option<String>,
    start: bool,
    quit: bool,
}

impl App {
    fn new(cfg: Config) -> Self {
        Self {
            cfg,
            field: 0,
            editing: false,
            buf: String::new(),
            status: None,
            start: false,
            quit: false,
        }
    }

    fn accent(&self) -> Color {
        if self.cfg.style == Style::Classic {
            return match self.cfg.theme {
                Theme::Dark => Color::Rgb(80, 200, 120),
                Theme::Light => Color::Rgb(17, 99, 41),
            };
        }
        let (r, g, b) = self.cfg.style.skin().rgb_for(self.cfg.theme);
        Color::Rgb(r, g, b)
    }

    fn palette(&self) -> Palette {
        let accent = self.accent();
        match self.cfg.theme {
            Theme::Dark => Palette {
                bg: Color::Reset,
                fg: Color::White,
                muted: Color::Gray,
                dim: Color::DarkGray,
                border: Color::DarkGray,
                green: Color::Green,
                red: Color::Red,
                accent,
                badge_fg: Color::Black,
            },
            Theme::Light => Palette {
                bg: Color::Rgb(248, 250, 252),
                fg: Color::Rgb(15, 23, 42),
                muted: Color::Rgb(51, 65, 85),
                dim: Color::Rgb(100, 116, 139),
                border: Color::Rgb(148, 163, 184),
                green: Color::Rgb(17, 99, 41),
                red: Color::Rgb(164, 14, 38),
                accent,
                badge_fg: Color::Rgb(248, 250, 252),
            },
        }
    }

    fn key(&mut self, code: KeyCode, mods: KeyModifiers) {
        if mods.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        if mods.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('s') {
            self.save();
            return;
        }
        if mods.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('r') {
            self.start = true;
            return;
        }

        if self.editing {
            match code {
                KeyCode::Enter => self.commit_edit(),
                KeyCode::Esc => {
                    self.editing = false;
                    self.buf.clear();
                }
                KeyCode::Backspace => {
                    self.buf.pop();
                }
                KeyCode::Char(c) => {
                    if FIELDS[self.field] == Field::Duration {
                        if c.is_ascii_digit() && self.buf.len() < 3 {
                            self.buf.push(c);
                        }
                    } else if FIELDS[self.field] == Field::Model && self.buf.len() < 32 {
                        self.buf.push(c);
                    }
                }
                _ => {}
            }
            return;
        }

        self.status = None;

        match code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit = true,
            KeyCode::Up | KeyCode::BackTab | KeyCode::Char('k') => {
                self.field = (self.field + FIELDS.len() - 1) % FIELDS.len();
            }
            KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => {
                self.field = (self.field + 1) % FIELDS.len();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if FIELDS[self.field] == Field::Save {
                    self.field = FIELDS.iter().position(|f| *f == Field::Start).unwrap_or(0);
                } else {
                    self.cycle(-1);
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if FIELDS[self.field] == Field::Start {
                    self.field = FIELDS.iter().position(|f| *f == Field::Save).unwrap_or(0);
                } else {
                    self.cycle(1);
                }
            }
            KeyCode::Char(' ') => match FIELDS[self.field] {
                Field::Start => self.start = true,
                Field::Save => self.save(),
                _ => self.cycle(1),
            },
            KeyCode::Char('t') => {
                self.cfg.theme = cycle(&Theme::ALL, self.cfg.theme, 1);
            }
            KeyCode::Char('s') => self.save(),
            KeyCode::Char(c @ '1'..='7') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < Style::ALL.len() {
                    self.cfg.style = Style::ALL[idx];
                }
            }
            KeyCode::Enter => match FIELDS[self.field] {
                Field::Start => self.start = true,
                Field::Save => self.save(),
                Field::Duration => {
                    self.buf = self.cfg.duration.to_string();
                    self.editing = true;
                }
                Field::Model => {
                    self.buf = self.cfg.model.clone().unwrap_or_default();
                    self.editing = true;
                }
                _ => self.cycle(1),
            },
            _ => {}
        }
    }

    fn mouse_click(&mut self, col: u16, row: u16) {
        // Header style shortcuts row (row == 1 in alternate screen)
        if row == 1 && col > 15 {
            let offset = (col.saturating_sub(15) / 12) as usize;
            if offset < Style::ALL.len() {
                self.cfg.style = Style::ALL[offset];
                return;
            }
        }

        // Form region: row 4..=8 for the 5 config fields
        if (4..=8).contains(&row) {
            let clicked_field = (row - 4) as usize;
            if clicked_field < 5 {
                if self.field == clicked_field {
                    if FIELDS[self.field].editable() {
                        if FIELDS[self.field] == Field::Duration {
                            self.buf = self.cfg.duration.to_string();
                        } else {
                            self.buf = self.cfg.model.clone().unwrap_or_default();
                        }
                        self.editing = true;
                    } else {
                        self.cycle(1);
                    }
                } else {
                    self.field = clicked_field;
                }
                return;
            }
        }

        // Action buttons row (row 10)
        if row == 10 {
            if (3..=21).contains(&col) {
                self.start = true;
            } else if (23..=42).contains(&col) {
                self.save();
            }
        }
    }

    fn cycle(&mut self, dir: i32) {
        match FIELDS[self.field] {
            Field::Style => self.cfg.style = cycle(&Style::ALL, self.cfg.style, dir),
            Field::Model => {
                if self.cfg.model.is_some() {
                    self.cfg.model = None;
                }
            }
            Field::Speed => self.cfg.speed = cycle(&crate::types::Speed::ALL, self.cfg.speed, dir),
            Field::Theme => self.cfg.theme = cycle(&Theme::ALL, self.cfg.theme, dir),
            Field::Duration => {
                let step = 5i64;
                let next = self.cfg.duration as i64 + if dir > 0 { step } else { -step };
                self.cfg.duration = next.clamp(0, crate::cli::MAX_MINUTES as i64) as u64;
            }
            _ => {}
        }
    }

    fn commit_edit(&mut self) {
        if FIELDS[self.field] == Field::Duration {
            if let Ok(v) = self.buf.trim().parse::<u64>() {
                self.cfg.duration = v.min(crate::cli::MAX_MINUTES);
            }
        } else if FIELDS[self.field] == Field::Model {
            let trimmed = self.buf.trim();
            if trimmed.is_empty() {
                self.cfg.model = None;
            } else {
                self.cfg.model = Some(trimmed.to_string());
            }
        }
        self.editing = false;
        self.buf.clear();
    }

    fn save(&mut self) {
        self.status = Some(match self.cfg.save() {
            Ok(path) => format!("\u{2713} Saved defaults to {}", path.display()),
            Err(e) => format!("! Save failed: {e}"),
        });
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.area();
        let pal = self.palette();
        if self.cfg.theme == Theme::Light {
            let bg_block = Block::default().style(TuiStyle::default().bg(pal.bg).fg(pal.fg));
            f.render_widget(bg_block, area);
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(13),
                Constraint::Length(2),
            ])
            .split(area);
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(rows[1]);

        self.draw_header(f, rows[0], &pal);
        self.draw_form(f, cols[0], &pal);
        self.draw_preview(f, cols[1], &pal);
        self.draw_footer(f, rows[2], &pal);
    }

    fn draw_header(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let mut spans = vec![
            Span::styled(
                " imposter ",
                TuiStyle::default()
                    .fg(pal.badge_fg)
                    .bg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  style ", TuiStyle::default().fg(pal.dim)),
        ];

        for (i, st) in Style::ALL.iter().enumerate() {
            let active = *st == self.cfg.style;
            let num = format!("{}", i + 1);
            if active {
                spans.push(Span::styled(
                    format!(" [{num}:{}] ", st.as_str()),
                    TuiStyle::default()
                        .fg(pal.badge_fg)
                        .bg(pal.accent)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    format!(" {num}:{} ", st.as_str()),
                    TuiStyle::default().fg(pal.muted),
                ));
            }
            if i + 1 < Style::ALL.len() {
                spans.push(Span::styled("\u{00b7}", TuiStyle::default().fg(pal.dim)));
            }
        }

        let theme_badge = match self.cfg.theme {
            Theme::Dark => "  [t: \u{25d0} dark] ",
            Theme::Light => "  [t: \u{263c} light] ",
        };
        spans.push(Span::styled(
            theme_badge,
            TuiStyle::default()
                .fg(pal.accent)
                .add_modifier(Modifier::BOLD),
        ));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(TuiStyle::default().fg(pal.border))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg));
        f.render_widget(Paragraph::new(Line::from(spans)).block(block), area);
    }

    fn draw_form(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let mut lines: Vec<Line> = Vec::with_capacity(12);

        for (i, field) in FIELDS.iter().enumerate() {
            if matches!(field, Field::Start | Field::Save) {
                continue;
            }
            let selected = i == self.field;
            let pointer = if selected { " \u{276f} " } else { "   " };
            let label_style = if selected {
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                TuiStyle::default().fg(pal.muted)
            };

            let mut spans = vec![
                Span::styled(pointer, TuiStyle::default().fg(pal.accent)),
                Span::styled(format!("{:<10}", field.label()), label_style),
            ];
            spans.extend(self.render_field_value(*field, selected, pal));
            lines.push(Line::from(spans));
        }

        lines.push(Line::from(""));

        let start_sel = FIELDS[self.field] == Field::Start;
        let save_sel = FIELDS[self.field] == Field::Save;
        let start_style = if start_sel {
            TuiStyle::default()
                .fg(pal.badge_fg)
                .bg(pal.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default()
                .fg(pal.accent)
                .add_modifier(Modifier::BOLD)
        };
        let save_style = if save_sel {
            TuiStyle::default()
                .fg(pal.badge_fg)
                .bg(pal.fg)
                .add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default().fg(pal.muted)
        };

        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(" \u{25b6} Start Session ", start_style),
            Span::raw("   "),
            Span::styled(" \u{2713} Save Defaults ", save_style),
        ]));

        lines.push(Line::from(""));
        let dim = TuiStyle::default().fg(pal.dim);
        let (_, desc_body) = self.field_description(FIELDS[self.field]);
        lines.push(Line::from(vec![
            Span::styled("   \u{00bb} ", TuiStyle::default().fg(pal.accent)),
            Span::styled(desc_body, dim),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(TuiStyle::default().fg(pal.accent))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg))
            .title(Span::styled(
                " Configuration ",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            ));
        f.render_widget(
            Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn render_field_value(
        &self,
        field: Field,
        selected: bool,
        pal: &Palette,
    ) -> Vec<Span<'static>> {
        let arrow_style = if selected {
            TuiStyle::default()
                .fg(pal.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default().fg(pal.dim)
        };
        let val_style = if selected {
            TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default().fg(pal.muted)
        };
        let dim = TuiStyle::default().fg(pal.dim);

        match field {
            Field::Style => {
                let idx = Style::ALL
                    .iter()
                    .position(|s| *s == self.cfg.style)
                    .unwrap_or(0)
                    + 1;
                vec![
                    Span::styled("\u{2039} ", arrow_style),
                    Span::styled(format!("{:<12}", self.cfg.style.as_str()), val_style),
                    Span::styled("\u{203a} ", arrow_style),
                    Span::styled(format!("({idx}/{})", Style::ALL.len()), dim),
                ]
            }
            Field::Model => {
                if self.editing && selected {
                    return vec![
                        Span::styled(
                            format!("{}\u{2588}", self.buf),
                            TuiStyle::default()
                                .fg(pal.accent)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("  (Enter save, Esc cancel)", dim),
                    ];
                }
                let display = match &self.cfg.model {
                    Some(m) if !m.is_empty() => m.clone(),
                    _ => format!("auto ({})", self.cfg.style.skin().model),
                };
                vec![
                    Span::styled(format!("{display:<24}"), val_style),
                    Span::styled("(Enter to edit)", dim),
                ]
            }
            Field::Speed => {
                let idx = crate::types::Speed::ALL
                    .iter()
                    .position(|s| *s == self.cfg.speed)
                    .unwrap_or(0)
                    + 1;
                vec![
                    Span::styled("\u{2039} ", arrow_style),
                    Span::styled(format!("{:<12}", self.cfg.speed.as_str()), val_style),
                    Span::styled("\u{203a} ", arrow_style),
                    Span::styled(format!("({idx}/{})", crate::types::Speed::ALL.len()), dim),
                ]
            }
            Field::Theme => {
                let dark_active = self.cfg.theme == Theme::Dark;
                let active_style = TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD);
                let inactive_style = TuiStyle::default().fg(pal.dim);
                if dark_active {
                    vec![
                        Span::styled("[\u{25d0} dark ]", active_style),
                        Span::raw(" "),
                        Span::styled(" \u{263c} light ", inactive_style),
                    ]
                } else {
                    vec![
                        Span::styled(" \u{25d0} dark  ", inactive_style),
                        Span::raw(" "),
                        Span::styled("[\u{263c} light]", active_style),
                    ]
                }
            }
            Field::Duration => {
                if self.editing && selected {
                    return vec![
                        Span::styled(
                            format!("{} min\u{2588}", self.buf),
                            TuiStyle::default()
                                .fg(pal.accent)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("  (0 = forever)", dim),
                    ];
                }
                let text = if self.cfg.duration == 0 {
                    "forever (\u{221e})".to_string()
                } else {
                    format!("{} min", self.cfg.duration)
                };
                vec![
                    Span::styled("\u{2039} ", arrow_style),
                    Span::styled(format!("{text:<12}"), val_style),
                    Span::styled("\u{203a} ", arrow_style),
                    Span::styled("(\u{2190}/\u{2192} \u{00b1}5m, Enter type)", dim),
                ]
            }
            Field::Start | Field::Save => Vec::new(),
        }
    }

    fn field_description(&self, field: Field) -> (&'static str, String) {
        match field {
            Field::Style => {
                let desc = match self.cfg.style {
                    Style::Claude => "Anthropic Claude Code v2.1: Clawd banner, \u{276f} prompt, \u{25cf} tools & \u{23bf} tree output.",
                    Style::Classic => "Legacy stakeholder system log: progress bars, symbol linker & p99 metrics.",
                    Style::Codex => "OpenAI Codex CLI (codex-rs): >_ header, \u{203a} prompt, \u{2022} Explored/Edited/Ran blocks.",
                    Style::Opencode => "OpenCode TUI: SplitBorder \u{2503} backgroundPanel header/prompt/tools, Thinking:, \u{2192} Read & \u{25a3} Build.",
                    Style::Pi => "earendil-works/pi: \u{2580}\u{2588}\u{2580} header, #343541 userMessageBg & #283228 toolSuccessBg boxes, footer.ts.",
                    Style::Antigravity => "Google Antigravity CLI (agy): dotted horizon banner, \u{25cb} trajectory tools & /statusline HUD.",
                    Style::Hollywood => "Hollywood Hacker: matrix hex dump torrent, mainframe override gauges & ACCESS GRANTED banners.",
                };
                ("Active CLI Persona", desc.to_string())
            }
            Field::Model => (
                "AI Model Designation",
                "Custom model name (e.g. claude-3-7-sonnet, gpt-4o, gemini-2.5-pro). Leave empty for style default."
                    .to_string(),
            ),
            Field::Speed => (
                "Transcript Playback Speed",
                "Slow (0.5x), Normal (1.0x), Fast (2.0x), Turbo (5.0x) with natural jitter variations."
                    .to_string(),
            ),
            Field::Theme => (
                "Color Theme (Dark / Light)",
                "Adapts accent, text, border, and diff contrast for dark or light terminals (shortcut: t)."
                    .to_string(),
            ),
            Field::Duration => (
                "Auto-Stop Timer",
                "0 runs until Ctrl-C. Set 1-999 minutes to exit cleanly on a deadline.".to_string(),
            ),
            Field::Start => (
                "Launch Session",
                "Press Enter, Space, or Ctrl+R to exit setup and start the fake session."
                    .to_string(),
            ),
            Field::Save => (
                "Persist Defaults",
                "Writes current settings to ~/.config/imposter/config.toml.".to_string(),
            ),
        }
    }

    fn draw_preview(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let fg = TuiStyle::default().fg(pal.fg);
        let dim = TuiStyle::default().fg(pal.dim);
        let green = TuiStyle::default().fg(pal.green);
        let red = TuiStyle::default().fg(pal.red);
        let bold = TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD);
        let acc = TuiStyle::default().fg(pal.accent);
        let acc_bold = TuiStyle::default()
            .fg(pal.accent)
            .add_modifier(Modifier::BOLD);

        let file = content::files(self.cfg.dev_type)[0];
        let prompt = content::PROMPTS[0];

        let effective_model = self
            .cfg
            .model
            .as_deref()
            .unwrap_or(self.cfg.style.skin().model);

        let mut lines: Vec<Line> = match self.cfg.style {
            Style::Classic => vec![
                Line::from(vec![
                    Span::styled(" \u{25b8} ", acc),
                    Span::styled("imposter \u{00b7} classic system log", bold),
                ]),
                Line::from(Span::styled(
                    " \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2591}\u{2591}\u{2591}  73%  analyzing workspace",
                    green,
                )),
                Line::from(Span::styled(
                    format!(" \u{2713} inspected {file} (194 lines)"),
                    fg,
                )),
                Line::from(Span::styled(
                    " \u{2713} p99 latency  38 ms \u{00b7} throughput 14.2k req/s",
                    fg,
                )),
                Line::from(Span::styled(
                    format!(" \u{25b8} {}", jargon::lines(self.cfg.jargon)[0]),
                    dim,
                )),
            ],
            Style::Claude => {
                let skin = self.cfg.style.skin();
                vec![
                    Line::from(vec![
                        Span::styled(" \u{256d}\u{2500}\u{2500}\u{2500} ", dim),
                        Span::styled("\u{273b} ", acc),
                        Span::styled(skin.title, bold),
                        Span::styled(" \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256e}", dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2502} ", dim),
                        Span::styled("\u{2590}\u{259b}\u{2588}\u{2588}\u{2588}\u{259c}\u{258c} ", acc),
                        Span::styled("Welcome back! ", bold),
                        Span::styled(effective_model, dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2502} ", dim),
                        Span::styled("\u{259d}\u{259c}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259b}\u{2598} ", acc),
                        Span::styled("~/code/distributed-cluster", dim),
                    ]),
                    Line::from(Span::styled(
                        " \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256f}",
                        dim,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{276f} ", acc_bold),
                        Span::styled(prompt, fg),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{25cf} ", acc),
                        Span::styled("Read", bold),
                        Span::styled(format!("({file})"), fg),
                    ]),
                    Line::from(Span::styled(
                        "   \u{23bf}  Read 140 lines (ctrl+o to expand)",
                        dim,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{25cf} ", acc),
                        Span::styled("Update", bold),
                        Span::styled(format!("({file})"), fg),
                    ]),
                    Line::from(Span::styled(
                        "      52 - const MAX_RETRIES: u32 = 3;",
                        red,
                    )),
                    Line::from(Span::styled(
                        "      52 + const MAX_RETRIES: u32 = 5;",
                        green,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{273b}", acc),
                        Span::styled(" Worked for 1m 12s \u{00b7} 3.4k tokens \u{00b7} $0.03", dim),
                    ]),
                    Line::from(Span::styled(format!("   {}", skin.footer), dim)),
                ]
            }
            Style::Codex => {
                let skin = self.cfg.style.skin();
                vec![
                    Line::from(vec![
                        Span::styled(" >_ ", acc_bold),
                        Span::styled(skin.title, bold),
                        Span::styled(format!(" \u{00b7} {effective_model}"), dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{203a} ", acc_bold),
                        Span::styled(prompt, fg),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2022} ", acc),
                        Span::styled("Explored", bold),
                    ]),
                    Line::from(Span::styled(
                        format!("   \u{2514} Read {file} (lines 1\u{2013}140)"),
                        dim,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{2022} ", acc),
                        Span::styled("Edited ", bold),
                        Span::styled(file, fg),
                        Span::styled(" (+5", green),
                        Span::styled(" -2)", red),
                    ]),
                    Line::from(Span::styled(
                        "     52 - const MAX_RETRIES: u32 = 3;",
                        red,
                    )),
                    Line::from(Span::styled(
                        "     52 + const MAX_RETRIES: u32 = 5;",
                        green,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{2022} ", acc),
                        Span::styled("Ran ", bold),
                        Span::styled("cargo test --lib", dim),
                    ]),
                    Line::from(Span::styled(
                        "   \u{2514} test result: ok. 42 passed; 0 failed",
                        dim,
                    )),
                    Line::from(Span::styled(
                        " \u{2500} Worked for 48s \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                        dim,
                    )),
                    Line::from(Span::styled(format!("   {}", skin.footer), dim)),
                ]
            }
            Style::Opencode => {
                let skin = self.cfg.style.skin();
                let (panel_bg, border_fg) = match self.cfg.theme {
                    Theme::Dark => (Color::Rgb(26, 27, 38), Color::Rgb(72, 79, 106)),
                    Theme::Light => (Color::Rgb(230, 233, 239), Color::Rgb(148, 163, 184)),
                };
                let p_border = TuiStyle::default().fg(border_fg).bg(panel_bg);
                let p_acc = TuiStyle::default()
                    .fg(pal.accent)
                    .bg(panel_bg)
                    .add_modifier(Modifier::BOLD);
                let p_bold = TuiStyle::default()
                    .fg(pal.fg)
                    .bg(panel_bg)
                    .add_modifier(Modifier::BOLD);
                let p_fg = TuiStyle::default().fg(pal.fg).bg(panel_bg);
                let p_dim = TuiStyle::default().fg(pal.dim).bg(panel_bg);
                let p_add = TuiStyle::default().fg(pal.green).bg(panel_bg);
                let p_del = TuiStyle::default().fg(pal.red).bg(panel_bg);
                vec![
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_border),
                        Span::styled("# distributed-cluster          ", p_bold),
                        Span::styled("39,413  20% ($0.29) ", p_dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_acc),
                        Span::styled(format!("{prompt:<47}"), p_fg),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", dim),
                        Span::styled(
                            "Thinking: Let me inspect the current retry path first.",
                            dim,
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("   \u{2192} ", acc),
                        Span::styled("Read ", bold),
                        Span::styled(format!("{file} [offset=1, limit=140]"), dim),
                    ]),
                    Line::from(vec![
                        Span::styled("   \u{2190} ", acc),
                        Span::styled("Edit ", bold),
                        Span::styled(file, fg),
                        Span::styled(" (+5", green),
                        Span::styled(" -2)", red),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_border),
                        Span::styled("  52 - const MAX_RETRIES: u32 = 3;             ", p_del),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_border),
                        Span::styled("  52 + const MAX_RETRIES: u32 = 5;             ", p_add),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_border),
                        Span::styled("$ cargo test --lib                             ", p_bold),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2503}  ", p_border),
                        Span::styled("test result: ok. 42 passed; 0 failed           ", p_dim),
                    ]),
                    Line::from(vec![
                        Span::styled("   \u{25a3} Build", acc_bold),
                        Span::styled(format!(" \u{00b7} {effective_model} \u{00b7} 4.2s"), dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}", acc),
                        Span::styled(" esc interrupt   ", dim),
                        Span::styled(skin.footer, dim),
                    ]),
                    Line::from(vec![
                        Span::styled(" ~/code/distributed-cluster (main)   ", dim),
                        Span::styled("\u{2022} ", green),
                        Span::styled("LSP  ", dim),
                        Span::styled("\u{2022} ", green),
                        Span::styled("MCP  1.2.14", dim),
                    ]),
                ]
            }
            Style::Pi => {
                let skin = self.cfg.style.skin();
                let (user_bg, tool_bg) = match self.cfg.theme {
                    Theme::Dark => (Color::Rgb(52, 53, 65), Color::Rgb(40, 50, 40)),
                    Theme::Light => (Color::Rgb(229, 231, 235), Color::Rgb(222, 238, 222)),
                };
                let user_style = TuiStyle::default().fg(pal.fg).bg(user_bg);
                let tool_title = TuiStyle::default()
                    .fg(pal.accent)
                    .bg(tool_bg)
                    .add_modifier(Modifier::BOLD);
                let tool_dim = TuiStyle::default().fg(pal.dim).bg(tool_bg);
                let tool_add = TuiStyle::default().fg(pal.green).bg(tool_bg);
                let tool_del = TuiStyle::default().fg(pal.red).bg(tool_bg);
                vec![
                    Line::from(vec![
                        Span::styled("   \u{2580}\u{2588}\u{2580}  ", acc_bold),
                        Span::styled(skin.title, bold),
                        Span::styled("  \u{00b7}  https://pi.dev", dim),
                    ]),
                    Line::from(Span::styled(
                        "   escape          to interrupt     ctrl+o  to expand tool output",
                        dim,
                    )),
                    Line::from(Span::styled(format!("   {prompt:<48}"), user_style)),
                    Line::from(Span::styled("   Thinking... (ctrl+t to expand)", dim)),
                    Line::from(Span::styled(
                        format!(
                            "   read ~/code/distributed-cluster/{file}:1-140     "
                        ),
                        tool_title,
                    )),
                    Line::from(Span::styled(
                        "   ... (140 lines, ctrl+o to expand)                ",
                        tool_dim,
                    )),
                    Line::from(Span::styled(
                        format!(
                            "   edit ~/code/distributed-cluster/{file} (+5 -2)   "
                        ),
                        tool_title,
                    )),
                    Line::from(Span::styled(
                        "   -52   const MAX_RETRIES: u32 = 3;                ",
                        tool_del,
                    )),
                    Line::from(Span::styled(
                        "   +52   const MAX_RETRIES: u32 = 5;                ",
                        tool_add,
                    )),
                    Line::from(Span::styled(
                        " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                        acc,
                    )),
                    Line::from(Span::styled(" ~/code/distributed-cluster (main)", dim)),
                    Line::from(Span::styled(format!(" {}", skin.footer), dim)),
                ]
            }
            Style::Antigravity => {
                let skin = self.cfg.style.skin();
                vec![
                    Line::from(Span::styled(
                        "   \u{00b7}\u{00b7}\u{00b7}\u{00b7}  \u{00b7}\u{00b7}\u{00b7}\u{00b7}\u{00b7}\u{00b7}  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                        acc,
                    )),
                    Line::from(vec![
                        Span::styled("   Welcome to the Antigravity CLI. ", bold),
                        Span::styled(effective_model, acc),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{276f} ", acc_bold),
                        Span::styled(prompt, fg),
                    ]),
                    Line::from(vec![
                        Span::styled(" \u{25cb} ", acc),
                        Span::styled("Read", bold),
                        Span::styled(format!("({file}:1-140)"), fg),
                    ]),
                    Line::from(Span::styled(
                        "   \u{23bf}  Read 140 lines (ctrl+o to expand)",
                        dim,
                    )),
                    Line::from(vec![
                        Span::styled(" \u{25cb} ", acc),
                        Span::styled("Edit", bold),
                        Span::styled(format!("({file})"), fg),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   \u{23bf}  Updated {file} ("), dim),
                        Span::styled("+5", green),
                        Span::styled(" ", dim),
                        Span::styled("-2", red),
                        Span::styled(")", dim),
                    ]),
                    Line::from(Span::styled(
                        "      52 - const MAX_RETRIES: u32 = 3;",
                        red,
                    )),
                    Line::from(Span::styled(
                        "      52 + const MAX_RETRIES: u32 = 5;",
                        green,
                    )),
                    Line::from(vec![
                        Span::styled("   READY", green),
                        Span::styled(" \u{2502} ", dim),
                        Span::styled(effective_model, acc),
                        Span::styled(" \u{2502} Ctx [\u{2588}\u{2588}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}] 18%", dim),
                    ]),
                    Line::from(Span::styled(format!("   {}", skin.footer), dim)),
                ]
            }
            Style::Hollywood => {
                let skin = self.cfg.style.skin();
                vec![
                    Line::from(Span::styled("  :: HOLLYWOOD CYBER WARFARE TERMINAL ::", acc_bold)),
                    Line::from(vec![
                        Span::styled("  TARGET: ", dim),
                        Span::styled("PENTAGON_PRIMARY_GRID", TuiStyle::default().fg(Color::Yellow)),
                        Span::styled(" \u{00b7} ", dim),
                        Span::styled(effective_model, acc),
                    ]),
                    Line::from(Span::styled("  0x7FFF94A0  48 89 E5 48 83 EC 20 E8 |H..H.. .|", acc)),
                    Line::from(Span::styled("  0x7FFF94B0  B8 00 00 00 00 48 89 EC |.....H..|", acc)),
                    Line::from(Span::styled("  [████████████████████░░░░░░░░░░] 68% OVERRIDING KERNEL", acc)),
                    Line::from(Span::styled("  \u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}", acc)),
                    Line::from(Span::styled("  \u{2551}           [ ACCESS GRANTED \u{00b7} ROOT SHELL ACQUIRED ]           \u{2551}", acc_bold)),
                    Line::from(Span::styled("  \u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}", acc)),
                    Line::from(Span::styled(format!("  {}", skin.footer), dim)),
                ]
            }
        };

        if self.cfg.complexity != Complexity::Low && self.cfg.style != Style::Classic {
            lines.push(Line::from(""));
            match self.cfg.style {
                Style::Antigravity => {
                    lines.push(Line::from(vec![
                        Span::styled(" \u{25cb} ", acc),
                        Span::styled("Subagents", bold),
                        Span::styled("(3 active \u{00b7} /agents)", fg),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("   \u{23bf}  ", dim),
                        Span::styled("\u{2713} ", green),
                        Span::styled("[research] ", acc),
                        Span::styled("Read the current retry path", dim),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("      ", dim),
                        Span::styled("\u{27f3} ", acc),
                        Span::styled("[verify]   ", acc),
                        Span::styled("Cover the 503 path with a test", fg),
                    ]));
                }
                Style::Codex => {
                    lines.push(Line::from(vec![
                        Span::styled(" \u{2022} ", acc),
                        Span::styled("Updated Plan", bold),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("   \u{2514} ", dim),
                        Span::styled("\u{2714} ", green),
                        Span::styled("Read the current retry path", dim),
                    ]));
                }
                Style::Opencode => {
                    lines.push(Line::from(vec![
                        Span::styled(" \u{2503}  ", dim),
                        Span::styled("# Todo  ", bold),
                        Span::styled("[\u{2713}] Read retry path  ", dim),
                        Span::styled("[\u{2022}] Cover 503 test", fg),
                    ]));
                }
                _ => {
                    lines.push(Line::from(vec![
                        Span::styled("   \u{2612} ", green),
                        Span::styled("Read the current retry path  ", dim),
                        Span::styled("\u{2610} ", dim),
                        Span::styled("Cover 503 test", fg),
                    ]));
                }
            }
        }

        if self.cfg.alerts {
            lines.push(Line::from(vec![
                Span::styled("   ! ", red),
                Span::styled(
                    "error: this `if` has identical blocks (auto-fixing\u{2026})",
                    red,
                ),
            ]));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(TuiStyle::default().fg(pal.border))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg))
            .title(Span::styled(
                format!(
                    " Live Preview ({} \u{00b7} {}) ",
                    self.cfg.style.as_str(),
                    self.cfg.theme.as_str()
                ),
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            ));
        f.render_widget(
            Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_footer(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let accent = pal.accent;
        let line = if let Some(status) = &self.status {
            Line::from(Span::styled(
                format!(" {status}"),
                TuiStyle::default().fg(accent).add_modifier(Modifier::BOLD),
            ))
        } else if self.editing {
            Line::from(vec![
                keycap("Enter", accent),
                Span::raw(" Commit   "),
                keycap("Backspace", accent),
                Span::raw(" Delete   "),
                keycap("Esc", accent),
                Span::raw(" Cancel editing"),
            ])
        } else {
            Line::from(vec![
                keycap("\u{2191}\u{2193}/Tab", accent),
                Span::raw(" Move  "),
                keycap("\u{2190}\u{2192}/Space", accent),
                Span::raw(" Cycle  "),
                keycap("Enter", accent),
                Span::raw(" Edit  "),
                keycap("1-7", accent),
                Span::raw(" Style  "),
                keycap("t", accent),
                Span::raw(" Theme  "),
                keycap("Ctrl+R", accent),
                Span::raw(" Start  "),
                keycap("Ctrl+S", accent),
                Span::raw(" Save  "),
                keycap("q/Esc", accent),
                Span::raw(" Quit"),
            ])
        };

        let help = Block::default()
            .borders(Borders::TOP)
            .border_style(TuiStyle::default().fg(pal.border))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.muted));
        f.render_widget(
            Paragraph::new(line)
                .style(TuiStyle::default().bg(pal.bg).fg(pal.muted))
                .block(help),
            area,
        );
    }
}

#[allow(dead_code)]
fn toggle_spans(on: bool, pal: &Palette) -> Vec<Span<'static>> {
    if on {
        vec![Span::styled(
            "[\u{25cf} ON ]",
            TuiStyle::default()
                .fg(pal.accent)
                .add_modifier(Modifier::BOLD),
        )]
    } else {
        vec![Span::styled(
            "[\u{25cb} OFF]",
            TuiStyle::default().fg(pal.dim),
        )]
    }
}

fn keycap(label: &'static str, accent: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        TuiStyle::default().fg(accent).add_modifier(Modifier::BOLD),
    )
}

fn cycle<T: Copy + PartialEq>(all: &[T], current: T, dir: i32) -> T {
    let idx = all.iter().position(|v| *v == current).unwrap_or(0) as i32;
    let n = all.len() as i32;
    all[(((idx + dir) % n + n) % n) as usize]
}

pub fn run(cfg: Config) -> Option<Config> {
    if enable_raw_mode().is_err() {
        return None;
    }
    let mut out = stdout();
    if execute!(out, EnterAlternateScreen, EnableMouseCapture).is_err() {
        let _ = disable_raw_mode();
        return None;
    }
    let backend = CrosstermBackend::new(out);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(_) => {
            let _ = disable_raw_mode();
            return None;
        }
    };

    let mut app = App::new(cfg);
    let mut result: Option<Config> = None;

    loop {
        if terminal.draw(|f| app.draw(f)).is_err() {
            break;
        }
        match event::poll(Duration::from_millis(120)) {
            Ok(true) => {}
            Ok(false) => continue,
            Err(_) => break,
        }
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                app.key(key.code, key.modifiers);
            }
            Ok(Event::Mouse(mouse)) => {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    app.mouse_click(mouse.column, mouse.row);
                }
            }
            _ => continue,
        }
        if app.start {
            result = Some(app.cfg.clone());
            break;
        }
        if app.quit {
            break;
        }
    }

    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();
    result
}
