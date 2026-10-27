use crate::config::SessionConfig;
use crate::types::{Complexity, DevelopmentType, JargonLevel};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io::{self, stdout, Stdout};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormField {
    DevType,
    Jargon,
    Complexity,
    ProjectName,
    Framework,
    Duration,
    Alerts,
    Minimal,
    Team,
    StartButton,
    SaveButton,
}

impl FormField {
    pub const ALL: [FormField; 11] = [
        FormField::DevType,
        FormField::Jargon,
        FormField::Complexity,
        FormField::ProjectName,
        FormField::Framework,
        FormField::Duration,
        FormField::Alerts,
        FormField::Minimal,
        FormField::Team,
        FormField::StartButton,
        FormField::SaveButton,
    ];

    pub fn next(&self) -> Self {
        let idx = Self::ALL.iter().position(|f| f == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }

    pub fn prev(&self) -> Self {
        let idx = Self::ALL.iter().position(|f| f == self).unwrap_or(0);
        if idx == 0 {
            Self::ALL[Self::ALL.len() - 1]
        } else {
            Self::ALL[idx - 1]
        }
    }
}

pub struct TuiApp {
    pub config: SessionConfig,
    pub duration: u64,
    pub current_field: FormField,
    pub is_editing: bool,
    pub edit_buffer: String,
    pub status_message: Option<(String, Color)>,
    pub should_start: bool,
    pub should_exit: bool,
}

impl TuiApp {
    pub fn new(initial_config: SessionConfig, initial_duration: u64) -> Self {
        Self {
            config: initial_config,
            duration: initial_duration,
            current_field: FormField::DevType,
            is_editing: false,
            edit_buffer: String::new(),
            status_message: None,
            should_start: false,
            should_exit: false,
        }
    }

    pub fn run_setup(mut self) -> io::Result<Option<(SessionConfig, u64)>> {
        enable_raw_mode()?;
        let mut stdout_handle = stdout();
        execute!(stdout_handle, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout_handle);
        let mut terminal = Terminal::new(backend)?;

        let res = self.event_loop(&mut terminal);

        // Always clean up terminal state before returning
        let _ = disable_raw_mode();
        let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
        let _ = terminal.show_cursor();

        res
    }

    fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> io::Result<Option<(SessionConfig, u64)>> {
        loop {
            terminal.draw(|f| self.draw_ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code, key.modifiers);
                    }
                }
            }

            if self.should_start {
                return Ok(Some((self.config.clone(), self.duration)));
            }

            if self.should_exit {
                return Ok(None);
            }
        }
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Global shortcuts
        if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
            self.should_exit = true;
            return;
        }

        if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('s') || code == KeyCode::F(5) {
            self.save_config();
            return;
        }

        if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('r') {
            self.should_start = true;
            return;
        }

        // Editing mode for text inputs
        if self.is_editing {
            match code {
                KeyCode::Enter => {
                    self.apply_editing();
                    self.is_editing = false;
                }
                KeyCode::Esc => {
                    self.is_editing = false;
                    self.edit_buffer.clear();
                }
                KeyCode::Backspace => {
                    self.edit_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.edit_buffer.push(c);
                }
                _ => {}
            }
            return;
        }

        // Normal navigation mode
        match code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
                self.current_field = self.current_field.next();
            }
            KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
                self.current_field = self.current_field.prev();
            }
            KeyCode::Enter => {
                match self.current_field {
                    FormField::ProjectName => {
                        self.is_editing = true;
                        self.edit_buffer = self.config.project_name.clone();
                    }
                    FormField::Framework => {
                        self.is_editing = true;
                        self.edit_buffer = self.config.framework.clone();
                    }
                    FormField::Duration => {
                        self.is_editing = true;
                        self.edit_buffer = if self.duration == 0 {
                            String::new()
                        } else {
                            self.duration.to_string()
                        };
                    }
                    FormField::Alerts => self.config.alerts_enabled = !self.config.alerts_enabled,
                    FormField::Minimal => self.config.minimal_output = !self.config.minimal_output,
                    FormField::Team => self.config.team_activity = !self.config.team_activity,
                    FormField::StartButton => self.should_start = true,
                    FormField::SaveButton => self.save_config(),
                    FormField::DevType => self.cycle_dev_type(1),
                    FormField::Jargon => self.cycle_jargon(1),
                    FormField::Complexity => self.cycle_complexity(1),
                }
            }
            KeyCode::Char(' ') => {
                match self.current_field {
                    FormField::Alerts => self.config.alerts_enabled = !self.config.alerts_enabled,
                    FormField::Minimal => self.config.minimal_output = !self.config.minimal_output,
                    FormField::Team => self.config.team_activity = !self.config.team_activity,
                    FormField::DevType => self.cycle_dev_type(1),
                    FormField::Jargon => self.cycle_jargon(1),
                    FormField::Complexity => self.cycle_complexity(1),
                    FormField::StartButton => self.should_start = true,
                    FormField::SaveButton => self.save_config(),
                    _ => {}
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                match self.current_field {
                    FormField::DevType => self.cycle_dev_type(-1),
                    FormField::Jargon => self.cycle_jargon(-1),
                    FormField::Complexity => self.cycle_complexity(-1),
                    FormField::Duration => {
                        if self.duration >= 5 {
                            self.duration -= 5;
                        } else {
                            self.duration = 0;
                        }
                    }
                    FormField::Alerts => self.config.alerts_enabled = !self.config.alerts_enabled,
                    FormField::Minimal => self.config.minimal_output = !self.config.minimal_output,
                    FormField::Team => self.config.team_activity = !self.config.team_activity,
                    _ => {}
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                match self.current_field {
                    FormField::DevType => self.cycle_dev_type(1),
                    FormField::Jargon => self.cycle_jargon(1),
                    FormField::Complexity => self.cycle_complexity(1),
                    FormField::Duration => {
                        self.duration += 5;
                    }
                    FormField::Alerts => self.config.alerts_enabled = !self.config.alerts_enabled,
                    FormField::Minimal => self.config.minimal_output = !self.config.minimal_output,
                    FormField::Team => self.config.team_activity = !self.config.team_activity,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn apply_editing(&mut self) {
        match self.current_field {
            FormField::ProjectName => {
                let trimmed = self.edit_buffer.trim();
                self.config.project_name = if trimmed.is_empty() {
                    "distributed-cluster".to_string()
                } else {
                    trimmed.to_string()
                };
            }
            FormField::Framework => {
                self.config.framework = self.edit_buffer.trim().to_string();
            }
            FormField::Duration => {
                if let Ok(parsed) = self.edit_buffer.trim().parse::<u64>() {
                    self.duration = parsed;
                } else if self.edit_buffer.trim().is_empty() {
                    self.duration = 0;
                }
            }
            _ => {}
        }
        self.edit_buffer.clear();
    }

    fn cycle_dev_type(&mut self, delta: isize) {
        let all = &DevelopmentType::ALL;
        let cur = all.iter().position(|&x| x == self.config.dev_type).unwrap_or(0);
        let len = all.len() as isize;
        let next_idx = ((cur as isize + delta) % len + len) % len;
        self.config.dev_type = all[next_idx as usize];
    }

    fn cycle_jargon(&mut self, delta: isize) {
        let all = &JargonLevel::ALL;
        let cur = all.iter().position(|&x| x == self.config.jargon_level).unwrap_or(0);
        let len = all.len() as isize;
        let next_idx = ((cur as isize + delta) % len + len) % len;
        self.config.jargon_level = all[next_idx as usize];
    }

    fn cycle_complexity(&mut self, delta: isize) {
        let all = &Complexity::ALL;
        let cur = all.iter().position(|&x| x == self.config.complexity).unwrap_or(0);
        let len = all.len() as isize;
        let next_idx = ((cur as isize + delta) % len + len) % len;
        self.config.complexity = all[next_idx as usize];
    }

    fn save_config(&mut self) {
        match self.config.save() {
            Ok(path) => {
                self.status_message = Some((
                    format!("Saved to {}", path.display()),
                    Color::Green,
                ));
            }
            Err(e) => {
                self.status_message = Some((format!("Save error: {}", e), Color::Red));
            }
        }
    }

    fn draw_ui(&self, f: &mut Frame) {
        let size = f.area();

        // Main Layout
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(16),   // Body
                Constraint::Length(3),  // Footer / Status
            ])
            .split(size);

        // Header
        let header_text = vec![
            Line::from(vec![
                Span::styled(" 🚀 RUST STAKEHOLDER SIMULATOR ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("- Interactive Setup Wizard", Style::default().fg(Color::White)),
            ]),
        ];
        let header = Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);
        f.render_widget(header, main_chunks[0]);

        // Body split: Left (Select Options), Right (Text inputs, Checkboxes, Action Buttons)
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

        self.draw_left_column(f, body_chunks[0]);
        self.draw_right_column(f, body_chunks[1]);

        // Footer / Help Bar
        let footer_text = if let Some((msg, color)) = &self.status_message {
            Line::from(vec![
                Span::styled(" [STATUS] ", Style::default().fg(*color).add_modifier(Modifier::BOLD)),
                Span::styled(msg.as_str(), Style::default().fg(Color::White)),
                Span::raw(" | "),
                Span::styled("Press Enter/Space to dismiss", Style::default().fg(Color::DarkGray)),
            ])
        } else {
            Line::from(vec![
                Span::styled(" [↑/↓/Tab] ", Style::default().fg(Color::Yellow)),
                Span::raw("Move  "),
                Span::styled(" [←/→/Space] ", Style::default().fg(Color::Yellow)),
                Span::raw("Change  "),
                Span::styled(" [Enter] ", Style::default().fg(Color::Yellow)),
                Span::raw("Edit/Select  "),
                Span::styled(" [Ctrl+S/F5] ", Style::default().fg(Color::Green)),
                Span::raw("Save  "),
                Span::styled(" [Ctrl+R/Enter] ", Style::default().fg(Color::Cyan)),
                Span::raw("Start  "),
                Span::styled(" [Esc/q] ", Style::default().fg(Color::Red)),
                Span::raw("Quit"),
            ])
        };

        let footer = Paragraph::new(footer_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .alignment(Alignment::Center);
        f.render_widget(footer, main_chunks[2]);
    }

    fn draw_left_column(&self, f: &mut Frame, area: Rect) {
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Dev Type
                Constraint::Length(5), // Jargon Level
                Constraint::Length(5), // Complexity
            ])
            .split(area);

        // 1. Development Type
        let is_dev_active = self.current_field == FormField::DevType;
        let dev_border_color = if is_dev_active { Color::Yellow } else { Color::DarkGray };
        let dev_types_spans: Vec<Line> = DevelopmentType::ALL
            .chunks(2)
            .map(|chunk| {
                let mut spans = Vec::new();
                for &dtype in chunk {
                    let is_selected = dtype == self.config.dev_type;
                    let prefix = if is_selected { " [●] " } else { " [ ] " };
                    let style = if is_selected {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    spans.push(Span::styled(format!("{}{:<18}", prefix, dtype.as_str()), style));
                }
                Line::from(spans)
            })
            .collect();

        let dev_box = Paragraph::new(dev_types_spans).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " {} Development Domain (←/→ or Enter) ",
                    if is_dev_active { "▶" } else { " " }
                ))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(dev_border_color)),
        );
        f.render_widget(dev_box, left_chunks[0]);

        // 2. Jargon Level
        let is_jargon_active = self.current_field == FormField::Jargon;
        let jargon_border_color = if is_jargon_active { Color::Yellow } else { Color::DarkGray };
        let jargon_spans: Vec<Span> = JargonLevel::ALL
            .iter()
            .map(|&lvl| {
                let is_selected = lvl == self.config.jargon_level;
                let text = format!(" {} {} ", if is_selected { "●" } else { "○" }, lvl.as_str());
                if is_selected {
                    Span::styled(text, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(text, Style::default().fg(Color::DarkGray))
                }
            })
            .collect();

        let jargon_box = Paragraph::new(vec![Line::from(jargon_spans)]).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " {} Technical Jargon Intensity ",
                    if is_jargon_active { "▶" } else { " " }
                ))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(jargon_border_color)),
        );
        f.render_widget(jargon_box, left_chunks[1]);

        // 3. Complexity
        let is_comp_active = self.current_field == FormField::Complexity;
        let comp_border_color = if is_comp_active { Color::Yellow } else { Color::DarkGray };
        let comp_spans: Vec<Span> = Complexity::ALL
            .iter()
            .map(|&lvl| {
                let is_selected = lvl == self.config.complexity;
                let text = format!(" {} {} ", if is_selected { "●" } else { "○" }, lvl.as_str());
                if is_selected {
                    Span::styled(text, Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(text, Style::default().fg(Color::DarkGray))
                }
            })
            .collect();

        let comp_box = Paragraph::new(vec![Line::from(comp_spans)]).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " {} Output Complexity & Busyness ",
                    if is_comp_active { "▶" } else { " " }
                ))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(comp_border_color)),
        );
        f.render_widget(comp_box, left_chunks[2]);
    }

    fn draw_right_column(&self, f: &mut Frame, area: Rect) {
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Project Name
                Constraint::Length(3), // Framework
                Constraint::Length(3), // Duration
                Constraint::Length(5), // Flags / Toggles
                Constraint::Length(4), // Action Buttons
            ])
            .split(area);

        // Project Name
        let is_project_active = self.current_field == FormField::ProjectName;
        let project_border = if is_project_active { Color::Yellow } else { Color::DarkGray };
        let project_display = if is_project_active && self.is_editing {
            format!("{}█", self.edit_buffer)
        } else {
            self.config.project_name.clone()
        };
        let project_p = Paragraph::new(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(project_display, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            if is_project_active && !self.is_editing {
                Span::styled("  (Press Enter to edit)", Style::default().fg(Color::DarkGray))
            } else {
                Span::raw("")
            },
        ])).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} Project Identifier ", if is_project_active { "▶" } else { " " }))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(project_border)),
        );
        f.render_widget(project_p, right_chunks[0]);

        // Framework
        let is_fw_active = self.current_field == FormField::Framework;
        let fw_border = if is_fw_active { Color::Yellow } else { Color::DarkGray };
        let fw_display = if is_fw_active && self.is_editing {
            format!("{}█", self.edit_buffer)
        } else if self.config.framework.is_empty() {
            "(Auto / Default)".to_string()
        } else {
            self.config.framework.clone()
        };
        let fw_p = Paragraph::new(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                fw_display,
                if self.config.framework.is_empty() {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                },
            ),
            if is_fw_active && !self.is_editing {
                Span::styled("  (Press Enter to edit)", Style::default().fg(Color::DarkGray))
            } else {
                Span::raw("")
            },
        ])).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} Framework / Tech Stack ", if is_fw_active { "▶" } else { " " }))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(fw_border)),
        );
        f.render_widget(fw_p, right_chunks[1]);

        // Duration
        let is_dur_active = self.current_field == FormField::Duration;
        let dur_border = if is_dur_active { Color::Yellow } else { Color::DarkGray };
        let dur_text = if is_dur_active && self.is_editing {
            format!("{} sec █", self.edit_buffer)
        } else if self.duration == 0 {
            "Infinite (Run until Ctrl+C)".to_string()
        } else {
            format!("{} seconds", self.duration)
        };
        let dur_p = Paragraph::new(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(dur_text, Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD)),
            Span::styled("  (←/→ ±5s, Enter to type)", Style::default().fg(Color::DarkGray)),
        ])).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} Simulation Duration ", if is_dur_active { "▶" } else { " " }))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(dur_border)),
        );
        f.render_widget(dur_p, right_chunks[2]);

        // Flags / Toggles
        let flag_lines = vec![
            self.render_toggle_line(
                FormField::Alerts,
                "Critical System Alerts",
                self.config.alerts_enabled,
                "Inject unexpected simulated crashes/warnings",
            ),
            self.render_toggle_line(
                FormField::Minimal,
                "Minimal Output Mode",
                self.config.minimal_output,
                "Less ANSI colors for subtler background look",
            ),
            self.render_toggle_line(
                FormField::Team,
                "Team Collaboration",
                self.config.team_activity,
                "Simulate git commits & PR reviews from teammates",
            ),
        ];
        let flags_box = Paragraph::new(flag_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Extra Simulation Options (Space / Enter to toggle) ")
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(flags_box, right_chunks[3]);

        // Action Buttons
        let btn_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(right_chunks[4]);

        let is_start_active = self.current_field == FormField::StartButton;
        let start_btn = Paragraph::new(Line::from(vec![
            Span::styled(
                " ▶ START SIMULATION ",
                if is_start_active {
                    Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                },
            ),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if is_start_active {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                }),
        );
        f.render_widget(start_btn, btn_chunks[0]);

        let is_save_active = self.current_field == FormField::SaveButton;
        let save_btn = Paragraph::new(Line::from(vec![
            Span::styled(
                " 💾 SAVE CONFIG (Ctrl+S) ",
                if is_save_active {
                    Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                },
            ),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if is_save_active {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Blue)
                }),
        );
        f.render_widget(save_btn, btn_chunks[1]);
    }

    fn render_toggle_line<'a>(
        &self,
        field: FormField,
        label: &'a str,
        value: bool,
        desc: &'a str,
    ) -> Line<'a> {
        let is_active = self.current_field == field;
        let cursor = if is_active { "▶" } else { " " };
        let box_symbol = if value { "[x]" } else { "[ ]" };
        let box_style = if value {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        Line::from(vec![
            Span::styled(format!(" {} ", cursor), if is_active { Style::default().fg(Color::Yellow) } else { Style::default() }),
            Span::styled(box_symbol, box_style),
            Span::styled(format!(" {:<24}", label), if is_active { Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::White) }),
            Span::styled(format!(" - {}", desc), Style::default().fg(Color::DarkGray)),
        ])
    }
}
