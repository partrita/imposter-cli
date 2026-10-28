//! Claude Code. This is the reference layout: it overrides nothing.

use super::{Renderer, Skin};

pub struct Claude;

pub static CLAUDE: Claude = Claude;

pub static SKIN: Skin = Skin {
    id: "claude",
    title: "Claude Code v2.1.76",
    model: "Opus 4.6 \u{00b7} Claude Max",
    accent: "\x1b[38;2;215;119;87m",
    rgb: (215, 119, 87),
    rgb_light: (180, 76, 43),
    bullet: "\u{25cf}",
    result: "\u{23bf}",
    prompt: "\u{276f} ",
    footer: "\u{23f5}\u{23f5} accept edits on (shift+tab to cycle)",
    hints: &[
        "\u{23f5}\u{23f5} accept edits on (shift+tab to cycle)",
        "esc to interrupt \u{00b7} /cost for the bill you are not paying",
        "? for shortcuts \u{00b7} ctrl+o to expand \u{00b7} /clear to forget this ever happened",
        "shift+tab to cycle modes (accept edits \u{00b7} plan mode) \u{00b7} nothing is being saved",
    ],
    thinking: "Thinking",
    provider: "Claude Max",
    commands: &[],
    hint: "/help for commands \u{00b7} ! for bash",
    boxed: true,
};

impl Renderer for Claude {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }
}
