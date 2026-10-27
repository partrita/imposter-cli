//! Pi (`earendil-works/pi` — `@earendil-works/pi-coding-agent`).
//!
//! Layout notes taken from `packages/coding-agent/src/modes/interactive/`:
//! - Theme (`dark.json` / `light.json`): soft teal accent `#8abeb7`, full-width
//!   tinted boxes for `userMessageBg` (`#343541`), `toolSuccessBg` (`#283228`),
//!   `toolPendingBg` (`#282832`), and `toolErrorBg` (`#3c2828`).
//! - `BuiltInHeaderComponent`: `▀█▀ / █▀█` block logo, single-column hotkey
//!   reference (`escape to interrupt`, `ctrl+c to clear`, `ctrl+o to expand tool
//!   output`, `ctrl+t to expand thinking`, etc.), and `[context]` `AGENTS.md`.
//! - `UserMessageComponent`: full-width `Box(1, 1, userMessageBg)` with top and
//!   bottom tinted padding lines.
//! - `AssistantMessageComponent`: `Thinking... (ctrl+t to expand)` above the
//!   assistant response text.
//! - `ToolExecutionComponent`: full-width `Box(1, 1, toolSuccessBg)` for `read`,
//!   `edit`, `$ <cmd>`, `find`, and `grep`.
//! - `CustomEditor` + `FooterComponent` (`footer.ts`): accent editor rule +
//!   `~/path (branch)` + `↑in ↓out Rcache Wcache $cost ctx% (auto)  (provider) model • thinking`.

use super::{ended_note, split_at_chars, Renderer, Skin};
use crate::ansi::{self, SpinnerKind, Ui};

pub struct Pi;

pub static PI: Pi = Pi;

pub static SKIN: Skin = Skin {
    id: "pi",
    title: "pi v0.67.68",
    model: "(anthropic) claude-sonnet-4-5 \u{2022} high",
    accent: "\x1b[38;2;138;190;183m",
    rgb: (138, 190, 183),
    rgb_light: (20, 118, 110),
    bullet: "\u{2580}\u{2588}\u{2580}",
    result: "...",
    prompt: "",
    footer: "\u{2191}1.2k \u{2193}340 R2.8k W410 $0.042 24.1%/200k (auto)",
    hints: &[
        "\u{2191}1.2k \u{2193}340 R2.8k W410 $0.042 24.1%/200k (auto)",
        "/compact to summarize context you never had",
        "ctrl+o to expand tool output \u{00b7} ctrl+t to expand thinking",
        "escape twice for session tree \u{00b7} shift+tab thinking level",
    ],
    thinking: "Working",
    provider: "anthropic",
    commands: &[],
    hint: "escape to interrupt \u{00b7} ctrl+o to expand \u{00b7} ctrl+t thinking",
    boxed: false,
};

const HOTKEYS: &[(&str, &str)] = &[
    ("escape", "to interrupt"),
    ("ctrl+c", "to clear"),
    ("ctrl+c twice", "to exit"),
    ("ctrl+d", "to exit (empty)"),
    ("shift+tab", "to cycle thinking level"),
    ("ctrl+p", "to cycle models"),
    ("ctrl+o", "to expand tool output"),
    ("ctrl+t", "to expand thinking"),
    ("/", "for commands"),
    ("!", "to run bash"),
];

impl Renderer for Pi {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Pi
    }

    fn welcome(&self, ui: &mut Ui) {
        let skin = self.skin();
        ui.blank();
        ui.parts(&[
            (skin.accent, "   \u{2580}\u{2588}\u{2580}  "),
            (ansi::BOLD, skin.title),
        ]);
        ui.parts(&[
            (skin.accent, "   \u{2588}\u{2580}\u{2588}  "),
            (ansi::DIM, "https://pi.dev"),
        ]);
        ui.blank();
        for &(key, desc) in HOTKEYS {
            ui.parts(&[(ansi::DIM, &format!("  {key:<16}")), (ansi::DIM, desc)]);
        }
        ui.blank();
        ui.parts(&[(ansi::DIM, "  [context]")]);
        ui.parts(&[(
            ansi::DIM,
            &format!("    ~/code/{}/AGENTS.md", crate::content::PROJECT),
        )]);
        ui.blank();
    }

    fn prompt(&self, ui: &mut Ui, text: &str) {
        let w = ui.width;
        let inner = w.saturating_sub(4);
        ui.blank();
        box_blank(ui, ansi::PI_USER_BG);
        for line in ansi::wrap(text, inner) {
            let used = 2 + line.chars().count();
            let pad = " ".repeat(w.saturating_sub(used));
            ui.type_on_bg(ansi::PI_USER_BG, "  ", &line, &pad, 7);
        }
        box_blank(ui, ansi::PI_USER_BG);
        ui.blank();
    }

    fn assistant(&self, ui: &mut Ui, text: &str) {
        ui.parts(&[(ansi::DIM, "  Thinking... (ctrl+t to expand)")]);
        ui.blank();
        for line in ansi::wrap(text, ui.width.saturating_sub(4)) {
            ui.write_raw("  ");
            ui.type_text(&line, 3);
            ui.nl();
        }
        ui.blank();
    }

    fn search(&self, ui: &mut Ui, query: &str, found: &str) {
        let title = format!("find \"*{query}*\" in src/");
        let detail = format!("... ({found}, ctrl+o to expand)");
        tool_box(ui, &title, &[&detail], false);
    }

    fn grep(&self, ui: &mut Ui, query: &str, found: &str) {
        let title = format!("grep \"{query}\" in src/");
        let detail = format!("... ({found}, ctrl+o to expand)");
        tool_box(ui, &title, &[&detail], false);
    }

    fn read(&self, ui: &mut Ui, file: &str, start: u64, len: u64) {
        let end = start + len;
        let title = format!(
            "read ~/code/{}/{file}:{start}-{end}",
            crate::content::PROJECT
        );
        let detail = format!("... ({len} lines, ctrl+o to expand)");
        tool_box(ui, &title, &[&detail], false);
    }

    fn update(&self, ui: &mut Ui, file: &str, adds: usize, rems: usize) {
        let title = format!(
            "edit ~/code/{}/{file} (+{adds} -{rems})",
            crate::content::PROJECT
        );
        box_blank(ui, ansi::PI_TOOL_OK_BG);
        box_line(ui, ansi::PI_TOOL_OK_TITLE, ansi::PI_TOOL_OK_BG, &title);
        box_blank(ui, ansi::PI_TOOL_OK_BG);
    }

    fn patch(&self, ui: &mut Ui, diff: &[&str], start: u64) {
        let mut old = start;
        let mut new = start;
        for raw in diff {
            let raw = *raw;
            match raw.chars().next() {
                Some('+') => {
                    let row = format!("+{new:<4} {}", &raw[1..]);
                    box_line(ui, ansi::PI_TOOL_OK_ADD, ansi::PI_TOOL_OK_BG, &row);
                    new += 1;
                }
                Some('-') => {
                    let row = format!("-{old:<4} {}", &raw[1..]);
                    box_line(ui, ansi::PI_TOOL_OK_DEL, ansi::PI_TOOL_OK_BG, &row);
                    old += 1;
                }
                _ => {
                    let body = raw.strip_prefix(' ').unwrap_or(raw);
                    let row = format!(" {new:<4} {body}");
                    box_line(ui, ansi::PI_TOOL_OK_DIM, ansi::PI_TOOL_OK_BG, &row);
                    old += 1;
                    new += 1;
                }
            }
            ui.pause(16);
        }
        box_blank(ui, ansi::PI_TOOL_OK_BG);
        ui.blank();
    }

    fn edit(&self, ui: &mut Ui, file: &str, note: &str) {
        let title = if note.is_empty() {
            format!("edit ~/code/{}/{file}", crate::content::PROJECT)
        } else {
            format!("edit ~/code/{}/{file} ({note})", crate::content::PROJECT)
        };
        box_blank(ui, ansi::PI_TOOL_OK_BG);
        box_line(ui, ansi::PI_TOOL_OK_TITLE, ansi::PI_TOOL_OK_BG, &title);
        box_blank(ui, ansi::PI_TOOL_OK_BG);
    }

    fn bash(&self, ui: &mut Ui, cmd: &str, out: &[&str]) {
        let title = format!("$ {cmd}");
        tool_box(ui, &title, out, false);
    }

    fn tool(&self, ui: &mut Ui, name: &str, args: &str, out: &str) {
        let title = if name == "Bash" {
            format!("$ {args}")
        } else {
            format!("{} {args}", name.to_ascii_lowercase())
        };
        let rows: &[&str] = if out.is_empty() { &[] } else { &[out] };
        tool_box(ui, &title, rows, false);
    }

    fn output(&self, ui: &mut Ui, _first: bool, text: &str, _color: &'static str) {
        box_line(ui, ansi::PI_TOOL_OK_DIM, ansi::PI_TOOL_OK_BG, text);
    }

    fn lint_error(&self, ui: &mut Ui) {
        let cmd = "$ cargo clippy --all-targets -- -D warnings";
        tool_box(ui, cmd, crate::content::LINT_ERROR, true);
        ui.pause(420);
        self.edit(ui, "src/api/handlers.rs", "Removed duplicated branch");
        self.patch(ui, crate::content::LINT_FIX, 54);
        tool_box(ui, cmd, &["0 warnings, 0 errors"], false);
    }

    fn todo(&self, ui: &mut Ui, items: &[&str]) {
        box_blank(ui, ansi::PI_TOOL_PEND_BG);
        box_line(ui, ansi::PI_TOOL_PEND_TITLE, ansi::PI_TOOL_PEND_BG, "todo");
        box_blank(ui, ansi::PI_TOOL_PEND_BG);
        for (i, item) in items.iter().enumerate() {
            let pending = i + 1 == items.len();
            let prefix = if pending { "[ ] " } else { "[x] " };
            let style = if pending {
                ansi::PI_TOOL_PEND_BG
            } else {
                ansi::PI_TOOL_PEND_DIM
            };
            let row = format!("{prefix}{item}");
            box_line(ui, style, ansi::PI_TOOL_PEND_BG, &row);
        }
        box_blank(ui, ansi::PI_TOOL_PEND_BG);
        ui.blank();
    }

    fn footer(&self, ui: &mut Ui, _secs: u64, tokens: u32) {
        let skin = self.skin();
        let inp = tokens;
        let out = tokens / 5 + 120;
        let read = tokens * 2 + 1_400;
        let written = tokens / 3 + 260;
        let cost = tokens as f64 * 0.000009 + 0.02;
        let pct = (tokens as f64 / 2000.0).clamp(4.2, 84.0);
        let rule = "\u{2500}".repeat(ui.width);
        ui.parts(&[(skin.accent, &rule)]);
        ui.parts(&[(
            ansi::DIM,
            &format!("~/code/{} (main)", crate::content::PROJECT),
        )]);
        let left = format!(
            "\u{2191}{} \u{2193}{} R{} W{} ${cost:.3} {pct:.1}%/200k (auto)",
            short(inp),
            short(out),
            short(read),
            short(written)
        );
        let right = if ui.width >= left.chars().count() + skin.model.chars().count() + 4 {
            skin.model
        } else {
            ""
        };
        let pad = ui
            .width
            .saturating_sub(left.chars().count() + right.chars().count())
            .max(1);
        ui.parts(&[
            (ansi::DIM, &left),
            (ansi::GRAY, &" ".repeat(pad)),
            (ansi::DIM, right),
        ]);
    }

    fn hints(&self) -> bool {
        false
    }

    fn ended(&self, ui: &mut Ui) {
        ui.parts(&[(ansi::GRAY, "  "), (ansi::DIM, ended_note())]);
    }
}

fn box_blank(ui: &mut Ui, bg: &'static str) {
    let spaces = " ".repeat(ui.width);
    ui.parts(&[(bg, &spaces)]);
}

fn box_line(ui: &mut Ui, fg_bg: &'static str, pad_bg: &'static str, text: &str) {
    let w = ui.width;
    let avail = w.saturating_sub(4);
    let (shown, _) = split_at_chars(text, avail);
    let used = 2 + shown.chars().count();
    let pad = " ".repeat(w.saturating_sub(used));
    ui.parts(&[(pad_bg, "  "), (fg_bg, shown), (pad_bg, &pad)]);
}

fn tool_box(ui: &mut Ui, title: &str, rows: &[&str], error: bool) {
    let (bg, title_style, row_style) = if error {
        (
            ansi::PI_TOOL_ERR_BG,
            ansi::PI_TOOL_ERR_TITLE,
            ansi::PI_TOOL_ERR_BG,
        )
    } else {
        (
            ansi::PI_TOOL_OK_BG,
            ansi::PI_TOOL_OK_TITLE,
            ansi::PI_TOOL_OK_DIM,
        )
    };
    box_blank(ui, bg);
    box_line(ui, title_style, bg, title);
    if !rows.is_empty() {
        box_blank(ui, bg);
        for line in rows {
            box_line(ui, row_style, bg, line);
            ui.pause(18);
        }
    }
    box_blank(ui, bg);
    ui.blank();
}

/// `1234` -> `1.2k`, matching Pi's `formatTokens` in `footer.ts`.
fn short(n: u32) -> String {
    if n >= 10_000 {
        format!("{}k", (n + 500) / 1_000)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1000.0)
    } else {
        n.to_string()
    }
}
