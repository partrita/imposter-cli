use crate::types::JargonLevel;

/// Restored from rust-stakeholder: the higher the level, the less it means.
const LOW: &[&str] = &[
    "cleaned up the module boundary",
    "moved the validation next to the parser",
    "removed a duplicated branch",
    "added a regression test",
    "renamed things so the next person has a chance",
    "made the error message say what actually happened",
];

const MEDIUM: &[&str] = &[
    "decoupled the retry policy from the transport layer",
    "introduced an explicit state machine for the handshake",
    "replaced the polling loop with an event-driven path",
    "collapsed three config sources into one loader",
    "moved the hot path off the request thread",
    "added structured spans around the payment webhook",
];

const HIGH: &[&str] = &[
    "implemented an idempotent, back-pressure-aware ingestion boundary",
    "introduced a zero-copy projection over the write-ahead log",
    "replaced the global lock with a sharded, lock-free ring buffer",
    "adopted a capability-based access model across the service mesh",
    "made the scheduler work-conserving under adversarial skew",
    "reified the domain invariants as a typestate machine",
];

const EXTREME: &[&str] = &[
    "implemented non-euclidean topology optimization for multi-dimensional data representation",
    "leveraged a quantum-inspired annealing heuristic over the dependency DAG",
    "built an eventually-consistent, self-healing consensus substrate for the config plane",
    "applied category-theoretic functors to the serialization boundary",
    "introduced a homomorphic cache with speculative invalidation at the edge",
    "migrated the whole stack to a hyper-converged, event-sourced, zero-trust architecture",
];

pub fn lines(level: JargonLevel) -> &'static [&'static str] {
    match level {
        JargonLevel::Low => LOW,
        JargonLevel::Medium => MEDIUM,
        JargonLevel::High => HIGH,
        JargonLevel::Extreme => EXTREME,
    }
}

/// How many lines of activity a given complexity level implies.
pub fn line_budget(complexity: crate::types::Complexity) -> usize {
    match complexity {
        crate::types::Complexity::Low => 2,
        crate::types::Complexity::Medium => 3,
        crate::types::Complexity::High => 4,
        crate::types::Complexity::Extreme => 6,
    }
}
