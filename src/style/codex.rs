//! OpenAI Codex CLI (`codex-rs`).
//!
//! Layout notes taken from the real CLI: a `>_` header box with version, model
//! and directory, a startup command list, a `›` prompt, `• Explored` /
//! `• Edited` / `• Ran` tool blocks with `└` results, a `─ Worked for Xs ───`
//! divider, and a `model · dir · % context left` status line.

use super::{box_bottom, box_row, box_top, ended_note, human_secs, patch_indented, Renderer, Skin};
use crate::ansi::{self, SpinnerKind, Ui};

pub struct Codex;

pub static CODEX: Codex = Codex;

pub static SKIN: Skin = Skin {
    id: "codex",
    title: "OpenAI Codex (v0.114.0)",
    model: "gpt-5.4 medium",
    accent: "\x1b[38;2;114;161;255m",
    rgb: (114, 161, 255),
    rgb_light: (9, 105, 218),
    bullet: "\u{2022}",
    result: "\u{2514}",
    prompt: "\u{203a} ",
    footer: "gpt-5.4 medium \u{00b7} ~/code/distributed-cluster \u{00b7} 94% context left",
    hints: &[
        "gpt-5.4 medium \u{00b7} ~/code/distributed-cluster \u{00b7} 94% context left",
        "/status to inspect this session",
        "/approvals to change sandbox permissions",
        "/review to check changes before shipping",
    ],
    thinking: "Working",
    provider: "OpenAI",
    commands: &[
        "/init - create an AGENTS.md file with instructions for Codex",
        "/status - show current session configuration",
        "/approvals - choose what Codex can do without approval",
        "/model - choose what model and reasoning effort to use",
        "/review - review any changes and find issues",
    ],
    hint: "workspace-write sandbox \u{00b7} /approvals",
    boxed: false,
};

impl Renderer for Codex {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Codex
    }

    fn welcome(&self, ui: &mut Ui) {
        let skin = self.skin();
        let w = ui.width.clamp(40, 70);
        box_top(ui, w);
        box_row(ui, w, &[(skin.accent, ">_ "), (ansi::BOLD, skin.title)]);
        box_row(ui, w, &[("", "")]);
        box_row(
            ui,
            w,
            &[
                (ansi::DIM, "model:     "),
                ("", skin.model),
                (ansi::DIM, "   /model to change"),
            ],
        );
        box_row(
            ui,
            w,
            &[
                (ansi::DIM, "directory: "),
                ("", &format!("~/code/{}", crate::content::PROJECT)),
            ],
        );
        box_bottom(ui, w);
        ui.blank();
        ui.parts(&[(
            ansi::DIM,
            "  To get started, describe a task or try one of these commands:",
        )]);
        ui.blank();
        for cmd in skin.commands {
            ui.parts(&[(ansi::DIM, "  "), (ansi::DIM, cmd)]);
        }
        ui.blank();
    }

    fn prompt(&self, ui: &mut Ui, text: &str) {
        ui.blank();
        ui.write_styled(self.skin().accent, self.skin().prompt);
        ui.type_text(text, 7);
        ui.write_raw("\n\n");
    }

    fn assistant(&self, ui: &mut Ui, text: &str) {
        let skin = self.skin();
        let lines = ansi::wrap(text, ui.width.saturating_sub(4));
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                ui.write_styled(skin.accent, "\u{2022} ");
                ui.type_text(line, 3);
                ui.nl();
            } else {
                ui.parts(&[(ansi::GRAY, "  "), ("", line)]);
            }
        }
    }

    fn search(&self, ui: &mut Ui, query: &str, found: &str) {
        let skin = self.skin();
        ui.parts(&[(skin.accent, skin.bullet), (ansi::BOLD, " Explored")]);
        self.output(
            ui,
            true,
            &format!("Search \"{query}\" in src ({found})"),
            ansi::DIM,
        );
    }

    fn grep(&self, ui: &mut Ui, query: &str, found: &str) {
        let skin = self.skin();
        ui.parts(&[(skin.accent, skin.bullet), (ansi::BOLD, " Explored")]);
        self.output(
            ui,
            true,
            &format!("Search \"{query}\" ({found})"),
            ansi::DIM,
        );
    }

    fn read(&self, ui: &mut Ui, file: &str, start: u64, len: u64) {
        let skin = self.skin();
        let end = start + len;
        ui.parts(&[(skin.accent, skin.bullet), (ansi::BOLD, " Explored")]);
        self.output(
            ui,
            true,
            &format!("Read {file} (lines {start}\u{2013}{end})"),
            ansi::DIM,
        );
    }

    fn update(&self, ui: &mut Ui, file: &str, adds: usize, rems: usize) {
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::BOLD, " Edited "),
            ("", file),
            (ansi::GREEN, &format!(" (+{adds}")),
            (ansi::RED, &format!(" -{rems})")),
        ]);
    }

    fn patch(&self, ui: &mut Ui, diff: &[&str], start: u64) {
        patch_indented(ui, "    ", diff, start);
    }

    fn edit(&self, ui: &mut Ui, file: &str, note: &str) {
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::BOLD, " Edited "),
            ("", file),
        ]);
        if !note.is_empty() {
            self.output(ui, true, note, ansi::DIM);
        }
    }

    fn tool(&self, ui: &mut Ui, name: &str, args: &str, out: &str) {
        let skin = self.skin();
        let label = match name {
            "Update" | "Edit" => "Edited",
            "Bash" => "Ran",
            other => other,
        };
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::GRAY, " "),
            (ansi::BOLD, label),
            (ansi::GRAY, " "),
            (ansi::DIM, args),
        ]);
        if !out.is_empty() {
            self.output(ui, true, out, ansi::DIM);
        }
    }

    fn output(&self, ui: &mut Ui, first: bool, text: &str, color: &'static str) {
        let color = if color.is_empty() { ansi::DIM } else { color };
        let lead = if first { "  \u{2514} " } else { "    " };
        ui.parts(&[(ansi::GRAY, lead), (color, text)]);
    }

    fn todo(&self, ui: &mut Ui, items: &[&str]) {
        let skin = self.skin();
        ui.parts(&[
            (skin.accent, skin.bullet),
            (ansi::GRAY, " "),
            (ansi::BOLD, "Updated Plan"),
        ]);
        for (i, item) in items.iter().enumerate() {
            let pending = i + 1 == items.len();
            let (code, glyph) = if pending {
                (ansi::DIM, "\u{25a1} ")
            } else {
                (ansi::GREEN, "\u{2714} ")
            };
            let lead = if i == 0 { "  \u{2514} " } else { "    " };
            let text_style = if pending { "" } else { ansi::DIM };
            ui.parts(&[(ansi::GRAY, lead), (code, glyph), (text_style, item)]);
        }
    }

    fn footer(&self, ui: &mut Ui, secs: u64, tokens: u32) {
        let skin = self.skin();
        let worked = format!("\u{2500} Worked for {} ", human_secs(secs));
        let trail = "\u{2500}".repeat(ui.width.saturating_sub(worked.chars().count()));
        ui.parts(&[(ansi::DIM, &worked), (ansi::GRAY, &trail)]);

        let left = 100u32.saturating_sub(tokens / 900).max(4);
        ui.parts(&[
            (ansi::GRAY, "  "),
            (ansi::DIM, skin.model),
            (
                ansi::DIM,
                &format!(" \u{00b7} ~/code/{} \u{00b7} ", crate::content::PROJECT),
            ),
            (
                ansi::DIM,
                &format!("{left}% context left \u{00b7} ? for shortcuts"),
            ),
        ]);
    }

    fn hints(&self) -> bool {
        false
    }

    fn ended(&self, ui: &mut Ui) {
        ui.parts(&[(ansi::GRAY, "  "), (ansi::DIM, ended_note())]);
    }
}
