//! The transcript engine.
//!
//! This module owns pacing, content decks and the shape of a turn. Anything
//! that decides *how* a line looks lives in `style::Renderer`; the engine only
//! says what happened ("searched for X", "read this file", "ran these tests").

use crate::ansi::Ui;
use crate::cli::Args;
use crate::config::Config;
use crate::content;
use crate::jargon;
use crate::rng::{Deck, Rng};
use std::time::Instant;

/// The shape of one turn. Varying the plan keeps the transcript from repeating
/// the same five lines in the same order forever.
#[derive(Copy, Clone, PartialEq)]
enum Step {
    Search,
    Grep,
    Read,
    Update,
    Test,
    Commit,
}

const PLANS: &[&[Step]] = &[
    &[Step::Search, Step::Read, Step::Update, Step::Test],
    &[Step::Read, Step::Update, Step::Test],
    &[
        Step::Grep,
        Step::Read,
        Step::Update,
        Step::Test,
        Step::Commit,
    ],
    &[Step::Read, Step::Read, Step::Update, Step::Test],
    &[Step::Search, Step::Read, Step::Test, Step::Update],
    &[
        Step::Read,
        Step::Search,
        Step::Update,
        Step::Test,
        Step::Commit,
    ],
];

struct Decks {
    prompts: Deck<&'static str>,
    intents: Deck<&'static str>,
    thoughts: Deck<&'static str>,
    searches: Deck<&'static str>,
    verbs: Deck<&'static str>,
    todos: Deck<&'static [&'static str]>,
    changes: Deck<(&'static [&'static str], &'static str)>,
    bash: Deck<(&'static str, &'static [&'static str])>,
    files: Deck<&'static str>,
    insights: Deck<&'static str>,
    plans: Deck<&'static [Step]>,
    hints: Deck<&'static str>,
}

impl Decks {
    fn new(cfg: &Config, rng: &mut Rng) -> Self {
        Self {
            prompts: Deck::new(content::PROMPTS, rng),
            intents: Deck::new(content::INTENTS, rng),
            thoughts: Deck::new(content::THOUGHTS, rng),
            searches: Deck::new(content::SEARCHES, rng),
            verbs: Deck::new(content::VERBS, rng),
            todos: Deck::new(content::TODOS, rng),
            changes: Deck::new(content::CHANGES, rng),
            bash: Deck::new(content::BASH_RUNS, rng),
            files: Deck::new(content::files(cfg.dev_type), rng),
            insights: Deck::new(jargon::lines(cfg.jargon), rng),
            plans: Deck::new(PLANS, rng),
            hints: Deck::new(cfg.style.skin().hints, rng),
        }
    }
}

fn pacing_pause(ui: &mut Ui, base_ms: u64, speed_factor: f32, rng: &mut Rng) {
    let scaled = (base_ms as f32 * speed_factor) as u64;
    // Add jitter: ±30% variation with a 15ms floor
    let jitter_range = (scaled as f32 * 0.3).round() as i64;
    let jitter = if jitter_range > 0 {
        rng.range((-jitter_range) as u64, jitter_range as u64) as i64 - jitter_range
    } else {
        0
    };
    let final_ms = ((scaled as i64 + jitter).max(15)) as u64;
    ui.pause(final_ms);
}

pub fn run(ui: &mut Ui, cfg: &Config, args: &Args, rng: &mut Rng, deadline: Option<Instant>) {
    let renderer = cfg.style.renderer();
    let skin = renderer.skin();
    let mut decks = Decks::new(cfg, rng);
    renderer.boot(ui, cfg, rng);

    let speed_factor = cfg.speed.delay_factor();
    let effective_model = cfg.model.as_deref().unwrap_or(skin.model);

    let mut tokens: u32 = rng.range(380, 940) as u32;
    let mut turns: u32 = 0;

    'session: while !crate::expired(deadline) {
        ui.refresh_width();
        turns += 1;
        let prompt = decks.prompts.next(rng);
        renderer.prompt(ui, prompt);
        pacing_pause(ui, 150, speed_factor, rng);

        renderer.assistant(ui, decks.intents.next(rng));
        pacing_pause(ui, 120, speed_factor, rng);

        // Sometimes show the tool's own thinking word, sometimes a varied verb,
        // so the spinner line itself never becomes a fixed signature.
        let verb = if rng.chance(1, 3) {
            skin.thinking
        } else {
            decks.verbs.next(rng)
        };
        ui.blank();
        let spin_dur = 1.05 * speed_factor;
        ui.spinner(
            skin.accent,
            verb,
            spin_dur,
            tokens,
            renderer.spinner_kind(),
            effective_model,
        );
        ui.flush();
        if crate::expired(deadline) {
            break;
        }

        // Show thinking stream animation (1-3 thought sentences)
        let thought_text = decks.thoughts.next(rng);
        renderer.thought(ui, thought_text);
        pacing_pause(ui, 100, speed_factor, rng);

        let plan = decks.plans.next(rng);
        let mut file = decks.files.next(rng);
        let mut summary = "";

        for step in plan {
            if crate::expired(deadline) {
                break 'session;
            }
            match *step {
                Step::Search => {
                    let query = decks.searches.next(rng);
                    let found = format!("Found {} files", rng.range(4, 41));
                    renderer.search(ui, query, &found);
                    tokens += rng.range(80, 260) as u32;
                }
                Step::Grep => {
                    let query = decks.searches.next(rng);
                    let found =
                        format!("{} matches in {} files", rng.range(3, 40), rng.range(1, 9));
                    renderer.grep(ui, query, &found);
                    tokens += rng.range(60, 200) as u32;
                }
                Step::Read => {
                    let start = rng.range(1, 380);
                    let len = rng.range(40, 220);
                    renderer.read(ui, file, start, len);
                    tokens += rng.range(300, 1400) as u32;
                    file = decks.files.next(rng);
                }
                Step::Update => {
                    let (diff, text) = decks.changes.next(rng);
                    let adds = diff.iter().filter(|l| l.starts_with('+')).count();
                    let rems = diff.iter().filter(|l| l.starts_with('-')).count();
                    renderer.update(ui, file, adds, rems);
                    renderer.patch(ui, diff, rng.range(18, 140));
                    summary = text;
                    tokens += rng.range(250, 800) as u32;
                    pacing_pause(ui, 90, speed_factor, rng);
                }
                Step::Test => {
                    let (cmd, out) = decks.bash.next(rng);
                    renderer.bash(ui, cmd, out);
                    tokens += rng.range(120, 420) as u32;
                }
                Step::Commit => {
                    let message = decks.insights.next(rng);
                    let cmd = format!("git commit -am \"{message}\"");
                    let out = format!("[main {:07x}] {message}", rng.next_u64() & 0xfff_ffff);
                    renderer.tool(ui, "Bash", &cmd, &out);
                    pacing_pause(ui, 250, speed_factor, rng);
                }
            }
            pacing_pause(ui, 55, speed_factor, rng);
        }

        if cfg.complexity != crate::types::Complexity::Low && turns % 2 == 1 {
            let items = decks.todos.next(rng);
            renderer.todo(ui, items);
            pacing_pause(ui, 160, speed_factor, rng);
        }

        if args.alerts && rng.chance(1, 2) {
            ui.blank();
            renderer.alert(ui, rng);
        }

        if !summary.is_empty() {
            ui.blank();
            renderer.assistant(ui, summary);
        }
        tokens += rng.range(300, 1100) as u32;
        renderer.footer(ui, rng.range(41, 220), tokens);
        if renderer.hints() {
            ui.blank();
            ui.parts(&[
                (crate::ansi::DIM, "  "),
                (crate::ansi::DIM, decks.hints.next(rng)),
            ]);
        }
        ui.blank();
        ui.flush();
        pacing_pause(ui, 900, speed_factor, rng);
    }

    ui.blank();
    renderer.ended(ui);
}
