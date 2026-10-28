use super::{Renderer, Skin};

pub struct Hollywood;

pub static HOLLYWOOD: Hollywood = Hollywood;

pub static SKIN: Skin = Skin {
    id: "hollywood",
    title: "HOLLYWOOD CYBER WARFARE v4.09",
    model: "CYBER-CORE v9.4",
    accent: "\x1b[38;2;0;255;102m",
    rgb: (0, 255, 102),
    rgb_light: (0, 180, 70),
    bullet: "\u{25b6}",
    result: "\u{255a}",
    prompt: "root@pentagon:~# ",
    footer: "MAINFRAME COMPROMISED \u{00b7} 0-DAY EXPLOIT ARMED",
    hints: &[
        "BREACHING DEFENSE GRID \u{00b7} NEURAL OVERRIDE ACTIVE",
        "SATELLITE DOWNLINK SYNCHRONIZED \u{00b7} BIND SHELL ONLINE",
        "ENCRYPTION KEY CRACKED \u{00b7} ESC TO ABORT MISSION",
    ],
    thinking: "Cracking",
    provider: "CYBER-CORE",
    commands: &[],
    hint: "/override to bypass firewall \u{00b7} /nuke to wipe logs",
    boxed: true,
};

impl Renderer for Hollywood {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }
}
