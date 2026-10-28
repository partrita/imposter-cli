use crate::types::DevelopmentType;

/// The fictional workspace every session claims to be in. This used to be
/// configurable with `--project`; it is fixed now.
pub const PROJECT: &str = "distributed-cluster";

pub const PROMPTS: &[&str] = &[
    "refactor the payment service and add retry tests",
    "why is p99 latency up since the last deploy?",
    "add structured logging to the ingest pipeline",
    "migrate the queue consumers to the new client",
    "this test is flaky on CI, fix it",
    "make the build faster, it takes 9 minutes",
    "add a healthcheck endpoint and wire it into k8s",
    "clean up config loading, it's a mess",
    "the login flow silently fails for SSO users",
    "write a migration for the new billing table",
    "cache the /search endpoint without breaking invalidation",
    "why does the container use 2 GB of RAM?",
    "split the monolith router into modules",
    "add rate limiting to the public API",
    "update the dependency tree, we're 40 versions behind",
    "fix the race condition in the worker pool",
    "add observability around the payment webhook",
    "reduce the Docker image size",
    "port the settings page to the new design tokens",
    "remove the deprecated v1 endpoints",
    "the webhook retries are duplicating charges, fix that",
    "add pagination to the admin list endpoints",
    "the SSO callback drops the state parameter",
    "instrument the worker so we can see queue lag",
    "replace the hand-rolled date parser with the std one",
    "make the integration suite run against a real database",
    "the upload endpoint buffers the whole file in memory",
    "split the feature flags into per-environment files",
    "add a graceful shutdown path to the server",
    "the CLI help text is out of date, regenerate it",
];

pub const INTENTS: &[&str] = &[
    "I'll start by reading the current implementation.",
    "Let me look at the failing path first.",
    "I'll trace where that number comes from.",
    "First, let me see how this is wired up.",
    "Let me check the tests before changing anything.",
    "I'll map the call sites first.",
    "I found the relevant module. Reading it now.",
    "Let me reproduce that locally before touching anything.",
    "I'll follow the request from the handler down.",
    "There are two paths here; checking which one runs in production.",
    "Reading the module and its tests together.",
    "Let me find the owner of this code path.",
    "I want to see the diff of the last deploy first.",
    "I'll start with the smallest change that proves the theory.",
    "Grabbing the relevant symbols before I edit.",
    "Let me check the schema before writing the query.",
    "I'll verify the assumption with a quick grep.",
    "The error message points here, confirming it now.",
    "Let me see how the sibling module solved this.",
    "I'll inspect the config precedence order.",
    "Tracing the value backwards from where it breaks.",
];

pub const THOUGHTS: &[&str] = &[
    "The caller expects idempotency here. If the retry triggers before the state machine transitions, we could spawn duplicate jobs. Let's trace the worker lifecycle and inspect concurrency locks.",
    "Analyzing recent regression traces... The connection pool is exhausted during peak spikes because keep-alive sockets aren't releasing properly under backpressure.",
    "Checking AST structure and symbol definitions. We need to ensure zero-cost abstraction without introducing unnecessary heap allocations in the critical path.",
    "The schema migration needs to be backward-compatible with older replicas. Adding an index concurrently prevents locking the primary table during traffic.",
    "Examining call graph: handler -> middleware -> repository. The authorization context might be dropped when propagating through async boundaries.",
    "Evaluating edge cases: empty payload, partial chunk streaming, and socket reset mid-transfer. Adding defensive bounds checks here will prevent panics.",
    "Tracing downstream queue lag. The batch size is fixed at 100, which starves consumer threads when individual payload sizes exceed 256KB.",
    "Let's review the memory profile: heap allocations spike during decompression. Streaming the raw buffer directly into the serializer should mitigate memory pressure.",
    "Checking test suite fixtures. The mock server was returning HTTP 200 with an empty body instead of simulating realistic network timeouts.",
    "Verifying invariants: state must remain atomic across thread boundaries. Using a compare-and-swap loop here guarantees thread safety without lock contention.",
];

pub const SEARCHES: &[&str] = &[
    "payment",
    "retry",
    "config",
    "rate limit",
    "webhook",
    "worker",
    "billing",
    "sso",
    "cache",
    "healthz",
    "metrics",
    "timeout",
];

pub const VERBS: &[&str] = &[
    "Noodling",
    "Percolating",
    "Cogitating",
    "Reticulating",
    "Simmering",
    "Nebulizing",
    "Vibing",
    "Clauding",
];

pub const TODOS: &[&[&str]] = &[
    &[
        "Read the current retry path",
        "Add backoff with jitter",
        "Cover the 503 path with a test",
    ],
    &[
        "Trace the slow query",
        "Add a composite index",
        "Verify p99 on staging",
    ],
    &[
        "Replace the global config",
        "Wire the new loader into main",
        "Delete the old defaults",
    ],
    &[
        "Extract the router",
        "Update imports across 14 files",
        "Run the integration suite",
    ],
    &[
        "Add the cache layer",
        "Handle invalidation on write",
        "Measure hit rate in staging",
    ],
    &[
        "Locate the hot path",
        "Move the allocation out of the loop",
        "Benchmark before and after",
    ],
    &[
        "Reproduce the flake in a loop",
        "Identify the shared state",
        "Isolate it per test",
    ],
    &[
        "Read the audit output",
        "Bump the vulnerable crate",
        "Re-run the audit",
    ],
    &[
        "Map the existing flags",
        "Add the new config key",
        "Document the default",
    ],
    &[
        "Find the unbounded query",
        "Add pagination",
        "Backfill the frontend caller",
    ],
    &[
        "Write the failing test",
        "Make it pass",
        "Refactor the duplication",
    ],
    &[
        "Profile the endpoint",
        "Remove the N+1 fetch",
        "Verify the query count",
    ],
    &[
        "Extract the interface",
        "Swap the implementation",
        "Delete the old adapter",
    ],
];

pub const CHANGES: &[(&[&str], &str)] = &[
    (DIFF_RETRY, "Done. Backoff now uses exponential delays with jitter, capped at 5 attempts. The 503 path is covered by two new tests."),
    (DIFF_LOG, "Done. Request handling is now traced with a span that carries request, route, user and tenant. Logs are structured, so the log query actually works."),
    (DIFF_CACHE, "Added the cache with a 60 second TTL and invalidation on write. Hit rate should land around 80% given the access pattern."),
    (DIFF_FLAGS, "Config now loads once at startup and is injected explicitly. Removed the hidden defaults."),
    (DIFF_TEST, "The flake was a race on the shared fixture. Tests now get their own transaction and run in parallel safely."),
    (DIFF_ASYNC, "The worker pool no longer holds the lock across await points. Notification is explicit, so the race is gone."),
    (DIFF_SQL, "Query now filters on status before the interval scan and hits the new composite index. p99 down to 38 ms locally."),
    (DIFF_PAGE, "Pagination is clamped on the server now. page=0 and page_size=10000 both behave, and the offset can no longer go negative."),
    (DIFF_ERR, "Upstream failures now map to a typed error with a stable code instead of a stringly-typed 500. The client gets something it can act on."),
    (DIFF_DEPS, "Bumped serde and tokio to the latest patch line. No API changes; the audit warnings are gone."),
    (DIFF_HEALTH, "Added /healthz with a database ping and a 2 second timeout. k8s now has a liveness and readiness probe."),
    (DIFF_RATE, "Public routes now share a token bucket per API key. Bursts of 20 are allowed; the rest get a 429 with Retry-After."),
    (DIFF_SSO, "The state parameter was dropped on the redirect. It is now signed, stored for 10 minutes, and verified on callback."),
    (DIFF_MONO, "Split the router into four modules with one responsibility each. No behavior change; the diff is mostly moves."),
    (DIFF_IMG, "Switched to a distroless runtime stage and dropped the build cache. Image is down from 1.9 GB to 210 MB."),
    (DIFF_OBS, "The queue now exports depth, age and processing time. Added one dashboard and two alerts for sustained lag."),
    (DIFF_MEM, "The upload path streams chunks instead of buffering. Peak RSS on a 400 MB upload dropped to 38 MB."),
    (DIFF_SHUTDOWN, "The server now drains connections on SIGTERM with a 20 second grace period before exiting."),
];

pub const BASH_RUNS: &[(&str, &[&str])] = &[
    (
        "cargo test --lib",
        &[
            "running 42 tests",
            "test result: ok. 42 passed; 0 failed; 0 ignored",
            "finished in 1.83s",
        ],
    ),
    (
        "cargo clippy --all-targets -- -D warnings",
        &[
            "Finished `dev` profile [unoptimized + debuginfo] target(s)",
            "0 warnings, 0 errors",
        ],
    ),
    (
        "npm test -- --run",
        &[
            "Test Files  14 passed (14)",
            "Tests  128 passed (128)",
            "Duration  6.42s",
        ],
    ),
    (
        "pnpm build",
        &[
            "vite v6.0.3 building for production...",
            "\u{2713} 1284 modules transformed.",
            "dist/assets/index.js  412.7 kB \u{2502} gzip: 128.4 kB",
        ],
    ),
    ("pytest -q", &["128 passed, 4 skipped in 12.4s"]),
    (
        "go test ./...",
        &[
            "ok  github.com/acme/api/internal/payment  2.104s",
            "ok  github.com/acme/api/internal/queue    0.881s",
        ],
    ),
    (
        "git diff --stat",
        &["4 files changed, 61 insertions(+), 18 deletions(-)"],
    ),
    (
        "cargo test --all-features",
        &[
            "running 128 tests",
            "test result: ok. 128 passed; 0 failed",
            "finished in 4.10s",
        ],
    ),
    ("cargo fmt --check", &["no formatting changes needed"]),
    (
        "cargo nextest run",
        &[
            "Summary  212 tests run: 212 passed, 0 skipped",
            "Finished in 3.482s",
        ],
    ),
    (
        "pnpm test -- --coverage",
        &["Tests  128 passed (128)", "Lines  94.2% | Functions 91.8%"],
    ),
    ("ruff check .", &["All checks passed!"]),
    ("make lint", &["lint: ok (eslint, prettier, tsc --noEmit)"]),
    (
        "docker build -t app:dev .",
        &[
            "=> exporting to image",
            "=> => naming to docker.io/library/app:dev",
        ],
    ),
    (
        "cargo audit",
        &["0 vulnerabilities found in 214 dependencies"],
    ),
    (
        "sqlx migrate run",
        &[
            "Applied 0007_billing (12.4ms)",
            "Applied 0008_indexes (8.1ms)",
        ],
    ),
    (
        "kubectl rollout status deploy/api",
        &["deployment \"api\" successfully rolled out"],
    ),
];

pub const DIFF_RETRY: &[&str] = &[
    "- const MAX_RETRIES: u32 = 3;",
    "+ const MAX_RETRIES: u32 = 5;",
    "+ const BASE_DELAY_MS: u64 = 120;",
    "",
    " pub async fn send(&self, req: Request) -> Result<Response> {",
    "-     self.client.post(req.url).send().await",
    "+     let backoff = BASE_DELAY_MS * 2u64.pow(attempt);",
    "+     tokio::time::sleep(Duration::from_millis(backoff)).await;",
    "+     self.client.post(req.url).timeout(self.timeout).send().await",
    " }",
];

pub const DIFF_LOG: &[&str] = &[
    "- println!(\"handling request {id}\");",
    "+ tracing::info!(request_id = %id, route = %path, \"handling request\");",
    "",
    " let span = Span::current();",
    "- span.record(\"user\", &user_id);",
    "+ span.record(\"user_id\", &user_id);",
    "+ span.record(\"tenant\", &tenant_id);",
];

pub const DIFF_CACHE: &[&str] = &[
    "- let results = db.query(&sql).await?;",
    "+ let key = cache_key(&sql, &params);",
    "+ if let Some(hit) = cache.get(&key).await { return Ok(hit); }",
    "+ let results = db.query(&sql).await?;",
    "+ cache.set(&key, &results, 60).await;",
    " Ok(results)",
];

pub const DIFF_FLAGS: &[&str] = &[
    "- if config.debug {",
    "+ if cfg!(debug_assertions) {",
    "     eprintln!(\"verbose output enabled\");",
    " }",
    "",
    "- let workers = 4;",
    "+ let workers = std::env::var(\"WORKERS\").ok()",
    "+     .and_then(|v| v.parse().ok())",
    "+     .unwrap_or_else(default_workers);",
];

pub const DIFF_TEST: &[&str] = &[
    "- #[test]",
    "- fn parses_empty_input() {",
    "+ #[test]",
    "+ fn parses_empty_input() -> Result<()> {",
    "     let got = parse(\"\")?;",
    "-     assert_eq!(got.len(), 0);",
    "+     assert!(got.is_empty());",
    "+     Ok(())",
    " }",
];

pub const DIFF_ASYNC: &[&str] = &[
    "- fn handle(&self) -> Result<()> {",
    "-     self.inner.lock().unwrap().push(item);",
    "+ async fn handle(&self, item: Item) -> Result<()> {",
    "+     let mut guard = self.inner.lock().await;",
    "+     guard.push(item);",
    "+     drop(guard);",
    "+     self.notify.notify_one();",
    "     Ok(())",
    " }",
];

pub const DIFF_SQL: &[&str] = &[
    "- SELECT id, total FROM invoices WHERE created_at > now() - interval '1 day';",
    "+ SELECT id, total",
    "+ FROM invoices",
    "+ WHERE created_at > now() - interval '1 day'",
    "+   AND status = 'paid';",
];

/// A fake failing lint, and the diff that "fixes" it.
pub const LINT_ERROR: &[&str] = &[
    "error: this `if` has identical blocks",
    "  --> src/api/handlers.rs:58:9",
    "   |",
    "58 |     if attempt > MAX_RETRIES {",
    "   |     ^^^^^^^^^^^^^^^^^^^^^^^^",
];

pub const LINT_FIX: &[&str] = &[
    "-     if attempt > MAX_RETRIES {",
    "-         return Err(Error::Exhausted);",
    "-     }",
    "-     if attempt > MAX_RETRIES {",
    "-         return Err(Error::Exhausted);",
    "+     if attempt > MAX_RETRIES {",
    "+         return Err(Error::Exhausted);",
    "+     }",
];

pub fn files(dev: DevelopmentType) -> &'static [&'static str] {
    match dev {
        DevelopmentType::Backend => &[
            "src/api/handlers.rs",
            "src/payment/service.rs",
            "src/queue/worker.rs",
            "src/config/settings.rs",
            "migrations/0007_billing.sql",
            "src/auth/middleware.rs",
            "src/billing/invoice.rs",
            "src/events/consumer.rs",
            "src/http/router.rs",
            "src/db/pool.rs",
        ],
        DevelopmentType::Frontend => &[
            "src/components/Checkout.tsx",
            "src/hooks/useCart.ts",
            "src/api/client.ts",
            "src/styles/tokens.css",
            "src/components/DataTable.tsx",
            "src/app/layout.tsx",
            "src/lib/format.ts",
            "src/hooks/useSession.ts",
        ],
        DevelopmentType::Fullstack => &[
            "src/server/routes.ts",
            "src/app/page.tsx",
            "src/lib/session.ts",
            "src/db/schema.prisma",
            "src/server/middleware.ts",
            "src/app/settings/page.tsx",
            "src/lib/billing.ts",
            "src/db/migrations/0004_teams.sql",
        ],
        DevelopmentType::DataScience => &[
            "pipelines/ingest.py",
            "dbt/models/revenue.sql",
            "src/features/build.py",
            "notebooks/cohort.ipynb",
            "pipelines/enrich.py",
            "dbt/models/churn.sql",
            "src/features/encode.py",
            "notebooks/ab_test.ipynb",
        ],
        DevelopmentType::DevOps => &[
            "infra/main.tf",
            "k8s/deployment.yaml",
            ".github/workflows/deploy.yml",
            "docker/Dockerfile",
            "infra/vpc.tf",
            "k8s/ingress.yaml",
            ".github/workflows/test.yml",
            "docker/compose.yaml",
        ],
        DevelopmentType::Blockchain => &[
            "contracts/Vault.sol",
            "contracts/test/Vault.t.sol",
            "scripts/deploy.ts",
            "src/lib/abi.ts",
            "contracts/Governor.sol",
            "contracts/test/Governor.t.sol",
            "scripts/verify.ts",
            "src/lib/signer.ts",
        ],
        DevelopmentType::MachineLearning => &[
            "train/loop.py",
            "model/attention.py",
            "data/loader.py",
            "serving/infer.py",
            "train/scheduler.py",
            "model/norm.py",
            "data/augment.py",
            "eval/metrics.py",
        ],
        DevelopmentType::SystemsProgramming => &[
            "src/alloc/arena.rs",
            "src/net/poll.rs",
            "src/sync/mutex.rs",
            "benches/throughput.rs",
            "src/mm/slab.rs",
            "src/io/uring.rs",
            "src/task/waker.rs",
            "benches/latency.rs",
        ],
        DevelopmentType::GameDevelopment => &[
            "src/render/pipeline.rs",
            "src/physics/collide.rs",
            "src/ecs/world.rs",
            "shaders/post.wgsl",
            "src/audio/mixer.rs",
            "src/input/buffer.rs",
            "src/assets/loader.rs",
            "shaders/sky.wgsl",
        ],
        DevelopmentType::Security => &[
            "src/auth/token.rs",
            "src/crypto/kdf.rs",
            "src/audit/log.rs",
            "policies/iam.json",
            "src/auth/session.rs",
            "src/crypto/nonce.rs",
            "src/net/firewall.rs",
            "policies/audit.rego",
        ],
    }
}

pub const DIFF_HEALTH: &[&str] = &[
    "+ #[get(\"/healthz\")]",
    "+ async fn healthz(db: Data<Pool>) -> Status {",
    "+     match tokio::time::timeout(Duration::from_secs(2), db.ping()).await {",
    "+         Ok(Ok(_)) => Status::Ok,",
    "+         _ => Status::ServiceUnavailable,",
    "+     }",
    "+ }",
];

pub const DIFF_RATE: &[&str] = &[
    "- let allowed = true;",
    "+ let allowed = limiter.check(&api_key, 20, Duration::from_secs(60)).await;",
    "+ if !allowed {",
    "+     return Err(ApiError::too_many_requests(retry_after));",
    "+ }",
];

pub const DIFF_SSO: &[&str] = &[
    "- let url = format!(\"{}/authorize?client_id={}\", base, id);",
    "+ let state = sign_state(&session_id, now)?;",
    "+ let url = format!(\"{}/authorize?client_id={}&state={}\", base, id, state);",
    "",
    " fn callback(params: Params) -> Result<Session> {",
    "-     let code = params.code;",
    "+     verify_state(&params.state, &session_id, now)?;",
    "+     let code = params.code;",
];

pub const DIFF_MONO: &[&str] = &[
    "- mod routes;",
    "+ mod routes {",
    "+     pub mod payments;",
    "+     pub mod accounts;",
    "+     pub mod admin;",
    "+     pub mod webhooks;",
    "+ }",
];

pub const DIFF_IMG: &[&str] = &[
    "- FROM rust:1.83 AS build",
    "+ FROM rust:1.83-slim AS build",
    "- FROM debian:bookworm",
    "+ FROM gcr.io/distroless/cc-debian12",
    "- COPY --from=build /app/target/release/app /app",
    "+ COPY --from=build /app/target/release/app /app",
];

pub const DIFF_OBS: &[&str] = &[
    "+ queue_depth.set(pending as i64);",
    "+ queue_age.record(oldest.elapsed().as_secs_f64());",
    "+ let timer = processing_seconds.start_timer();",
    "  let job = queue.pop().await?;",
    "+ drop(timer);",
];

pub const DIFF_MEM: &[&str] = &[
    "- let body = req.bytes().await?;",
    "- let parsed = parse_upload(&body)?;",
    "+ let mut stream = req.into_stream();",
    "+ while let Some(chunk) = stream.next().await {",
    "+     writer.write_all(&chunk?).await?;",
    "+ }",
];

pub const DIFF_SHUTDOWN: &[&str] = &[
    "- server.await?;",
    "+ let signal = shutdown_signal();",
    "+ tokio::select! {",
    "+     _ = server => {},",
    "+     _ = signal => {",
    "+         graceful.shutdown(Duration::from_secs(20)).await;",
    "+     }",
    "+ }",
];

pub const DIFF_PAGE: &[&str] = &[
    "  let page = query.page.unwrap_or(1);",
    "+ let page = query.page.unwrap_or(1).max(1);",
    "+ let page_size = query.page_size.unwrap_or(20).clamp(1, 100);",
    "  let offset = (page - 1) * page_size;",
];

pub const DIFF_ERR: &[&str] = &[
    "-     return Err(anyhow::anyhow!(\"failed\"));",
    "+     return Err(ApiError::upstream(",
    "+         \"payment provider rejected the request\",",
    "+     ));",
];

pub const DIFF_DEPS: &[&str] = &[
    "- serde = \"1.0.180\"",
    "+ serde = \"1.0.229\"",
    "- tokio = \"1.38\"",
    "+ tokio = \"1.47\"",
];
