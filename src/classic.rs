use crate::ansi::{self, Ui};
use crate::cli::Args;
use crate::config::Config;
use crate::content;
use crate::jargon;
use crate::rng::{Deck, Rng};
use std::time::Instant;

const SERVICES: &[&str] = &[
    "ingest-gateway",
    "billing-core",
    "session-store",
    "event-relay",
    "search-index",
    "notify-worker",
    "auth-proxy",
    "media-pipeline",
];

pub fn run(ui: &mut Ui, cfg: &Config, args: &Args, rng: &mut Rng, deadline: Option<Instant>) {
    boot(ui, cfg);
    let budget = jargon::line_budget(cfg.complexity);
    let mut insights = Deck::new(jargon::lines(cfg.jargon), rng);
    let mut services = Deck::new(SERVICES, rng);
    let mut step = 0u32;

    while !crate::expired(deadline) {
        step += 1;
        match step % 5 {
            1 => analyze(ui, rng, &mut services, budget),
            2 => metrics(ui, rng, budget),
            3 => network(ui, rng, &mut services, budget),
            4 => system(ui, rng, budget),
            _ => processing(ui, rng, &mut services, budget),
        }

        ui.parts(&[(ansi::DIM, "  \u{203a} "), (ansi::DIM, insights.next(rng))]);
        ui.pause(90);

        if args.alerts && rng.chance(1, 3) {
            alert(ui, rng);
        }
        ui.blank();
        ui.flush();
        ui.pause(600);
    }

    ui.parts(&[(ansi::DIM, "session terminated.")]);
}

fn boot(ui: &mut Ui, cfg: &Config) {
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
    for pct in [18u32, 41, 67, 88, 100] {
        let filled = (pct as usize * width) / 100;
        let bar = format!(
            "{}{}",
            "\u{2588}".repeat(filled),
            "\u{2591}".repeat(width - filled)
        );
        ui.parts(&[
            (ansi::GREEN, &bar),
            (ansi::DIM, &format!(" {pct:>3}%")),
            ("", &format!("  {}", boot_label(pct))),
        ]);
        ui.pause(110);
    }
    ui.blank();
}

fn boot_label(pct: u32) -> &'static str {
    match pct {
        18 => "resolving modules",
        41 => "warming caches",
        67 => "linking artifacts",
        88 => "checking invariants",
        _ => "ready",
    }
}

fn header(ui: &mut Ui, name: &str) {
    ui.parts(&[(ansi::ACCENT, "\u{25b8} "), (ansi::BOLD, name)]);
}

fn ok(ui: &mut Ui, text: &str) {
    ui.parts(&[(ansi::GREEN, "  \u{2713} "), ("", text)]);
    ui.pause(55);
}

fn warn(ui: &mut Ui, text: &str) {
    ui.parts(&[(ansi::YELLOW, "  ! "), ("", text)]);
    ui.pause(55);
}

fn info(ui: &mut Ui, text: &str) {
    ui.parts(&[(ansi::DIM, "  \u{00b7} "), (ansi::DIM, text)]);
    ui.pause(40);
}

fn analyze(ui: &mut Ui, rng: &mut Rng, services: &mut Deck<&'static str>, budget: usize) {
    header(ui, "code analysis");
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
        ok(ui, line);
    }
}

fn metrics(ui: &mut Ui, rng: &mut Rng, budget: usize) {
    header(ui, "runtime metrics");
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
    for line in lines.iter().take(budget) {
        if line.starts_with("error budget") {
            info(ui, line);
        } else {
            ok(ui, line);
        }
    }
}

fn network(ui: &mut Ui, rng: &mut Rng, services: &mut Deck<&'static str>, budget: usize) {
    header(ui, "network");
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
            warn(ui, line);
        } else {
            ok(ui, line);
        }
    }
}

fn system(ui: &mut Ui, rng: &mut Rng, budget: usize) {
    header(ui, "system");
    let lines = [
        format!("cpu    {}% of 8 cores", rng.range(6, 74)),
        format!("memory {} MB resident", rng.range(38, 720)),
        format!("disk   {} MB written", rng.range(2, 180)),
        format!("fds    {} open", rng.range(20, 900)),
        format!("{} voluntary context switches/s", rng.range(200, 9000)),
        format!("gc pause p99 {} ms", rng.range(0, 12)),
    ];
    for line in lines.iter().take(budget) {
        ok(ui, line);
    }
    info(ui, "no swap pressure");
}

fn processing(ui: &mut Ui, rng: &mut Rng, services: &mut Deck<&'static str>, budget: usize) {
    header(ui, "data processing");
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
        ok(ui, line);
    }
}

fn alert(ui: &mut Ui, rng: &mut Rng) {
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
    info(ui, "auto-remediation: scaled pool 20 -> 32");
}
