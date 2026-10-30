//! OpenCode TUI (`anomalyco/opencode` — `@opentui/core` + SolidJS session view).
//!
//! Layout notes taken from `packages/opencode/src/cli/cmd/tui/routes/session/`
//! and the reference screenshot (`opencode-indexnow-qwen3.5-27bq3_b.jpg`):
//! - Every major container (`Header`, `UserMessage`, `BlockTool` for `Bash`,
//!   `Edit`, and `TodoWrite`, plus the bottom `Prompt` composer) uses a
//!   full-width `backgroundPanel` (`#1a1b26` dark / `#e6e9ef` light) with a
//!   `SplitBorder` left vertical bar (`┃`).
//! - `Header` (`header.tsx`): `SplitBorder` + `backgroundPanel` showing `# <title>`
//!   on the left and `<tokens>  <pct>% ($<cost>)` on the right.
//! - `UserMessage`: `SplitBorder` left bar in the active agent's accent color
//!   (`#569cff`) on `backgroundPanel`.
//! - `ReasoningPart`: muted `┃  Thinking: ...` block above tool calls.
//! - `InlineTool`: `   → Read ... [offset=X, limit=Y]`, `   ✱ Glob ...`, `   ✱ Grep ...`.
//! - `BlockTool`: `SplitBorder` + `backgroundPanel` for `← Edit` diffs, `# Shell`
//!   (`$ <cmd>`), and `# Todo` (`[✓]` / `[•]`).
//! - Turn meta badge: `   ▣ Build · qwen3.5-27b · Xs`, followed by the prompt
//!   composer panel, `ctrl+t variants  tab agents  ctrl+p commands`, and the
//!   `footer.tsx` bar (`~/code/... (main)` + `• LSP  • MCP  1.2.14`).

use super::{ended_note, human_secs, split_at_chars, Renderer, Skin};
use crate::ansi::{self, SpinnerKind, Ui};

pub struct Opencode;

pub static OPENCODE: Opencode = Opencode;

pub static SKIN: Skin = Skin {
    id: "opencode",
    title: "opencode v1.2.14",
    model: "qwen3.5-27b",
    accent: "\x1b[38;2;86;156;255m",
    rgb: (86, 156, 255),
    rgb_light: (37, 99, 235),
    bullet: "\u{25a3}",
    result: "\u{2192}",
    prompt: "\u{2503}  ",
    footer: "ctrl+t variants  tab agents  ctrl+p commands",
    hints: &[
        "ctrl+t variants  tab agents  ctrl+p commands",
        "/undo reverts changes that were never made",
        "tab to switch between Build and Plan agents",
        "ctrl+p for the command palette \u{00b7} ctrl+x l for sessions",
    ],
    thinking: "Thinking",
    provider: "OpenRouter",
    commands: &[],
    hint: "Build \u{00b7} qwen3.5-27b \u{00b7} OpenRouter",
    boxed: false,
};

const KEYBINDS: &str = "ctrl+t variants  tab agents  ctrl+p commands";

impl Renderer for Opencode {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Opencode
    }

    fn welcome(&self, ui: &mut Ui) {
        let w = ui.width;
        let right = "39,413  20% ($0.29)";
        let title = format!("# {}", crate::content::PROJECT);
        let avail = w.saturating_sub(6);
        let show_right = avail >= title.chars().count() + right.chars().count() + 2;
        let right_shown = if show_right { right } else { "" };
        let text_used = title.chars().count() + right_shown.chars().count();
        let mid_pad = " ".repeat(avail.saturating_sub(text_used).max(1));
        let total_used = 4 + text_used + mid_pad.chars().count();
        let tail_pad = " ".repeat(w.saturating_sub(total_used));

        ui.blank();
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        ui.parts(&[
            (ansi::OC_PANEL_BORDER, " \u{2503}  "),
            (ansi::OC_PANEL_BOLD, &title),
            (ansi::OC_PANEL_BG, &mid_pad),
            (ansi::OC_PANEL_DIM, right_shown),
            (ansi::OC_PANEL_BG, &tail_pad),
        ]);
        panel_blank(ui, ansi::OC_PANEL_BORDER);
    }

    fn prompt(&self, ui: &mut Ui, text: &str) {
        let w = ui.width;
        let inner = w.saturating_sub(6);
        ui.blank();
        panel_blank(ui, ansi::OC_PANEL_ACCENT);
        for line in ansi::wrap(text, inner) {
            let used = 4 + line.chars().count();
            let pad = " ".repeat(w.saturating_sub(used));
            ui.type_on_bg_lead(
                ansi::OC_PANEL_ACCENT,
                " \u{2503}  ",
                ansi::OC_PANEL_BG,
                &line,
                &pad,
                7,
            );
        }
        panel_blank(ui, ansi::OC_PANEL_ACCENT);
        ui.blank();
    }

    fn assistant(&self, ui: &mut Ui, text: &str) {
        if is_reasoning_opener(text) {
            let full = format!("Thinking: {text}");
            for line in ansi::wrap(&full, ui.width.saturating_sub(6)) {
                ui.parts(&[(ansi::GRAY, " \u{2503}  "), (ansi::DIM, &line)]);
            }
            ui.blank();
        } else {
            ui.blank();
            for line in ansi::wrap(text, ui.width.saturating_sub(5)) {
                ui.write_raw("   ");
                ui.type_text(&line, 3);
                ui.nl();
            }
        }
    }

    fn search(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("\"src/**/*{query}*\"");
        self.tool(ui, "Glob", &args, found);
    }

    fn grep(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("\"{query}\" in src");
        self.tool(ui, "Grep", &args, found);
    }

    fn read(&self, ui: &mut Ui, file: &str, start: u64, len: u64) {
        let args = format!("{file} [offset={start}, limit={len}]");
        self.tool(ui, "Read", &args, "");
    }

    fn update(&self, ui: &mut Ui, file: &str, adds: usize, rems: usize) {
        let skin = self.skin();
        ui.parts(&[
            (ansi::GRAY, "   "),
            (skin.accent, "\u{2190} "),
            (ansi::BOLD, "Edit "),
            ("", file),
            (ansi::GREEN, &format!(" (+{adds}")),
            (ansi::RED, &format!(" -{rems})")),
        ]);
    }

    fn patch(&self, ui: &mut Ui, diff: &[&str], start: u64) {
        let mut old = start;
        let mut new = start;
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        for raw in diff {
            let raw = *raw;
            match raw.chars().next() {
                Some('+') => {
                    let row = format!("{new:>4} + {}", &raw[1..]);
                    panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_GREEN, &row);
                    new += 1;
                }
                Some('-') => {
                    let row = format!("{old:>4} - {}", &raw[1..]);
                    panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_RED, &row);
                    old += 1;
                }
                _ => {
                    let body = raw.strip_prefix(' ').unwrap_or(raw);
                    let row = format!("{new:>4}   {body}");
                    panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_DIM, &row);
                    old += 1;
                    new += 1;
                }
            }
            ui.pause(16);
        }
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        ui.blank();
    }

    fn edit(&self, ui: &mut Ui, file: &str, note: &str) {
        let skin = self.skin();
        let suffix = if note.is_empty() {
            String::new()
        } else {
            format!(" ({note})")
        };
        ui.parts(&[
            (ansi::GRAY, "   "),
            (skin.accent, "\u{2190} "),
            (ansi::BOLD, "Edit "),
            ("", file),
            (ansi::DIM, &suffix),
        ]);
    }

    fn bash(&self, ui: &mut Ui, cmd: &str, out: &[&str]) {
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        panel_line(
            ui,
            ansi::OC_PANEL_BORDER,
            ansi::OC_PANEL_BOLD,
            &format!("$ {cmd}"),
        );
        for line in out {
            panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_DIM, line);
            ui.pause(22);
        }
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        ui.blank();
    }

    fn tool(&self, ui: &mut Ui, name: &str, args: &str, out: &str) {
        let skin = self.skin();
        if name == "Bash" {
            let rows: &[&str] = if out.is_empty() { &[] } else { &[out] };
            self.bash(ui, args, rows);
            return;
        }
        let (glyph, label) = match name {
            "Read" => ("\u{2192}", "Read"),
            "Update" | "Edit" => ("\u{2190}", "Edit"),
            other => ("\u{2731}", other),
        };
        let args = if args.is_empty() {
            String::new()
        } else {
            format!(" {args}")
        };
        let out = if out.is_empty() {
            String::new()
        } else {
            format!(" ({out})")
        };
        ui.parts(&[
            (ansi::GRAY, "   "),
            (skin.accent, glyph),
            (ansi::GRAY, " "),
            (ansi::BOLD, label),
            (ansi::DIM, &args),
            (ansi::DIM, &out),
        ]);
    }

    fn output(&self, ui: &mut Ui, _first: bool, text: &str, color: &'static str) {
        let style = if color == ansi::RED {
            ansi::OC_PANEL_RED
        } else {
            ansi::OC_PANEL_DIM
        };
        panel_line(ui, ansi::OC_PANEL_BORDER, style, text);
    }

    fn alert(&self, ui: &mut Ui, rng: &mut crate::rng::Rng) {
        let cmd = "cargo clippy --all-targets -- -D warnings";
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        panel_line(
            ui,
            ansi::OC_PANEL_BORDER,
            ansi::OC_PANEL_BOLD,
            &format!("$ {cmd}"),
        );
        for line in crate::content::LINT_ERROR {
            panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_RED, line);
            ui.pause(20);
        }
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        ui.pause(420);
        self.edit(ui, "src/api/handlers.rs", "Removed the duplicated branch");
        self.patch(ui, crate::content::LINT_FIX, 54);
        self.bash(ui, cmd, &["0 warnings, 0 errors"]);
        let _ = rng; // suppress unused warning for now
    }

    fn todo(&self, ui: &mut Ui, items: &[&str]) {
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        panel_line(ui, ansi::OC_PANEL_BORDER, ansi::OC_PANEL_BOLD, "# Todo");
        for (i, item) in items.iter().enumerate() {
            let pending = i + 1 == items.len();
            let row = if pending {
                format!("[\u{2022}] {item}")
            } else {
                format!("[\u{2713}] {item}")
            };
            let style = if pending {
                ansi::OC_PANEL_BG
            } else {
                ansi::OC_PANEL_DIM
            };
            panel_line(ui, ansi::OC_PANEL_BORDER, style, &row);
        }
        panel_blank(ui, ansi::OC_PANEL_BORDER);
        ui.blank();
    }

    fn footer(&self, ui: &mut Ui, secs: u64, tokens: u32) {
        let skin = self.skin();
        let w = ui.width;
        let cost = tokens as f64 * 0.000009;
        let pct = (tokens / 1200).clamp(6, 78);

        // 1. Assistant turn completion badge (`▣ Build · qwen3.5-27b · 4.2s`)
        ui.parts(&[
            (ansi::GRAY, "   "),
            (skin.accent, "\u{25a3} Build"),
            (
                ansi::DIM,
                &format!(
                    " \u{00b7} {} \u{00b7} {} ({} tokens \u{00b7} {pct}% \u{00b7} ${cost:.2})",
                    skin.model,
                    human_secs(secs),
                    ansi::tokens_label(tokens)
                ),
            ),
        ]);
        ui.blank();

        // 2. Keybind hint bar (`⬝⬝⬝⬝⬝⬝⬝⬝ esc interrupt   ctrl+t variants  tab agents  ctrl+p commands`)
        let left_str =
            " \u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d} esc interrupt";
        let l = left_str.chars().count();
        let r = KEYBINDS.chars().count() + 1;
        let show_keys = w >= l + r + 2;
        let keys = if show_keys { KEYBINDS } else { "" };
        let mid = w.saturating_sub(l + keys.chars().count() + 1).max(1);
        ui.parts(&[
            (ansi::GRAY, " "),
            (
                skin.accent,
                "\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}\u{2b1d}",
            ),
            (ansi::DIM, " esc interrupt"),
            (ansi::GRAY, &" ".repeat(mid)),
            (ansi::DIM, keys),
        ]);

        // 3. `routes/session/footer.tsx` bar (`~/code/... (main)` + `• LSP  • MCP  1.2.14`)
        let cwd = format!(" ~/code/{} (main)", crate::content::PROJECT);
        let right_status = "\u{2022} LSP  \u{2022} MCP  1.2.14";
        let show_status = w >= cwd.chars().count() + right_status.chars().count() + 3;
        if show_status {
            let pad = w
                .saturating_sub(cwd.chars().count() + right_status.chars().count() + 1)
                .max(1);
            ui.parts(&[
                (ansi::DIM, &cwd),
                (ansi::GRAY, &" ".repeat(pad)),
                (ansi::GREEN, "\u{2022}"),
                (ansi::DIM, " LSP  "),
                (ansi::GREEN, "\u{2022}"),
                (ansi::DIM, " MCP  1.2.14"),
            ]);
        } else {
            ui.parts(&[(ansi::DIM, &cwd)]);
        }
    }

    fn hints(&self) -> bool {
        false
    }

    fn ended(&self, ui: &mut Ui) {
        ui.parts(&[(ansi::GRAY, "   "), (ansi::DIM, ended_note())]);
    }
}

fn panel_blank(ui: &mut Ui, border_code: &'static str) {
    let pad = " ".repeat(ui.width.saturating_sub(4));
    ui.parts(&[(border_code, " \u{2503}  "), (ansi::OC_PANEL_BG, &pad)]);
}

fn panel_line(ui: &mut Ui, border_code: &'static str, fg_bg: &'static str, text: &str) {
    let w = ui.width;
    let avail = w.saturating_sub(5);
    let (shown, _) = split_at_chars(text, avail);
    let used = 4 + shown.chars().count();
    let pad = " ".repeat(w.saturating_sub(used));
    ui.parts(&[
        (border_code, " \u{2503}  "),
        (fg_bg, shown),
        (ansi::OC_PANEL_BG, &pad),
    ]);
}

fn is_reasoning_opener(text: &str) -> bool {
    crate::content::INTENTS.contains(&text)
}
