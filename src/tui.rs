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
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style as TuiStyle};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap};
use ratatui::{Frame, Terminal};
use std::io::stdout;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tab {
    Main,
    Settings,
    About,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SettingsField {
    Style,
    Model,
    Speed,
    Theme,
    Duration,
    Save,
}

const SETTINGS_FIELDS: [SettingsField; 6] = [
    SettingsField::Style,
    SettingsField::Model,
    SettingsField::Speed,
    SettingsField::Theme,
    SettingsField::Duration,
    SettingsField::Save,
];

impl SettingsField {
    fn label(self) -> &'static str {
        match self {
            SettingsField::Style => "Style",
            SettingsField::Model => "Model",
            SettingsField::Speed => "Speed",
            SettingsField::Theme => "Theme",
            SettingsField::Duration => "Duration",
            SettingsField::Save => "",
        }
    }

    #[allow(dead_code)]
    fn editable(self) -> bool {
        matches!(self, SettingsField::Duration | SettingsField::Model)
    }
}

const IMPOSTER_BANNER: &[&str] = &[
    r" ___ __  __ ___  ___  ____ _____ ___  ____  ",
    r"|_ _|  \/  | _ \/ _ \/ ___|_   _/ _ \|  _ \ ",
    r" | || |\/| |  _/ (_) \___ \ | || (_) | |_) |",
    r"|___|_|  |_|_|  \___/|____/ |_| \___/|_| \_\",
];

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
    active_tab: Tab,
    settings_field: usize,
    editing: bool,
    buf: String,
    status: Option<String>,
    start: bool,
    quit: bool,
    tick: usize,
}

impl App {
    fn new(cfg: Config) -> Self {
        Self {
            cfg,
            active_tab: Tab::Main,
            settings_field: 0,
            editing: false,
            buf: String::new(),
            status: None,
            start: false,
            quit: false,
            tick: 0,
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
                    if SETTINGS_FIELDS[self.settings_field] == SettingsField::Duration {
                        if c.is_ascii_digit() && self.buf.len() < 3 {
                            self.buf.push(c);
                        }
                    } else if SETTINGS_FIELDS[self.settings_field] == SettingsField::Model
                        && self.buf.len() < 32
                    {
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
            KeyCode::Char('\t') | KeyCode::BackTab => {
                self.active_tab = match self.active_tab {
                    Tab::Main => Tab::Settings,
                    Tab::Settings => Tab::About,
                    Tab::About => Tab::Main,
                };
            }
            KeyCode::Char('1') => self.active_tab = Tab::Main,
            KeyCode::Char('2') => self.active_tab = Tab::Settings,
            KeyCode::Char('3') => self.active_tab = Tab::About,
            _ => match self.active_tab {
                Tab::Main => match code {
                    KeyCode::Enter | KeyCode::Char(' ') => self.start = true,
                    KeyCode::Right | KeyCode::Char('l') => self.active_tab = Tab::Settings,
                    _ => {}
                },
                Tab::Settings => match code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.settings_field = (self.settings_field + SETTINGS_FIELDS.len() - 1)
                            % SETTINGS_FIELDS.len();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.settings_field = (self.settings_field + 1) % SETTINGS_FIELDS.len();
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if SETTINGS_FIELDS[self.settings_field] != SettingsField::Save {
                            self.cycle(-1);
                        } else {
                            self.active_tab = Tab::Main;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if SETTINGS_FIELDS[self.settings_field] != SettingsField::Save {
                            self.cycle(1);
                        } else {
                            self.active_tab = Tab::About;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if SETTINGS_FIELDS[self.settings_field] == SettingsField::Save {
                            self.save();
                        } else {
                            self.cycle(1);
                        }
                    }
                    KeyCode::Char('t') => {
                        self.cfg.theme = cycle(&Theme::ALL, self.cfg.theme, 1);
                    }
                    KeyCode::Char('s') => self.save(),
                    KeyCode::Enter => match SETTINGS_FIELDS[self.settings_field] {
                        SettingsField::Save => self.save(),
                        SettingsField::Duration => {
                            self.buf = self.cfg.duration.to_string();
                            self.editing = true;
                        }
                        SettingsField::Model => {
                            self.buf = self.cfg.model.clone().unwrap_or_default();
                            self.editing = true;
                        }
                        _ => self.cycle(1),
                    },
                    _ => {}
                },
                Tab::About => match code {
                    KeyCode::Left | KeyCode::Char('h') => self.active_tab = Tab::Settings,
                    KeyCode::Right | KeyCode::Char('l') => self.active_tab = Tab::Main,
                    KeyCode::Enter | KeyCode::Char(' ') => self.start = true,
                    _ => {}
                },
            },
        }
    }

    fn mouse_click(&mut self, col: u16, row: u16) {
        // Tab bar click (row 1 or 2)
        if row <= 2 {
            if col < 14 {
                self.active_tab = Tab::Main;
            } else if col < 30 {
                self.active_tab = Tab::Settings;
            } else if col < 44 {
                self.active_tab = Tab::About;
            }
            return;
        }

        match self.active_tab {
            Tab::Main => {
                // START button is roughly in rows 9..=15
                if (9..=16).contains(&row) {
                    self.start = true;
                }
            }
            Tab::Settings => {
                // Settings items in row 4..=9
                if (4..=8).contains(&row) {
                    let clicked = (row - 4) as usize;
                    if clicked < 5 {
                        if self.settings_field == clicked {
                            if SETTINGS_FIELDS[self.settings_field].editable() {
                                if SETTINGS_FIELDS[self.settings_field] == SettingsField::Duration {
                                    self.buf = self.cfg.duration.to_string();
                                } else {
                                    self.buf = self.cfg.model.clone().unwrap_or_default();
                                }
                                self.editing = true;
                            } else {
                                self.cycle(1);
                            }
                        } else {
                            self.settings_field = clicked;
                        }
                    }
                } else if (12..=15).contains(&row) {
                    self.settings_field = SETTINGS_FIELDS
                        .iter()
                        .position(|f| *f == SettingsField::Save)
                        .unwrap_or(0);
                    self.save();
                }
            }
            Tab::About => {}
        }
    }

    fn cycle(&mut self, dir: i32) {
        match SETTINGS_FIELDS[self.settings_field] {
            SettingsField::Style => self.cfg.style = cycle(&Style::ALL, self.cfg.style, dir),
            SettingsField::Model => {
                if self.cfg.model.is_some() {
                    self.cfg.model = None;
                }
            }
            SettingsField::Speed => {
                self.cfg.speed = cycle(&crate::types::Speed::ALL, self.cfg.speed, dir)
            }
            SettingsField::Theme => self.cfg.theme = cycle(&Theme::ALL, self.cfg.theme, dir),
            SettingsField::Duration => {
                let step = 5i64;
                let next = self.cfg.duration as i64 + if dir > 0 { step } else { -step };
                self.cfg.duration = next.clamp(0, crate::cli::MAX_MINUTES as i64) as u64;
            }
            _ => {}
        }
    }

    fn commit_edit(&mut self) {
        if SETTINGS_FIELDS[self.settings_field] == SettingsField::Duration {
            if let Ok(v) = self.buf.trim().parse::<u64>() {
                self.cfg.duration = v.min(crate::cli::MAX_MINUTES);
            }
        } else if SETTINGS_FIELDS[self.settings_field] == SettingsField::Model {
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

        // Terminal too small check
        if area.width < 45 || area.height < 12 {
            let warn_lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "⚠️  Terminal Too Small",
                    TuiStyle::default()
                        .fg(pal.accent)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!("Current: {}x{}", area.width, area.height),
                    TuiStyle::default().fg(pal.muted),
                )),
                Line::from(Span::styled(
                    "Min required: 45x12",
                    TuiStyle::default().fg(pal.dim),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Please resize terminal window",
                    TuiStyle::default().fg(pal.fg),
                )),
            ];
            let warn_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(TuiStyle::default().fg(pal.accent));
            f.render_widget(
                Paragraph::new(warn_lines)
                    .alignment(Alignment::Center)
                    .block(warn_block),
                area,
            );
            return;
        }

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // BIOS Header + Tab bar
                Constraint::Min(6),    // Tab content
                Constraint::Length(2), // Status & Footer
            ])
            .split(area);

        self.draw_tabs(f, main_layout[0], &pal);

        match self.active_tab {
            Tab::Main => self.draw_main_tab(f, main_layout[1], &pal),
            Tab::Settings => self.draw_settings_tab(f, main_layout[1], &pal),
            Tab::About => self.draw_about_tab(f, main_layout[1], &pal),
        }

        self.draw_footer(f, main_layout[2], &pal);
    }

    fn draw_tabs(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let titles = vec![
            Line::from(" [1] MAIN "),
            Line::from(" [2] SETTINGS "),
            Line::from(" [3] ABOUT "),
        ];

        let selected_idx = match self.active_tab {
            Tab::Main => 0,
            Tab::Settings => 1,
            Tab::About => 2,
        };

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(TuiStyle::default().fg(pal.border))
                    .title(Span::styled(
                        " IMPOSTER ",
                        TuiStyle::default()
                            .fg(pal.accent)
                            .add_modifier(Modifier::BOLD),
                    )),
            )
            .select(selected_idx)
            .style(TuiStyle::default().fg(pal.muted))
            .highlight_style(
                TuiStyle::default()
                    .fg(pal.badge_fg)
                    .bg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(Span::styled("│", TuiStyle::default().fg(pal.dim)));

        f.render_widget(tabs, area);
    }

    fn draw_main_tab(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let (base_r, base_g, base_b) = if self.cfg.style == Style::Classic {
            match self.cfg.theme {
                Theme::Dark => (80u8, 200u8, 120u8),
                Theme::Light => (17u8, 99u8, 41u8),
            }
        } else {
            self.cfg.style.skin().rgb_for(self.cfg.theme)
        };

        let mut lines = Vec::new();
        lines.push(Line::from(""));

        // If terminal width is tight or height is small, use compact banner
        let is_compact = area.width < 62 || area.height < 16;
        if is_compact {
            lines.push(Line::from(Span::styled(
                "═══  I M P O S T E R  ═══",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            for (row_idx, line) in IMPOSTER_BANNER.iter().enumerate() {
                let mut spans = Vec::new();
                for (col_idx, ch) in line.chars().enumerate() {
                    // Wave phase combines column position, row offset, and tick counter
                    let phase = ((col_idx * 4 + row_idx * 12 + self.tick * 8) % 360) as f32;
                    let rad = phase * std::f32::consts::PI / 180.0;
                    let sin_val = (rad.sin() + 1.0) / 2.0; // 0.0 ..= 1.0

                    // Interpolate brightness/glow around the base accent color
                    let factor = 0.55 + 0.65 * sin_val; // 0.55 ..= 1.20
                    let r = ((base_r as f32 * factor).min(255.0)) as u8;
                    let g = ((base_g as f32 * factor).min(255.0)) as u8;
                    let b = ((base_b as f32 * factor).min(255.0)) as u8;

                    spans.push(Span::styled(
                        ch.to_string(),
                        TuiStyle::default()
                            .fg(Color::Rgb(r, g, b))
                            .add_modifier(Modifier::BOLD),
                    ));
                }
                lines.push(Line::from(spans));
            }
        }

        if !is_compact {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "╔═══════════════════════════════════════════════╗",
            TuiStyle::default().fg(pal.accent),
        )));
        lines.push(Line::from(Span::styled(
            "║               ▶  S T A R T  ◀                 ║",
            TuiStyle::default()
                .fg(pal.badge_fg)
                .bg(pal.accent)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "╚═══════════════════════════════════════════════╝",
            TuiStyle::default().fg(pal.accent),
        )));
        lines.push(Line::from(Span::styled(
            "Press [Enter] or [Space] to Start",
            TuiStyle::default()
                .fg(pal.dim)
                .add_modifier(Modifier::ITALIC),
        )));

        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(TuiStyle::default().fg(pal.border));

        f.render_widget(
            Paragraph::new(lines)
                .alignment(Alignment::Center)
                .block(main_block),
            area,
        );
    }

    fn draw_about_tab(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "imposter",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "\"아무것도 하지 않으면서, 최신 AI 코딩 에이전트로 바이브 코딩하는 척하기\"",
                TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Author  : ", TuiStyle::default().fg(pal.muted)),
                Span::styled("deepthought", TuiStyle::default().fg(pal.accent)),
            ]),
            Line::from(vec![
                Span::styled("Source  : ", TuiStyle::default().fg(pal.muted)),
                Span::styled(
                    "https://github.com/partrita/imposter-cli",
                    TuiStyle::default().fg(pal.fg),
                ),
            ]),
            Line::from(vec![
                Span::styled("Original: ", TuiStyle::default().fg(pal.dim)),
                Span::styled(
                    "https://github.com/giacomo-b/rust-stakeholder",
                    TuiStyle::default().fg(pal.dim),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "─── USAGE & SHORTCUTS ───",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(vec![
                Span::styled(
                    "  imposter           ",
                    TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    ": 대화형 TUI 설정 마법사 실행",
                    TuiStyle::default().fg(pal.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  imposter -c        ",
                    TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    ": 저장된 설정으로 바로 세션 실행",
                    TuiStyle::default().fg(pal.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  imposter -s codex  ",
                    TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    ": 지정한 스타일로 세션 직접 실행",
                    TuiStyle::default().fg(pal.muted),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Ctrl+C / Esc       ",
                    TuiStyle::default()
                        .fg(pal.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    ": 세션 실행 중 언제든 즉시 종료",
                    TuiStyle::default().fg(pal.muted),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Minimal dependencies · Single-threaded · Zero token cost",
                TuiStyle::default().fg(pal.dim),
            )),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(TuiStyle::default().fg(pal.border))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg))
            .title(Span::styled(
                " About ",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            ));

        f.render_widget(
            Paragraph::new(lines)
                .alignment(Alignment::Center)
                .block(block)
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_settings_tab(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        if area.width < 75 {
            // Narrow responsive layout: single vertical column
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(10), Constraint::Length(5)])
                .split(area);

            self.draw_settings_form(f, rows[0], pal);
            self.draw_settings_actions(f, rows[1], pal);
        } else {
            // Wide layout: 2-column (form + preview)
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
                .split(area);

            let left_rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(11), Constraint::Length(5)])
                .split(cols[0]);

            self.draw_settings_form(f, left_rows[0], pal);
            self.draw_settings_actions(f, left_rows[1], pal);
            self.draw_preview(f, cols[1], pal);
        }
    }

    fn draw_settings_form(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let mut lines: Vec<Line> = Vec::with_capacity(10);

        for (i, field) in SETTINGS_FIELDS.iter().enumerate() {
            if *field == SettingsField::Save {
                continue;
            }
            let selected = i == self.settings_field;
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
        let dim = TuiStyle::default().fg(pal.dim);
        let (_, desc_body) = self.field_description(SETTINGS_FIELDS[self.settings_field]);
        lines.push(Line::from(vec![
            Span::styled("   \u{00bb} ", TuiStyle::default().fg(pal.accent)),
            Span::styled(desc_body, dim),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(TuiStyle::default().fg(pal.border))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg))
            .title(Span::styled(
                " Settings ",
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

    fn draw_settings_actions(&self, f: &mut Frame, area: Rect, pal: &Palette) {
        let save_sel = SETTINGS_FIELDS[self.settings_field] == SettingsField::Save;

        let save_style = if save_sel {
            TuiStyle::default()
                .fg(pal.badge_fg)
                .bg(pal.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default().fg(pal.fg).add_modifier(Modifier::BOLD)
        };

        let save_cursor = if save_sel { "\u{25b6} " } else { "  " };

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!(" {save_cursor}"),
                    TuiStyle::default().fg(pal.accent),
                ),
                Span::styled(" [ SAVE CONFIGURATION (Enter) ] ", save_style),
            ]),
        ];

        let action_border_color = if save_sel { pal.accent } else { pal.border };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(TuiStyle::default().fg(action_border_color))
            .style(TuiStyle::default().bg(pal.bg).fg(pal.fg))
            .title(Span::styled(
                " Save ",
                TuiStyle::default()
                    .fg(pal.accent)
                    .add_modifier(Modifier::BOLD),
            ));

        f.render_widget(Paragraph::new(lines).block(block), area);
    }

    fn render_field_value(
        &self,
        field: SettingsField,
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
            SettingsField::Style => {
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
            SettingsField::Model => {
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
            SettingsField::Speed => {
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
            SettingsField::Theme => {
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
            SettingsField::Duration => {
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
            SettingsField::Save => Vec::new(),
        }
    }

    fn field_description(&self, field: SettingsField) -> (&'static str, String) {
        match field {
            SettingsField::Style => {
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
            SettingsField::Model => (
                "AI Model Designation",
                "Custom model name (e.g. claude-3-7-sonnet, gpt-4o, gemini-2.5-pro). Leave empty for style default."
                    .to_string(),
            ),
            SettingsField::Speed => (
                "Transcript Playback Speed",
                "Slow (0.5x), Normal (1.0x), Fast (2.0x), Turbo (5.0x) with natural jitter variations."
                    .to_string(),
            ),
            SettingsField::Theme => (
                "Color Theme (Dark / Light)",
                "Adapts accent, text, border, and diff contrast for dark or light terminals (shortcut: t)."
                    .to_string(),
            ),
            SettingsField::Duration => (
                "Auto-Stop Timer",
                "0 runs until Ctrl-C. Set 1-999 minutes to exit cleanly on a deadline.".to_string(),
            ),
            SettingsField::Save => (
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
                Span::raw(" Cancel"),
            ])
        } else {
            match self.active_tab {
                Tab::Main => Line::from(vec![
                    keycap("Enter/Space", accent),
                    Span::raw(" Start  "),
                    keycap("Tab", accent),
                    Span::raw(" Next Tab  "),
                    keycap("q / Esc", accent),
                    Span::raw(" Quit"),
                ]),
                Tab::Settings => Line::from(vec![
                    keycap("\u{2191}\u{2193}/j,k", accent),
                    Span::raw(" Select  "),
                    keycap("\u{2190}\u{2192}/Space", accent),
                    Span::raw(" Cycle  "),
                    keycap("Enter", accent),
                    Span::raw(" Edit/Save  "),
                    keycap("t", accent),
                    Span::raw(" Theme  "),
                    keycap("Ctrl+S", accent),
                    Span::raw(" Save  "),
                    keycap("Tab", accent),
                    Span::raw(" Next Tab  "),
                    keycap("q/Esc", accent),
                    Span::raw(" Quit"),
                ]),
                Tab::About => Line::from(vec![
                    keycap("Enter/Space", accent),
                    Span::raw(" Start  "),
                    keycap("Tab", accent),
                    Span::raw(" Next Tab  "),
                    keycap("q / Esc", accent),
                    Span::raw(" Quit"),
                ]),
            }
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
        app.tick = app.tick.wrapping_add(1);
        if terminal.draw(|f| app.draw(f)).is_err() {
            break;
        }
        match event::poll(Duration::from_millis(80)) {
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
