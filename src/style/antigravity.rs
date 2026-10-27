//! Google Antigravity CLI (`agy`).
//!
//! Layout notes taken from the real `agy` TUI: the dotted/horizon ASCII glyph
//! banner (`···· / ······ / ──────── / ···  ···  ···  ···`) with
//! `"Welcome to the Antigravity CLI."`, `───`-framed `❯` prompt, `○` circle
//! tool trajectory lines (`○ Read(...)`, `○ Edit(...)`, `○ Bash(...)`,
//! `○ Search(...)`, `○ Grep(...)`, `○ Subagents(...)`) with `⎿` tree output and
//! `(ctrl+o to expand)` (`view.toggle_trajectory`), and the two-line
//! `/statusline` HUD with 10-block context/quota bars and `accept-edits` mode.

use super::{ended_note, human_secs, patch_indented, split_at_chars, Renderer, Skin};
use crate::ansi::{self, SpinnerKind, Ui};

pub struct Antigravity;

pub static ANTIGRAVITY: Antigravity = Antigravity;

pub static SKIN: Skin = Skin {
    id: "antigravity",
    title: "Antigravity CLI (agy)",
    model: "Gemini 3.1 Pro (High)",
    accent: "\x1b[38;2;138;180;248m",
    rgb: (138, 180, 248),
    rgb_light: (26, 115, 232),
    bullet: "\u{25cb}",
    result: "\u{23bf}",
    prompt: "\u{276f} ",
    footer: "\u{23f5}\u{23f5} accept-edits (shift+tab) \u{00b7} ctrl+o trajectory \u{00b7} ctrl+k approve \u{00b7} alt+j next agent",
    hints: &[
        "\u{23f5}\u{23f5} accept-edits (shift+tab) \u{00b7} ctrl+o trajectory \u{00b7} ctrl+k approve \u{00b7} alt+j next agent",
        "/statusline to customize the HUD \u{00b7} /agents to inspect subagents",
        "/goal to run autonomously until nothing is accomplished",
        "sandbox: standard \u{00b7} 0 subagents \u{00b7} 0 files modified",
    ],
    thinking: "Thinking",
    provider: "Google DeepMind",
    commands: &[
        "@file        attach file or symbol context",
        "!cmd         execute shell command in sandbox",
        "/agents      manage background subagents (research, self, browser)",
        "/statusline  configure real-time status bar HUD",
        "/goal        autonomous execution until goal is met",
    ],
    hint: "? for help \u{00b7} @ for files \u{00b7} ! for shell \u{00b7} /agents \u{00b7} /statusline",
    boxed: false,
};

impl Renderer for Antigravity {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Antigravity
    }

    fn welcome(&self, ui: &mut Ui) {
        let skin = self.skin();
        ui.blank();
        ui.parts(&[(skin.accent, "  \u{00b7}\u{00b7}\u{00b7}\u{00b7}")]);
        ui.parts(&[(
            skin.accent,
            "  \u{00b7}\u{00b7}\u{00b7}\u{00b7}\u{00b7}\u{00b7}",
        )]);
        ui.parts(&[(
            skin.accent,
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        )]);
        ui.parts(&[
            (
                skin.accent,
                "  \u{00b7}\u{00b7}\u{00b7}  \u{00b7}\u{00b7}\u{00b7}",
            ),
            (
                ansi::DIM,
                "  \u{00b7}\u{00b7}\u{00b7}  \u{00b7}\u{00b7}\u{00b7}",
            ),
        ]);
        ui.blank();
        ui.parts(&[(ansi::BOLD, "  Welcome to the Antigravity CLI.")]);
        ui.parts(&[
            (ansi::DIM, "  "),
            (skin.accent, skin.model),
            (
                ansi::DIM,
                &format!(
                    " \u{00b7} ~/code/{} (main) \u{00b7} mode: accept-edits",
                    crate::content::PROJECT
                ),
            ),
        ]);
        ui.parts(&[(ansi::DIM, "  "), (ansi::DIM, skin.hint)]);
        ui.blank();
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
                ui.write_styled(skin.accent, "\u{2726} ");
                ui.type_text(line, 3);
                ui.nl();
            } else {
                ui.parts(&[(ansi::GRAY, "  "), ("", line)]);
            }
        }
    }

    fn search(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("pattern: \"*{query}*\", path: \"src\"");
        let out = format!("{found} (ctrl+o to expand)");
        self.tool(ui, "Search", &args, &out);
    }

    fn grep(&self, ui: &mut Ui, query: &str, found: &str) {
        let args = format!("query: \"{query}\", path: \"src\"");
        let out = format!("Found {found} (ctrl+o to expand)");
        self.tool(ui, "Grep", &args, &out);
    }

    fn read(&self, ui: &mut Ui, file: &str, start: u64, len: u64) {
        let end = start + len;
        let args = format!("{file}:{start}-{end}");
        let out = format!("Read {len} lines (ctrl+o to expand)");
        self.tool(ui, "Read", &args, &out);
    }

    fn update(&self, ui: &mut Ui, file: &str, adds: usize, rems: usize) {
        self.tool(ui, "Edit", file, "");
        ui.parts(&[
            (ansi::GRAY, "  \u{23bf}  "),
            (ansi::DIM, &format!("Updated {file} (")),
            (ansi::GREEN, &format!("+{adds}")),
            (ansi::DIM, " "),
            (ansi::RED, &format!("-{rems}")),
            (ansi::DIM, ")"),
        ]);
    }

    fn patch(&self, ui: &mut Ui, diff: &[&str], start: u64) {
        patch_indented(ui, "     ", diff, start);
    }

    fn edit(&self, ui: &mut Ui, file: &str, note: &str) {
        self.tool(ui, "Edit", file, note);
    }

    fn bash(&self, ui: &mut Ui, cmd: &str, out: &[&str]) {
        self.tool(ui, "Bash", cmd, "");
        let last = out.len().saturating_sub(1);
        for (i, line) in out.iter().enumerate() {
            if i == last {
                let expanded = format!("{line} (ctrl+o to expand)");
                self.output(ui, i == 0, &expanded, ansi::DIM);
            } else {
                self.output(ui, i == 0, line, ansi::DIM);
            }
            ui.pause(22);
        }
    }

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

    fn output(&self, ui: &mut Ui, first: bool, text: &str, color: &'static str) {
        let color = if color.is_empty() { ansi::DIM } else { color };
        let lead = if first { "  \u{23bf}  " } else { "     " };
        ui.parts(&[(ansi::GRAY, lead), (color, text)]);
    }

    fn todo(&self, ui: &mut Ui, items: &[&str]) {
        let skin = self.skin();
        let args = format!("{} active \u{00b7} /agents", items.len());
        self.tool(ui, "Subagents", &args, "");
        let roles = ["[research]", "[self]    ", "[verify]  ", "[self]    "];
        for (i, item) in items.iter().enumerate() {
            let pending = i + 1 == items.len();
            let (icon_col, icon) = if pending {
                (skin.accent, "\u{27f3} ")
            } else {
                (ansi::GREEN, "\u{2713} ")
            };
            let role = roles[i % roles.len()];
            let lead = if i == 0 { "  \u{23bf}  " } else { "     " };
            let text_col = if pending { "" } else { ansi::DIM };
            ui.parts(&[
                (ansi::GRAY, lead),
                (icon_col, icon),
                (skin.accent, role),
                ("", " "),
                (text_col, item),
            ]);
        }
    }

    fn footer(&self, ui: &mut Ui, secs: u64, tokens: u32) {
        let skin = self.skin();
        let ctx_pct = (tokens / 320).clamp(6, 68) as usize;
        let quota_pct = 100usize.saturating_sub(ctx_pct / 2).clamp(40, 96);
        let ctx_bar = bar10(ctx_pct);
        let quota_bar = bar10(quota_pct);

        ui.blank();
        if ui.width >= 76 {
            ui.parts(&[
                (ansi::GRAY, "  "),
                (ansi::GREEN, "READY"),
                (ansi::GRAY, " \u{2502} "),
                (skin.accent, skin.model),
                (ansi::GRAY, " \u{2502} "),
                (ansi::DIM, &format!("Ctx [{ctx_bar}] {ctx_pct}%")),
                (ansi::GRAY, " \u{2502} "),
                (ansi::DIM, &format!("Quota [{quota_bar}] {quota_pct}%")),
                (ansi::GRAY, " \u{2502} "),
                (ansi::DIM, &format!("main \u{00b7} {}", human_secs(secs))),
            ]);
        } else {
            ui.parts(&[
                (ansi::GRAY, "  "),
                (ansi::GREEN, "READY"),
                (ansi::GRAY, " \u{2502} "),
                (skin.accent, skin.model),
                (ansi::GRAY, " \u{2502} "),
                (
                    ansi::DIM,
                    &format!("Ctx [{ctx_bar}] {ctx_pct}% \u{00b7} {}", human_secs(secs)),
                ),
            ]);
        }
        ui.parts(&[(ansi::DIM, "  "), (ansi::DIM, skin.footer)]);
    }

    fn hints(&self) -> bool {
        false
    }

    fn ended(&self, ui: &mut Ui) {
        ui.parts(&[
            (ansi::DIM, "  "),
            (self.skin().accent, "\u{25cb} "),
            (ansi::DIM, ended_note()),
        ]);
    }
}

fn bar10(pct: usize) -> String {
    let filled = (pct * 10 / 100).clamp(1, 10);
    format!(
        "{}{}",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(10 - filled)
    )
}
