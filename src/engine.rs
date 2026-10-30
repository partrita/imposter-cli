//! Unified session engine — shared pacing, deadline, RNG loop for all styles.
//!
//! Each style provides a `Session` implementation defining its phase sequence.
//! The engine handles timing, deadline checking, and UI flushing.

use crate::ansi::Ui;
use crate::cli::Args;
use crate::config::Config;
use crate::rng::Rng;
use crate::style::Renderer;
use std::time::Instant;

/// A session phase: a unit of work that renders something and returns whether to continue.
pub trait Phase {
    fn render(
        &mut self,
        ui: &mut Ui,
        cfg: &Config,
        args: &Args,
        rng: &mut Rng,
        deadline: Option<Instant>,
    ) -> bool;
}

/// Session definition: a sequence of phases that repeat until deadline.
pub trait Session {
    /// Returns the next phase to execute, or None to restart the cycle.
    fn next_phase(&mut self) -> Option<&mut dyn Phase>;
    fn setup(&mut self, ui: &mut Ui, cfg: &Config, rng: &mut Rng);
    fn teardown(&mut self, ui: &mut Ui);
}

/// Run a session with the given style's renderer.
pub fn run_session<S: Session>(
    ui: &mut Ui,
    cfg: &Config,
    args: &Args,
    rng: &mut Rng,
    deadline: Option<Instant>,
    mut session: S,
) {
    let speed_factor = cfg.speed.delay_factor();

    session.setup(ui, cfg, rng);

    'session: loop {
        if crate::expired(deadline) {
            break;
        }

        // Run phases in a cycle
        loop {
            if crate::expired(deadline) {
                break 'session;
            }
            let Some(phase) = session.next_phase() else {
                // Cycle complete, restart
                break;
            };
            if !phase.render(ui, cfg, args, rng, deadline) {
                break 'session;
            }
            ui.flush();
            pacing_pause(ui, 900, speed_factor, rng);
        }
    }

    session.teardown(ui);
}

fn pacing_pause(ui: &mut Ui, base_ms: u64, speed_factor: f32, rng: &mut Rng) {
    let scaled = (base_ms as f32 * speed_factor) as u64;
    let jitter_range = (scaled as f32 * 0.3).round() as i64;
    let jitter = if jitter_range > 0 {
        rng.range((-jitter_range) as u64, jitter_range as u64) as i64 - jitter_range
    } else {
        0
    };
    let final_ms = ((scaled as i64 + jitter).max(15)) as u64;
    ui.pause(final_ms);
}

// Re-export transcript engine for AI styles
pub mod transcript {
    use super::*;
    use crate::content;
    use crate::jargon;
    use crate::rng::Deck;

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
}

// Hollywood Session implementation
pub mod hollywood_session {
    use super::*;
    use crate::style::hollywood::Hollywood;

    enum HollywoodPhase {
        SplitStream,
        FirewallBreach,
        MatrixWaterfall,
        AccessGranted,
        PacketSniff,
        BruteForceKeys,
        SatelliteScramble,
    }

    struct HollywoodPhaseImpl {
        phase: HollywoodPhase,
    }

    impl Phase for HollywoodPhaseImpl {
        fn render(
            &mut self,
            ui: &mut Ui,
            cfg: &Config,
            _args: &Args,
            rng: &mut Rng,
            _deadline: Option<Instant>,
        ) -> bool {
            let renderer = &Hollywood;
            let speed_factor = cfg.speed.delay_factor();
            match self.phase {
                HollywoodPhase::SplitStream => renderer.render_split_stream(ui, rng, speed_factor),
                HollywoodPhase::FirewallBreach => {
                    renderer.render_firewall_breach(ui, rng, speed_factor)
                }
                HollywoodPhase::MatrixWaterfall => {
                    renderer.render_matrix_waterfall(ui, rng, speed_factor)
                }
                HollywoodPhase::AccessGranted => {
                    renderer.render_access_granted(ui, rng, speed_factor)
                }
                HollywoodPhase::PacketSniff => renderer.render_packet_sniff(ui, rng, speed_factor),
                HollywoodPhase::BruteForceKeys => {
                    renderer.render_brute_force_keys(ui, rng, speed_factor)
                }
                HollywoodPhase::SatelliteScramble => renderer.alert(ui, rng),
            }
            true
        }
    }

    pub struct HollywoodSession {
        phases: Vec<HollywoodPhaseImpl>,
        phase_idx: usize,
    }

    impl HollywoodSession {
        pub fn new() -> Self {
            use HollywoodPhase::*;
            Self {
                phases: vec![
                    HollywoodPhaseImpl { phase: SplitStream },
                    HollywoodPhaseImpl {
                        phase: FirewallBreach,
                    },
                    HollywoodPhaseImpl {
                        phase: MatrixWaterfall,
                    },
                    HollywoodPhaseImpl {
                        phase: AccessGranted,
                    },
                    HollywoodPhaseImpl { phase: PacketSniff },
                    HollywoodPhaseImpl {
                        phase: BruteForceKeys,
                    },
                    HollywoodPhaseImpl {
                        phase: SatelliteScramble,
                    },
                ],
                phase_idx: 0,
            }
        }
    }

    impl Session for HollywoodSession {
        fn next_phase(&mut self) -> Option<&mut dyn Phase> {
            if self.phase_idx >= self.phases.len() {
                self.phase_idx = 0;
                return None; // Signal cycle complete
            }
            let phase = &mut self.phases[self.phase_idx] as &mut dyn Phase;
            self.phase_idx += 1;
            Some(phase)
        }

        fn setup(&mut self, ui: &mut Ui, cfg: &Config, rng: &mut Rng) {
            let renderer = &Hollywood;
            renderer.boot(ui, cfg, rng);
        }

        fn teardown(&mut self, ui: &mut Ui) {
            let renderer = &Hollywood;
            renderer.ended(ui);
        }
    }
}

// Classic Session implementation
pub mod classic_session {
    use super::*;
    use crate::rng::Deck;
    use crate::style::classic::Classic;

    enum ClassicPhase {
        Analyze,
        Metrics,
        Network,
        System,
        Processing,
        Alert,
        TaskEvent,
    }

    struct ClassicPhaseImpl {
        phase: ClassicPhase,
    }

    impl Phase for ClassicPhaseImpl {
        fn render(
            &mut self,
            ui: &mut Ui,
            cfg: &Config,
            args: &Args,
            rng: &mut Rng,
            _deadline: Option<Instant>,
        ) -> bool {
            let renderer = &Classic;
            let speed_factor = cfg.speed.delay_factor();
            let budget = crate::jargon::line_budget(cfg.complexity);
            match self.phase {
                ClassicPhase::Analyze => renderer.render_analyze(ui, rng, self.services(), budget),
                ClassicPhase::Metrics => renderer.render_metrics(ui, rng, budget),
                ClassicPhase::Network => renderer.render_network(ui, rng, self.services(), budget),
                ClassicPhase::System => renderer.render_system(ui, rng, budget),
                ClassicPhase::Processing => {
                    renderer.render_processing(ui, rng, self.services(), budget)
                }
                ClassicPhase::Alert => {
                    if args.alerts && rng.chance(1, 3) {
                        renderer.alert(ui, rng);
                    }
                }
                ClassicPhase::TaskEvent => {
                    if rng.chance(2, 5) {
                        let task_type = rng.range(0, 3);
                        match task_type {
                            0 => renderer.render_task_progress_bar(
                                ui,
                                rng,
                                self.services(),
                                speed_factor,
                            ),
                            1 => renderer.render_task_counter_stream(
                                ui,
                                rng,
                                self.services(),
                                speed_factor,
                            ),
                            2 => renderer.render_task_spinner_job(
                                ui,
                                rng,
                                self.services(),
                                speed_factor,
                            ),
                            _ => renderer.render_task_multi_step_pipeline(
                                ui,
                                rng,
                                self.services(),
                                speed_factor,
                            ),
                        }
                    }
                }
            }
            true
        }
    }

    impl ClassicPhaseImpl {
        fn services(&mut self) -> Deck<&'static str> {
            Deck::new(
                crate::style::classic::SERVICES,
                &mut crate::rng::Rng::new(None),
            )
        }
    }

    pub struct ClassicSession {
        phases: Vec<ClassicPhaseImpl>,
        phase_idx: usize,
    }

    impl ClassicSession {
        pub fn new() -> Self {
            use ClassicPhase::*;
            Self {
                phases: vec![
                    ClassicPhaseImpl { phase: Analyze },
                    ClassicPhaseImpl { phase: Metrics },
                    ClassicPhaseImpl { phase: Network },
                    ClassicPhaseImpl { phase: System },
                    ClassicPhaseImpl { phase: Processing },
                    ClassicPhaseImpl { phase: Alert },
                    ClassicPhaseImpl { phase: TaskEvent },
                ],
                phase_idx: 0,
            }
        }
    }

    impl Session for ClassicSession {
        fn next_phase(&mut self) -> Option<&mut dyn Phase> {
            if self.phase_idx >= self.phases.len() {
                self.phase_idx = 0;
                return None; // Signal cycle complete
            }
            let phase = &mut self.phases[self.phase_idx] as &mut dyn Phase;
            self.phase_idx += 1;
            Some(phase)
        }

        fn setup(&mut self, ui: &mut Ui, cfg: &Config, rng: &mut Rng) {
            let renderer = &Classic;
            renderer.boot(ui, cfg, rng);
        }

        fn teardown(&mut self, ui: &mut Ui) {
            let renderer = &Classic;
            renderer.ended(ui);
        }
    }
}
