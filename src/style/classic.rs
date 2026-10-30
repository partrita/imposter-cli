//! Classic "stakeholder" style — legacy system log renderer.
//!
//! Implements the Classic phase sequence as a Renderer: animated boot,
//! analyze/metrics/network/system/processing phases, alerts, task events
//! with progress bars (compile, stream, spinner, pipeline).

use super::{Renderer, Skin};
use crate::ansi::{self, SpinnerKind, Ui};
use crate::config::Config;
use crate::content;
use crate::rng::{Deck, Rng};

pub const SERVICES: &[&str] = &[
    "ingest-gateway",
    "billing-core",
    "session-store",
    "event-relay",
    "search-index",
    "notify-worker",
    "auth-proxy",
    "media-pipeline",
];

pub struct Classic;

pub static CLASSIC: Classic = Classic;

pub static SKIN: Skin = Skin {
    id: "classic",
    title: "imposter classic",
    model: "legacy",
    accent: "\x1b[38;2;215;119;87m",
    rgb: (215, 119, 87),
    rgb_light: (180, 76, 43),
    bullet: "\u{25b8}",
    result: "\u{2713}",
    prompt: "",
    footer: "session terminated.",
    hints: &[],
    thinking: "Processing",
    provider: "",
    commands: &[],
    hint: "",
    boxed: false,
};

impl Renderer for Classic {
    fn skin(&self) -> &'static Skin {
        &SKIN
    }

    fn spinner_kind(&self) -> SpinnerKind {
        SpinnerKind::Claude
    }

    fn boot(&self, ui: &mut Ui, cfg: &Config, _rng: &mut Rng) {
        let _speed_factor = cfg.speed.delay_factor();

        ui.blank();
        ui.parts(&[
            (ansi::ACCENT, "imposter"),
            (
                ansi::DIM,
                &format!(" {} \u{00b7} classic", env!("CARGO_PKG_VERSION")),
            ),
        ]);
        ui.parts(&[(ansi::DIM, "project   "), ("", content::PROJECT)]);
        ui.parts(&[(ansi::DIM, "env       "), ("", cfg.dev_type.as_str())]);
        ui.blank();

        let width = 24;
        if !ui.animate {
            let pct = 100u32;
            let bar = "\u{2588}".repeat(width);
            ui.parts(&[
                (ansi::GREEN, &bar),
                (ansi::DIM, " 100%"),
                ("", &format!("  {}", Self::boot_label(pct))),
            ]);
            ui.blank();
            return;
        }

        for pct in 1..=100u32 {
            let filled = (pct as usize * width) / 100;
            let bar = format!(
                "{}{}",
                "\u{2588}".repeat(filled),
                "\u{2591}".repeat(width - filled)
            );
            ui.write_raw("\r\x1b[2K");
            ui.parts_inline(&[
                (ansi::GREEN, &bar),
                (ansi::DIM, &format!(" {pct:>3}%")),
                ("", &format!("  {}", Self::boot_label(pct))),
            ]);
            ui.flush();
            let pause_ms = match pct {
                18 | 41 | 67 | 88 => 70,
                99 => 80,
                _ => 12,
            };
            ui.pause(pause_ms);
        }
        ui.nl();
        ui.blank();
    }

    fn progress_bar(&self, ui: &mut Ui, pct: u32, label: &str) {
        let width = 24;
        let filled = (width * pct as usize) / 100;
        let bar = format!(
            "{}{}",
            "\u{2588}".repeat(filled),
            "\u{2591}".repeat(width - filled)
        );
        ui.write_raw("\r\x1b[2K");
        ui.parts_inline(&[
            (ansi::ACCENT, "  ["),
            (ansi::GREEN, &bar),
            (ansi::ACCENT, "]"),
            (ansi::DIM, &format!(" {pct:>3}% ")),
            ("", label),
        ]);
        ui.flush();
    }

    fn alert(&self, ui: &mut Ui, rng: &mut Rng) {
        let severity = if rng.chance(1, 4) {
            "CRITICAL"
        } else {
            "WARNING"
        };
        ui.blank();
        ui.parts(&[
            (ansi::RED, "  alert "),
            (ansi::BOLD, severity),
            (ansi::DIM, "  connection pool nearing capacity"),
        ]);
        ui.pause(120);
        ui.parts(&[
            (ansi::GREEN, "  \u{2713} "),
            ("", "auto-remediation: scaled pool 20 -> 32"),
        ]);
    }

    fn ended(&self, ui: &mut Ui) {
        ui.parts(&[(ansi::DIM, "session terminated.")]);
    }

    fn hints(&self) -> bool {
        false
    }
}

impl Classic {
    fn boot_label(pct: u32) -> &'static str {
        match pct {
            0..=24 => "resolving modules",
            25..=50 => "warming caches",
            51..=75 => "linking artifacts",
            76..=95 => "checking invariants",
            _ => "ready",
        }
    }

    fn header(ui: &mut Ui, name: &str) {
        ui.parts(&[(ansi::ACCENT, "\u{25b8} "), (ansi::BOLD, name)]);
        ui.flush();
        ui.pause(60);
    }

    fn ok(ui: &mut Ui, text: &str) {
        ui.parts(&[(ansi::GREEN, "  \u{2713} "), ("", text)]);
        ui.flush();
        ui.pause(55);
    }

    fn warn(ui: &mut Ui, text: &str) {
        ui.parts(&[(ansi::YELLOW, "  ! "), ("", text)]);
        ui.flush();
        ui.pause(55);
    }

    fn info(ui: &mut Ui, text: &str) {
        ui.parts(&[(ansi::DIM, "  \u{00b7} "), (ansi::DIM, text)]);
        ui.flush();
        ui.pause(45);
    }

    // Phase rendering methods (called by the unified engine)

    pub fn render_analyze(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        budget: usize,
    ) {
        Self::header(ui, "code analysis");
        let modules = rng.range(64, 480);
        let symbols = modules * rng.range(12, 51);
        let service = services.next(rng);
        let lines = [
            format!("resolved {symbols} symbols across {modules} modules"),
            format!(
                "type coverage {}.{}% in {service}",
                rng.range(93, 99),
                rng.range(0, 9)
            ),
            format!(
                "cyclomatic complexity {}.{} avg (max {})",
                rng.range(3, 7),
                rng.range(0, 9),
                rng.range(8, 24)
            ),
            format!("{} unused exports", rng.range(0, 7)),
            format!("{} unreachable branches in vendored code", rng.range(2, 14)),
            format!("{} crates checked, 0 advisories", rng.range(12, 40)),
        ];
        for line in lines.iter().take(budget) {
            Self::ok(ui, line);
        }
    }

    pub fn render_metrics(&self, ui: &mut Ui, rng: &mut Rng, budget: usize) {
        Self::header(ui, "runtime metrics");
        let lines = [
            format!("p50 latency  {} ms", rng.range(4, 28)),
            format!("p99 latency  {} ms", rng.range(38, 190)),
            format!("throughput   {} req/s", rng.range(480, 4200)),
            format!("queue depth  {} messages", rng.range(0, 2400)),
            format!("cache hit    {}.{}%", rng.range(78, 99), rng.range(0, 9)),
            format!(
                "error budget {}.{}% remaining",
                rng.range(88, 99),
                rng.range(0, 9)
            ),
        ];
        for (i, line) in lines.iter().take(budget).enumerate() {
            if i == 5 {
                Self::info(ui, line);
            } else {
                Self::ok(ui, line);
            }
        }
    }

    pub fn render_network(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        budget: usize,
    ) {
        Self::header(ui, "network");
        let host = services.next(rng);
        let lines = [
            format!("{host}:443  tls ok   rtt {} ms", rng.range(2, 48)),
            format!("{} keep-alive connections", rng.range(12, 240)),
            format!("{} retransmits in last 60s", rng.range(0, 12)),
            format!("dns p99 {} ms", rng.range(1, 24)),
            format!("{} upstream nodes healthy", rng.range(3, 18)),
            format!("{} requests shed by the limiter", rng.range(0, 400)),
        ];
        for (i, line) in lines.iter().take(budget).enumerate() {
            if i == 2 {
                Self::warn(ui, line);
            } else {
                Self::ok(ui, line);
            }
        }
    }

    pub fn render_system(&self, ui: &mut Ui, rng: &mut Rng, budget: usize) {
        Self::header(ui, "system");
        let lines = [
            format!("cpu    {}% of 8 cores", rng.range(6, 74)),
            format!("memory {} MB resident", rng.range(38, 720)),
            format!("disk   {} MB written", rng.range(2, 180)),
            format!("fds    {} open", rng.range(20, 900)),
            format!("{} voluntary context switches/s", rng.range(200, 9000)),
            format!("gc pause p99 {} ms", rng.range(0, 12)),
        ];
        for line in lines.iter().take(budget) {
            Self::ok(ui, line);
        }
        Self::info(ui, "no swap pressure");
    }

    pub fn render_processing(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        budget: usize,
    ) {
        Self::header(ui, "data processing");
        let service = services.next(rng);
        let rows = rng.range(12, 940) * 1000;
        let lines = [
            format!("{rows} rows scanned in {service}"),
            format!("{} shards merged", rng.range(2, 32)),
            format!("{} records deduplicated", rng.range(100, 40_000)),
            format!("batch size {} MB", rng.range(4, 64)),
            format!("{} late arrivals dropped", rng.range(0, 60)),
            format!("checkpoint in {} s", rng.range(5, 60)),
        ];
        for line in lines.iter().take(budget) {
            Self::ok(ui, line);
        }
    }

    // Task event rendering (progress bars)

    pub fn render_task_progress_bar(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        speed_factor: f32,
    ) {
        let service = services.next(rng);
        let heavy = rng.chance(1, 3);
        let task_name = if heavy {
            format!("optimizing AST & generating binaries for {service}")
        } else {
            format!("compiling {service} crate artifacts")
        };

        let total_secs = if heavy {
            rng.range(20, 42) as f32 / 10.0
        } else {
            rng.range(5, 14) as f32 / 10.0
        };

        if !ui.animate {
            Self::ok(ui, &format!("{task_name} completed in {total_secs:.1}s"));
            return;
        }

        let mut pct = 0u32;
        let base_sleep = ((total_secs * 1000.0 * speed_factor) / 35.0).max(10.0) as u64;

        while pct < 100 {
            let step = rng.range(2, if heavy { 6 } else { 12 }) as u32;
            pct = (pct + step).min(100);
            self.progress_bar(ui, pct, &task_name);
            let sleep = if rng.chance(1, 7) {
                base_sleep * 3
            } else {
                base_sleep
            };
            ui.pause(sleep);
        }
        ui.write_raw("\r\x1b[2K");
        Self::ok(ui, &format!("{task_name} completed in {total_secs:.1}s"));
    }

    pub fn render_task_counter_stream(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        speed_factor: f32,
    ) {
        let service = services.next(rng);
        let is_mb = rng.chance(1, 2);
        let target = if is_mb {
            rng.range(32, 180) as u32
        } else {
            rng.range(48, 260) as u32
        };
        let unit = if is_mb { "MB" } else { "chunks" };
        let task_name = format!("streaming payload to {service}");
        let total_secs = (rng.range(8, 20) as f32 / 10.0) * speed_factor;

        if !ui.animate {
            Self::ok(
                ui,
                &format!("{task_name} ({target}/{target} {unit}) finished"),
            );
            return;
        }

        let mut current = 0u32;
        let step_time = ((total_secs * 1000.0) / 25.0).max(12.0) as u64;

        while current < target {
            let inc = rng.range(2, (target / 8).max(3) as u64) as u32;
            current = (current + inc).min(target);
            let pct = (current * 100) / target.max(1);
            self.progress_bar(ui, pct, &format!("[{current}/{target} {unit}] {task_name}"));
            ui.pause(step_time);
        }

        ui.write_raw("\r\x1b[2K");
        Self::ok(
            ui,
            &format!("{task_name} ({target}/{target} {unit}) finished in {total_secs:.1}s"),
        );
    }

    pub fn render_task_spinner_job(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        speed_factor: f32,
    ) {
        let service = services.next(rng);
        let jobs = [
            format!("acquiring cluster write lock for {service}"),
            format!("verifying schema checksums across nodes in {service}"),
            format!("rebuilding bloom filters for {service}"),
            format!("pruning stale snapshot entries in {service}"),
        ];
        let job_name = &jobs[rng.below(jobs.len())];
        let elapsed = rng.range(6, 18) as f32 / 10.0;

        if !ui.animate {
            Self::ok(ui, &format!("{job_name} ({elapsed:.1}s)"));
            return;
        }

        let frames = [
            "\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}", "\u{2826}",
            "\u{2827}", "\u{2807}", "\u{280f}",
        ];
        let ticks = ((elapsed * 1000.0 * speed_factor) / 70.0).max(8.0) as usize;

        for i in 0..ticks {
            let frame = frames[i % frames.len()];
            ui.write_raw("\r\x1b[2K");
            ui.parts_inline(&[
                (ansi::ACCENT, &format!("  {frame} ")),
                (ansi::DIM, job_name),
                (ansi::DIM, " \u{2026}"),
            ]);
            ui.flush();
            ui.pause(70);
        }

        ui.write_raw("\r\x1b[2K");
        Self::ok(ui, &format!("{job_name} ({elapsed:.1}s)"));
    }

    pub fn render_task_multi_step_pipeline(
        &self,
        ui: &mut Ui,
        rng: &mut Rng,
        mut services: Deck<&'static str>,
        speed_factor: f32,
    ) {
        let service = services.next(rng);
        let pipeline = [
            (
                "fetching upstream dependencies",
                rng.range(3, 8) as f32 / 10.0,
            ),
            ("linking native extensions", rng.range(5, 12) as f32 / 10.0),
            ("deploying worker revisions", rng.range(4, 9) as f32 / 10.0),
        ];

        Self::header(ui, &format!("pipeline: release sync for {service}"));
        let frames = [
            "\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}",
        ];

        for (step_idx, (name, duration)) in pipeline.iter().enumerate() {
            let step_num = step_idx + 1;
            let prefix = format!("  [{step_num}/3] ");
            if !ui.animate {
                Self::ok(ui, &format!("[{step_num}/3] {name}"));
                continue;
            }

            let ticks = ((*duration * 1000.0 * speed_factor) / 60.0).max(6.0) as usize;
            for i in 0..ticks {
                let frame = frames[i % frames.len()];
                ui.write_raw("\r\x1b[2K");
                ui.parts_inline(&[
                    (ansi::ACCENT, &prefix),
                    (ansi::YELLOW, &format!("{frame} ")),
                    (ansi::DIM, name),
                ]);
                ui.flush();
                ui.pause(60);
            }

            ui.write_raw("\r\x1b[2K");
            ui.parts(&[
                (ansi::ACCENT, &prefix),
                (ansi::GREEN, "\u{2713} "),
                ("", name),
                (ansi::DIM, &format!(" ({duration:.1}s)")),
            ]);
            ui.flush();
            ui.pause(40);
        }
    }
}
