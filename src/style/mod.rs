//! Style registry.
//!
//! Every fake CLI lives in its own module and provides two things: a [`Skin`]
//! (glyphs, colors, copy) and an implementation of [`Renderer`] (the few layout
//! decisions that genuinely differ between the real tools).
//!
//! Adding a style is three steps:
//!
//! 1. add a variant to [`Style`] (plus `ALL`, `as_str`, `parse`, `renderer`),
//! 2. add `src/style/<name>.rs` with a skin and a `Renderer` impl,
//! 3. declare the module below.
//!
//! Every renderer method has a default, so a new style can start as a five-line
//! file that only names its skin and grow one override at a time.

use crate::ansi::{self, SpinnerKind, Ui};
use crate::content;

pub mod antigravity;
pub mod claude;
pub mod codex;
pub mod opencode;
pub mod pi;

/// A visual "style": the fake AI CLI we are impersonating.
///
/// This is the single user-facing selector (`--style`). `Classic` is not an AI
/// CLI; it has its own renderer in `classic.rs` and never asks for a skin.
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Style {
    #[default]
    Claude,
    Codex,
    Opencode,
    Pi,
    Antigravity,
    Classic,
}

impl Style {
    #[allow(dead_code)] // used by the TUI
    pub const ALL: [Style; 6] = [
        Style::Claude,
        Style::Codex,
        Style::Opencode,
        Style::Pi,
        Style::Antigravity,
        Style::Classic,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Style::Claude => "claude",
            Style::Classic => "classic",
            Style::Codex => "codex",
            Style::Opencode => "opencode",
            Style::Pi => "pi",
            Style::Antigravity => "antigravity",
        }
    }

    pub fn parse(input: &str) -> Option<Style> {
        match input.trim().to_ascii_lowercase().as_str() {
            "claude" | "claude-code" | "cc" => Some(Style::Claude),
            "classic" | "legacy" | "stakeholder" | "system" => Some(Style::Classic),
            "codex" | "openai" => Some(Style::Codex),
            "opencode" | "oc" => Some(Style::Opencode),
            "pi" | "pi-agent" => Some(Style::Pi),
            "antigravity" | "antigravity-cli" | "agy" => Some(Style::Antigravity),
            _ => None,
        }
    }

    pub fn skin(self) -> &'static Skin {
        self.renderer().skin()
    }

    /// Static dispatch target for this style. `Classic` borrows the Claude skin
    /// only because the type demands one; its renderer never calls it.
    pub fn renderer(self) -> &'static dyn Renderer {
        match self {
            Style::Claude | Style::Classic => &claude::CLAUDE,
            Style::Codex => &codex::CODEX,
            Style::Opencode => &opencode::OPENCODE,
            Style::Pi => &pi::PI,
            Style::Antigravity => &antigravity::ANTIGRAVITY,
        }
    }
}

/// Everything that is pure data about a style. Kept separate from the renderer
/// so the TUI can preview a style without running a session.
pub struct Skin {
    pub id: &'static str,
    pub title: &'static str,
    pub model: &'static str,
    /// 24-bit SGR prefix (dark theme; `Ui` maps to light theme automatically).
    pub accent: &'static str,
    #[allow(dead_code)] // used by the TUI accent preview (dark theme)
    pub rgb: (u8, u8, u8),
    #[allow(dead_code)] // used by the TUI accent preview (light theme)
    pub rgb_light: (u8, u8, u8),
    pub bullet: &'static str,
    pub result: &'static str,
    pub prompt: &'static str,
    #[allow(dead_code)]
    pub footer: &'static str,
    /// Rotating status hints shown after each turn. `footer` is the first one.
    pub hints: &'static [&'static str],
    pub thinking: &'static str,
    /// Provider label shown by opencode-style footers. Empty when unused.
    pub provider: &'static str,
    /// Startup command hints (Codex prints these under its header).
    pub commands: &'static [&'static str],
    pub boxed: bool,
    pub hint: &'static str,
}

impl Skin {
    #[allow(dead_code)]
    pub fn rgb_for(&self, theme: crate::types::Theme) -> (u8, u8, u8) {
        match theme {
            crate::types::Theme::Dark => self.rgb,
            crate::types::Theme::Light => self.rgb_light,
        }
    }
}

/// The per-style layout hooks.
///
/// The transcript engine owns pacing, content decks and the turn structure; a
/// renderer only decides how a prompt, a tool call or a footer looks. All
/// methods have defaults that produce the generic/Claude layout, so overriding
/// just `skin()` is enough for a new style.
pub trait Renderer: Sync {
    fn skin(&self) -> &'static Skin;

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Claude
    }

    fn welcome(&self, ui: &mut Ui) {
        generic_welcome(ui, self.skin());
    }

    fn prompt(&self, ui: &mut Ui, text: &str) {
        let skin = self.skin();
        let rule = "\u{2500}".repeat(ui.width);
        ui.blank();
        ui.parts(&[(ansi::GRAY, &rule)]);
        let full = format!("{}{}", skin.prompt, text);
        let lines = ansi::wrap(&full, ui.width.saturating_sub(2));
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                let (glyph, rest) = split_at_chars(line, skin.prompt.chars().count());
                ui.write_styled(skin.accent, glyph);
                ui.type_text(rest, 7);
            } else {
                ui.write_raw("  ");
                ui.type_text(line, 5);
            }
            ui.nl();
        }
        ui.parts(&[(ansi::GRAY, &rule)]);
        ui.blank();
    }

    fn assistant(&self, ui: &mut Ui, text: &str) {
        let skin = self.skin();
        let lines = ansi::wrap(text, ui.width.saturating_sub(4));
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                ui.write_styled(skin.accent, skin.bullet);
                ui.write_raw(" ");
                ui.type_text(line, 3);
                ui.nl();
            } else {
                ui.parts(&[(ansi::GRAY, "  "), ("", line)]);
            }
        }
    }

    fn search(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("pattern: \"{query}\", path: \"src\"");
        let out = format!("{found} (ctrl+o to expand)");
        self.tool(ui, "Search", &args, &out);
    }

    fn grep(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("pattern: \"{query}\"");
        let out = format!("Found {found} (ctrl+o to expand)");
        self.tool(ui, "Grep", &args, &out);
    }

    fn read(&self, ui: &mut Ui, file: &str, _start: u64, len: u64) {
        let out = format!("Read {len} lines (ctrl+o to expand)");
        self.tool(ui, "Read", file, &out);
    }

    fn update(&self, ui: &mut Ui, file: &str, adds: usize, rems: usize) {
        self.tool(ui, "Update", file, "");
        let text = format!(
            "Updated {file} with {adds} {} and {rems} {}",
            plural(adds, "addition", "additions"),
            plural(rems, "removal", "removals")
        );
        self.output(ui, true, &text, ansi::DIM);
    }

    fn patch(&self, ui: &mut Ui, diff: &[&str], start: u64) {
        patch_indented(ui, "     ", diff, start);
    }

    fn bash(&self, ui: &mut Ui, cmd: &str, out: &[&str]) {
        self.tool(ui, "Bash", cmd, "");
        for (i, line) in out.iter().enumerate() {
            self.output(ui, i == 0, line, ansi::DIM);
            ui.pause(24);
        }
    }

    /// One tool call: `Name(args)` plus an optional single result line.
    fn tool(&self, ui: &mut Ui, name: &str, args: &str, out: &str) {
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::GRAY, " "),
            (ansi::BOLD, name),
            (ansi::GRAY, "("),
            ("", args),
            (ansi::GRAY, ")"),
        ]);
        if !out.is_empty() {
            self.output(ui, true, out, ansi::DIM);
        }
    }

    /// One continuation line of tool output. `first` selects the tree glyph.
    fn output(&self, ui: &mut Ui, first: bool, text: &str, color: &'static str) {
        let skin = self.skin();
        if first {
            ui.parts(&[
                (ansi::GRAY, "  "),
                (ansi::GRAY, skin.result),
                (ansi::GRAY, "  "),
                (color, text),
            ]);
        } else {
            ui.parts(&[(ansi::GRAY, "     "), (color, text)]);
        }
    }

    fn edit(&self, ui: &mut Ui, file: &str, note: &str) {
        self.tool(ui, "Edit", file, note);
    }

    fn todo(&self, ui: &mut Ui, items: &[&str]) {
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::GRAY, " "),
            (ansi::BOLD, "Update Todos"),
        ]);
        for (i, item) in items.iter().enumerate() {
            let pending = i + 1 == items.len();
            let (code, glyph) = if pending {
                (ansi::DIM, "\u{2610} ")
            } else {
                (ansi::GREEN, "\u{2612} ")
            };
            let lead = if i == 0 { "  \u{23bf}  " } else { "     " };
            let text_style = if pending { "" } else { ansi::DIM };
            ui.parts(&[(ansi::GRAY, lead), (code, glyph), (text_style, item)]);
        }
    }

    fn footer(&self, ui: &mut Ui, secs: u64, tokens: u32) {
        let cost = tokens as f64 * 0.000009;
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, "\u{273b}"),
            (
                ansi::DIM,
                &format!(
                    " Worked for {} \u{00b7} {} tokens \u{00b7} ${:.2}",
                    human_secs(secs),
                    ansi::tokens_label(tokens),
                    cost
                ),
            ),
        ]);
    }

    /// Whether the engine should print a rotating hint after the footer.
    fn hints(&self) -> bool {
        true
    }

    fn ended(&self, ui: &mut Ui) {
        let skin = self.skin();
        ui.parts(&[
            (ansi::DIM, "  "),
            (ansi::DIM, skin.bullet),
            (
                ansi::DIM,
                &format!(" {} \u{00b7} {}", ended_note(), skin.id),
            ),
        ]);
    }

    /// A fake lint failure, its fix and the clean re-run. Built from the hooks
    /// above, so overriding `tool`/`output` is enough for a new style.
    fn lint_error(&self, ui: &mut Ui) {
        let cmd = "cargo clippy --all-targets -- -D warnings";
        self.tool(ui, "Bash", cmd, "");
        for (i, line) in content::LINT_ERROR.iter().enumerate() {
            self.output(ui, i == 0, line, ansi::RED);
            ui.pause(20);
        }
        ui.pause(420);
        self.edit(ui, "src/api/handlers.rs", "Removed the duplicated branch");
        self.patch(ui, content::LINT_FIX, 54);
        ui.blank();
        self.tool(ui, "Bash", cmd, "0 warnings, 0 errors");
    }
}

/// Renders a coloured unified diff with a custom left prefix. Shared by every style.
pub(super) fn patch_indented(ui: &mut Ui, prefix: &str, diff: &[&str], start: u64) {
    let mut old = start;
    let mut new = start;
    for raw in diff {
        let raw = *raw;
        match raw.chars().next() {
            Some('+') => {
                ui.parts(&[
                    (ansi::GRAY, prefix),
                    (ansi::DIM, &format!("{new:>4} ")),
                    (ansi::GREEN, "+ "),
                    (ansi::GREEN, &raw[1..]),
                ]);
                new += 1;
            }
            Some('-') => {
                ui.parts(&[
                    (ansi::GRAY, prefix),
                    (ansi::DIM, &format!("{old:>4} ")),
                    (ansi::RED, "- "),
                    (ansi::RED, &raw[1..]),
                ]);
                old += 1;
            }
            _ => {
                let body = raw.strip_prefix(' ').unwrap_or(raw);
                ui.parts(&[
                    (ansi::GRAY, prefix),
                    (ansi::DIM, &format!("{new:>4} ")),
                    ("", "  "),
                    (ansi::DIM, body),
                ]);
                old += 1;
                new += 1;
            }
        }
        ui.pause(16);
    }
}

pub(crate) fn human_secs(secs: u64) -> String {
    if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    }
}

pub(super) fn ended_note() -> &'static str {
    "session ended \u{00b7} 0 files were actually modified"
}

pub(super) fn plural(n: usize, one: &'static str, many: &'static str) -> &'static str {
    if n == 1 {
        one
    } else {
        many
    }
}

pub(super) fn box_top(ui: &mut Ui, w: usize) {
    ui.parts(&[(
        ansi::GRAY,
        &format!("\u{256d}{}\u{256e}", "\u{2500}".repeat(w - 2)),
    )]);
}

pub(super) fn box_top_titled(
    ui: &mut Ui,
    w: usize,
    lead_color: &'static str,
    lead: &str,
    title_color: &'static str,
    title: &str,
) {
    let inner = w.saturating_sub(6);
    let lead_len = lead.chars().count();
    let max_title = inner.saturating_sub(lead_len + 1);
    let (title_shown, _) = split_at_chars(title, max_title);
    let used = lead_len + title_shown.chars().count() + 1;
    let dashes = "\u{2500}".repeat(inner.saturating_sub(used));
    ui.parts(&[
        (ansi::GRAY, "\u{256d}\u{2500}\u{2500}\u{2500} "),
        (lead_color, lead),
        (title_color, title_shown),
        (ansi::GRAY, " "),
        (ansi::GRAY, &dashes),
        (ansi::GRAY, "\u{256e}"),
    ]);
}

pub(super) fn box_bottom(ui: &mut Ui, w: usize) {
    ui.parts(&[(
        ansi::GRAY,
        &format!("\u{2570}{}\u{256f}", "\u{2500}".repeat(w - 2)),
    )]);
}

pub(super) fn box_row(ui: &mut Ui, w: usize, parts: &[(&'static str, &str)]) {
    let avail = w.saturating_sub(4);
    let mut used = 0;
    let mut line: Vec<(&'static str, &str)> = Vec::with_capacity(parts.len() + 3);
    line.push((ansi::GRAY, "\u{2502} "));
    for &(code, text) in parts {
        let room = avail.saturating_sub(used);
        let len = text.chars().count();
        if len <= room {
            line.push((code, text));
            used += len;
            continue;
        }
        // Truncate instead of overflowing a narrow terminal.
        if room > 1 {
            let (head, _) = split_at_chars(text, room - 1);
            line.push((code, head));
            line.push((code, "\u{2026}"));
        }
        used = avail;
        break;
    }
    let pad = " ".repeat(avail.saturating_sub(used));
    line.push((ansi::GRAY, &pad));
    line.push((ansi::GRAY, " \u{2502}"));
    ui.parts(&line);
}

fn box_split_row(
    ui: &mut Ui,
    lw: usize,
    rw: usize,
    left_color: &'static str,
    left_centered: &str,
    right_color: &'static str,
    right_text: &str,
) {
    let (l_shown, _) = split_at_chars(left_centered, lw);
    let l_len = l_shown.chars().count();
    let l_left_pad = " ".repeat(lw.saturating_sub(l_len) / 2);
    let l_right_pad = " ".repeat(lw.saturating_sub(l_len + l_left_pad.len()));

    let (r_shown, _) = split_at_chars(right_text, rw);
    let r_len = r_shown.chars().count();
    let r_pad = " ".repeat(rw.saturating_sub(r_len));

    ui.parts(&[
        (ansi::GRAY, "\u{2502} "),
        ("", &l_left_pad),
        (left_color, l_shown),
        ("", &l_right_pad),
        (ansi::GRAY, " \u{2502} "),
        (right_color, r_shown),
        ("", &r_pad),
        (ansi::GRAY, " \u{2502}"),
    ]);
}

pub(super) fn split_at_chars(text: &str, at: usize) -> (&str, &str) {
    match text.char_indices().nth(at) {
        Some((idx, _)) => (&text[..idx], &text[idx..]),
        None => (text, ""),
    }
}

fn generic_welcome(ui: &mut Ui, skin: &Skin) {
    let cwd = format!("~/code/{}", content::PROJECT);
    if skin.boxed {
        let w = ui.width.clamp(40, 74);
        box_top_titled(ui, w, skin.accent, "\u{273b} ", ansi::BOLD, skin.title);
        if w >= 68 {
            let lw = 30;
            let rw = w - lw - 7;
            let div = "\u{2500}".repeat(rw);
            box_split_row(
                ui,
                lw,
                rw,
                ansi::BOLD,
                "Welcome back!",
                skin.accent,
                "Tips for getting started",
            );
            box_split_row(
                ui,
                lw,
                rw,
                "",
                "",
                ansi::DIM,
                "Run /init to create a CLAUDE.md",
            );
            box_split_row(
                ui,
                lw,
                rw,
                skin.accent,
                "\u{2590}\u{259b}\u{2588}\u{2588}\u{2588}\u{259c}\u{258c}",
                ansi::DIM,
                "Use @ to mention files in chat",
            );
            box_split_row(
                ui,
                lw,
                rw,
                skin.accent,
                "\u{259d}\u{259c}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259b}\u{2598}",
                ansi::GRAY,
                &div,
            );
            box_split_row(
                ui,
                lw,
                rw,
                skin.accent,
                "\u{2598}\u{2598} \u{259d}\u{259d}",
                skin.accent,
                "Recent activity",
            );
            box_split_row(ui, lw, rw, "", "", ansi::DIM, "No recent activity");
            box_split_row(
                ui,
                lw,
                rw,
                ansi::DIM,
                skin.model,
                ansi::DIM,
                "/resume for previous sessions",
            );
            box_split_row(ui, lw, rw, ansi::DIM, &cwd, ansi::DIM, skin.hint);
        } else {
            box_row(ui, w, &[(ansi::BOLD, "  Welcome back!")]);
            box_row(
                ui,
                w,
                &[(
                    skin.accent,
                    "    \u{2590}\u{259b}\u{2588}\u{2588}\u{2588}\u{259c}\u{258c}",
                )],
            );
            box_row(
                ui,
                w,
                &[(
                    skin.accent,
                    "   \u{259d}\u{259c}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259b}\u{2598}",
                )],
            );
            box_row(
                ui,
                w,
                &[(skin.accent, "     \u{2598}\u{2598} \u{259d}\u{259d}")],
            );
            box_row(ui, w, &[(ansi::DIM, "  "), (ansi::DIM, skin.model)]);
            box_row(ui, w, &[(ansi::DIM, "  "), (ansi::DIM, &cwd)]);
            box_row(ui, w, &[(ansi::DIM, "  "), (ansi::DIM, skin.hint)]);
        }
        box_bottom(ui, w);
        ui.blank();
    } else {
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::BOLD, " "),
            (ansi::BOLD, skin.title),
            (ansi::DIM, &format!(" \u{00b7} {}", skin.model)),
        ]);
        ui.parts(&[(ansi::DIM, "  cwd: "), ("", &cwd)]);
        ui.parts(&[(ansi::DIM, "  "), (ansi::DIM, skin.hint)]);
        ui.blank();
    }
}
