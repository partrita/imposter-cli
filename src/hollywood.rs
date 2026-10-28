use crate::ansi::{self, Ui};
use crate::cli::Args;
use crate::config::Config;
use crate::rng::Rng;
use std::time::Instant;

const TARGETS: &[&str] = &[
    "PENTAGON_PRIMARY_GRID",
    "GCHQ_ENCRYPTION_RELAY",
    "NSA_QUANTUM_MAINFRAME",
    "KREMLIN_SECURE_SUBNET",
    "SWIFT_INTERBANK_GATEWAY",
    "ORBITAL_DEFENSE_SATELLITE_4",
    "INTERPOL_CENTRAL_REGISTRY",
    "CERN_HADRON_CONTROLLER",
];

const EXPLOITS: &[&str] = &[
    "ZERO_DAY_BUFFER_OVERFLOW",
    "ELLIPTIC_CURVE_POISONING",
    "SIDE_CHANNEL_POWER_CORRELATION",
    "QUANTUM_SHOR_FACTORIZATION",
    "BGP_ROUTE_HIJACKING_VECTOR",
    "MEMORY_CORRUPTION_ROP_CHAIN",
    "SQLI_BLIND_TIME_BASED_INJECTION",
    "SPECTRE_V2_BRANCH_TARGET_INJECTION",
];

const ASCII_BANNER: &[&str] = &[
    r"  ______   ______   ______   ______   ______   ______   ______ ",
    r" /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/ ",
    r"  _    _  ____  _      _     __   __ _          __   ____  _____  ",
    r" | |  | |/ __ \| |    | |    \ \ / /| |        / /  / __ \|  __ \ ",
    r" | |__| | |  | | |    | |     \ V / | |  __   / /  | |  | | |  | |",
    r" |  __  | |  | | |    | |      > <  | | /  \ / /   | |  | | |  | |",
    r" | |  | | |__| | |____| |____ / . \ | |/ /\ V /    | |__| | |__| |",
    r" |_|  |_|\____/|______|______/_/ \_\|___/  \_/      \____/|_____/ ",
    r"  ______   ______   ______   ______   ______   ______   ______ ",
    r" /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/ ",
];

const CYAN: &str = "\x1b[38;2;0;255;204m";
const GREEN: &str = "\x1b[38;2;0;255;102m";
const MAGENTA: &str = "\x1b[38;2;255;0;127m";

pub fn run(ui: &mut Ui, cfg: &Config, args: &Args, rng: &mut Rng, deadline: Option<Instant>) {
    boot(ui, cfg, rng);

    let speed_factor = cfg.speed.delay_factor();
    let mut step = 0u32;

    while !crate::expired(deadline) {
        step += 1;
        match step % 6 {
            1 => split_stream(ui, rng, speed_factor),
            2 => firewall_breach(ui, rng, speed_factor),
            3 => matrix_waterfall(ui, rng, speed_factor),
            4 => access_granted(ui, rng, speed_factor),
            5 => packet_sniff(ui, rng, speed_factor),
            _ => brute_force_keys(ui, rng, speed_factor),
        }

        if args.alerts && rng.chance(1, 2) {
            satellite_scramble(ui, rng, speed_factor);
        }

        ui.flush();
        let pause_ms = ((60.0 * speed_factor) as u64).max(10);
        ui.pause(pause_ms);
    }

    ui.parts(&[(
        ansi::RED,
        " [!] CONNECTION SEVERED BY COUNTER-CYBER COMMAND.",
    )]);
}

fn boot(ui: &mut Ui, cfg: &Config, rng: &mut Rng) {
    let speed_factor = cfg.speed.delay_factor();

    for line in ASCII_BANNER {
        ui.parts(&[(CYAN, line)]);
        let p = ((10.0 * speed_factor) as u64).max(2);
        ui.pause(p);
    }

    let model = cfg.model.as_deref().unwrap_or("CYBER-CORE v9.4 neural");
    ui.parts(&[
        (ansi::BOLD, " :: TARGET MAINFRAME : "),
        (
            ansi::YELLOW,
            TARGETS[rng.range(0, (TARGETS.len() - 1) as u64) as usize],
        ),
        (ansi::DIM, " \u{00b7} "),
        (GREEN, model),
    ]);
    ui.parts(&[(
        ansi::DIM,
        " :: INITIATING HIGH-FREQUENCY NEURAL OVERRIDE STREAM...",
    )]);
    ui.flush();
    let p = ((180.0 * speed_factor) as u64).max(20);
    ui.pause(p);
}

/// 2-column cyberpunk split stream:
/// Left: Raw Hex dump | Right: Live Breach / Exploit logs
fn split_stream(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    let lines = rng.range(14, 24);

    for i in 0..lines {
        let addr = format!("0x{:08X}", rng.next_u64() & 0xFFFF_FFFF);
        let mut hex_part = String::with_capacity(24);
        for _ in 0..8 {
            let b = rng.range(0, 255) as u8;
            hex_part.push_str(&format!("{:02X} ", b));
        }

        let right_msg = match i % 5 {
            0 => format!("{}HOOK -> TRAP_INTERRUPT 0x00{}", GREEN, rng.range(10, 99)),
            1 => format!(
                "{}ROP_CHAIN [0x{:04X}] POP RAX ; RET{}",
                CYAN,
                rng.range(0x1000, 0xFFFF),
                ansi::RESET
            ),
            2 => format!(
                "{}DECRYPTING RSA-4096 [{:04X}...] OK{}",
                MAGENTA,
                rng.range(0x1000, 0xFFFF),
                ansi::RESET
            ),
            3 => format!(
                "{}BYPASS_CANARY_COOKIE [0x{:08X}]{}",
                ansi::YELLOW,
                rng.next_u64() & 0xFFFF_FFFF,
                ansi::RESET
            ),
            _ => format!(
                "{}EXFILTRATE -> BLOB {:>3}MB/s [CH-4]{}",
                GREEN,
                rng.range(380, 940),
                ansi::RESET
            ),
        };

        ui.parts(&[
            (ansi::DIM, &addr),
            (ansi::DIM, " "),
            (GREEN, &hex_part),
            (CYAN, "│ "),
            ("", &right_msg),
        ]);
        let p = ((6.0 * speed_factor) as u64).max(1);
        ui.pause(p);
    }
}

fn matrix_waterfall(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    let rows = rng.range(10, 18);
    let width = ui.width.clamp(40, 80);
    let matrix_chars = b"0123456789ABCDEF!#%&*+-=<>~XYZ";

    for _ in 0..rows {
        let mut line = String::with_capacity(width);
        for _ in 0..width {
            let ch = matrix_chars[(rng.range(0, (matrix_chars.len() - 1) as u64)) as usize] as char;
            line.push(ch);
        }
        let color = match rng.range(0, 2) {
            0 => GREEN,
            1 => CYAN,
            _ => ansi::BOLD,
        };
        ui.parts(&[(color, &line)]);
        let p = ((5.0 * speed_factor) as u64).max(1);
        ui.pause(p);
    }
}

fn firewall_breach(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    let target = TARGETS[rng.range(0, (TARGETS.len() - 1) as u64) as usize];
    let exploit = EXPLOITS[rng.range(0, (EXPLOITS.len() - 1) as u64) as usize];

    ui.parts(&[
        (CYAN, " [>>>] OVERRIDING FIREWALL: "),
        (ansi::BOLD, target),
        (ansi::DIM, " \u{00b7} "),
        (MAGENTA, exploit),
    ]);

    let width = 36;
    for percent in (0..=100).step_by(10) {
        let filled = (width * percent) / 100;
        let empty = width - filled;
        let bar = format!(
            " [{}{}] {:>3}%",
            "\u{2588}".repeat(filled),
            "\u{2591}".repeat(empty),
            percent
        );
        ui.write_raw(&format!(
            "\r\x1b[2K\x1b[38;2;0;255;204m{bar} \x1b[38;2;255;0;127mBYPASSING PROXY LAYERS..."
        ));
        ui.flush();
        let p = ((10.0 * speed_factor) as u64).max(2);
        ui.pause(p);
    }
    ui.nl();
}

fn access_granted(ui: &mut Ui, _rng: &mut Rng, speed_factor: f32) {
    let w = ui.width.clamp(40, 72);
    let border = "\u{2550}".repeat(w - 4);

    ui.parts(&[(CYAN, &format!("  \u{2554}{border}\u{2557}"))]);
    ui.parts(&[(
        GREEN,
        "  \u{2551}           [ ACCESS GRANTED \u{00b7} ROOT SHELL ACQUIRED ]           \u{2551}",
    )]);
    ui.parts(&[(
        MAGENTA,
        "  \u{2551}           MAINFRAME COMPROMISED \u{00b7} EXFILTRATION READY          \u{2551}",
    )]);
    ui.parts(&[(CYAN, &format!("  \u{255a}{border}\u{255d}"))]);

    let p = ((100.0 * speed_factor) as u64).max(15);
    ui.pause(p);
}

fn packet_sniff(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    for _ in 0..8 {
        let src_ip = format!(
            "10.{}.{}.{}",
            rng.range(1, 254),
            rng.range(1, 254),
            rng.range(1, 254)
        );
        let dst_ip = format!("192.168.{}.{}", rng.range(1, 100), rng.range(1, 254));
        let port = rng.range(1024, 65535);
        let len = rng.range(64, 1518);
        ui.parts(&[
            (MAGENTA, " PKT: "),
            (ansi::DIM, &src_ip),
            (ansi::GRAY, " \u{2192} "),
            (GREEN, &dst_ip),
            (
                ansi::GRAY,
                &format!(":{} [{}b] SYN/ACK TLS_v1.3", port, len),
            ),
        ]);
        let p = ((7.0 * speed_factor) as u64).max(1);
        ui.pause(p);
    }
}

fn brute_force_keys(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    let count = rng.range(6, 12);
    for _ in 0..count {
        let key = format!("{:016X}{:016X}", rng.next_u64(), rng.next_u64());
        ui.parts(&[
            (ansi::DIM, " TRY_KEY: "),
            (ansi::DIM, &key),
            (ansi::RED, " [MISMATCH]"),
        ]);
        let p = ((4.0 * speed_factor) as u64).max(1);
        ui.pause(p);
    }
    let valid_key = format!("{:016X}{:016X}", rng.next_u64(), rng.next_u64());
    ui.parts(&[
        (ansi::BOLD, " TRY_KEY: "),
        (GREEN, &valid_key),
        (CYAN, " [MATCH FOUND \u{00b7} KEY DECRYPTED]"),
    ]);
}

fn satellite_scramble(ui: &mut Ui, rng: &mut Rng, speed_factor: f32) {
    ui.parts(&[(
        ansi::RED,
        " [!] SATELLITE DEFENSE RADAR INTERCEPT DETECTED!",
    )]);
    for _ in 0..3 {
        let freq = format!("{:.3} GHz", rng.range(14000, 18000) as f64 / 1000.0);
        ui.parts(&[
            (ansi::YELLOW, "     COUNTER-JAMMING FREQ: "),
            (ansi::BOLD, &freq),
            (CYAN, " [FREQUENCY HOPPING ENGAGED]"),
        ]);
        let p = ((12.0 * speed_factor) as u64).max(2);
        ui.pause(p);
    }
}
