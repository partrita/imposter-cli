# imposter

**아무것도 하지 않으면서, Claude Code로 바이브 코딩하는 것처럼 보이는 CLI.**

터미널에 가짜 Claude Code 세션을 그려줍니다. 파일은 하나도 바뀌지 않고, 토큰도 한 개도 쓰지 않습니다.
사람들이 지나갈 때 화면만 보면 "저 사람 뭐 대단한 걸 하고 있네"가 됩니다.

> **SATIRE ALERT** — 농담 프로젝트입니다. 실제로 무언가를 해내는 척하는 도구이고, 그 이상은 아닙니다.

```
╭─── ✻ Claude Code v2.1.76 ──────────────────────────────────────────────╮
│         Welcome back!          │ Tips for getting started              │
│                                │ Run /init to create a CLAUDE.md       │
│            ▐▛███▜▌             │ Use @ to mention files in chat        │
│           ▝▜█████▛▘            │ ───────────────────────────────────── │
│             ▘▘ ▝▝              │ Recent activity                       │
│                                │ No recent activity                    │
│     Opus 4.6 · Claude Max      │ /resume for previous sessions         │
│   ~/code/distributed-cluster   │ /help for commands · ! for bash       │
╰────────────────────────────────────────────────────────────────────────╯

──────────────────────────────────────────────────────────────────────────
❯ refactor the payment service and add retry tests
──────────────────────────────────────────────────────────────────────────

⏺ I'll start by reading the current implementation.
⏺ Search(pattern: "payment", path: "src")
  ⎿  Found 5 files (ctrl+o to expand)
⏺ Read(src/payment/service.rs)
  ⎿  Read 194 lines (ctrl+o to expand)

✻ Simmering… (esc to interrupt · 4s · ↓ 2.1k tokens)

⏺ Update(src/payment/service.rs)
  ⎿  Updated src/payment/service.rs with 5 additions and 2 removals
       52 -  const MAX_RETRIES: u32 = 3;
       52 +  const MAX_RETRIES: u32 = 5;
       53 +  const BASE_DELAY_MS: u64 = 120;
       55    pub async fn send(&self, req: Request) -> Result<Response> {
⏺ Bash(cargo test --lib)
  ⎿  running 42 tests
     test result: ok. 42 passed; 0 failed; 0 ignored

⏺ Done. Backoff now uses exponential delays with jitter, capped at 5 attempts.
✻ Worked for 1m 12s · 3.4k tokens · $0.03

  ⏵⏵ accept edits on (shift+tab to cycle)
```

---

## 이번 개편에서 바뀐 것

| 항목 | 이전 (`rust-stakeholder`) | 지금 (`imposter`) |
| --- | --- | --- |
| 이름 | rust-stakeholder | **imposter** |
| 화면 | 진행바 + 시스템 로그 | **`--style` 6종** (AI CLI 5종 + classic, 기본 `claude`) |
| 의존성 | 224개 패키지 | **코어 0개** (TUI는 선택 기능) |
| 바이너리 | 여러 MB (224개 크레이트) | **377 KB** (TUI 제외) · **558 KB** (기본) |
| 최대 RSS | 이 저장소에서 측정하지 않음 | **약 2 MB** |
| 설정 화면 | ratatui 전체 화면 TUI | **ratatui TUI** (`-i`, TTY) 또는 stdin 질문 |
| 설정 파일 | serde + toml | 40줄 직접 파서 |
| 배포 | 수동 | **GitHub Actions → crates.io + 5개 플랫폼 바이너리** |
| 지원 OS | Linux | **Linux · macOS · Windows** |
| Ctrl-C | 핸들러 스레드 필요 | 그냥 종료 (일반 출력 모드는 터미널 상태를 건드리지 않음) |

### 왜 이렇게 가벼운가

- **코어 런타임 의존성 0개.** `clap`, `serde`, `toml`, `indicatif`, `rand`, `colored`,
  `console`, `ctrlc` 없이 표준 라이브러리만 씁니다. TUI만 `ratatui` + `crossterm`을
  선택 기능(`tui`, 기본 켜짐)으로 씁니다.
- **`--no-default-features`로 TUI를 통째로 뺄 수 있습니다.** 그 빌드는 377 KB이고
  런타임 의존성이 정말 0개입니다.
- **스레드 없음, async 런타임 없음.** 메인 스레드 하나가 `write` + `sleep`만 합니다.
- **일반 출력 모드는 대체 화면(alternate screen)도 raw mode도 쓰지 않습니다.**
  전체 화면을 다시 그리지 않고 한 줄씩 흘려보냅니다. TUI는 `-i`로 설정할 때만
  잠깐 켜졌다가 원래 화면으로 돌아옵니다.
- **8 KB 출력 버퍼 하나 + 줄 버퍼 재사용.** 한 줄을 `String` 버퍼에 조립해서
  `write_all` 한 번으로 내보내고, ANSI 코드는 스타일이 실제로 바뀔 때만 넣습니다.
- **파이프가 닫히면 즉시 종료.** `imposter | head`가 CPU를 태우며 영원히 돌지 않습니다.
- **난수는 xorshift64\* 16바이트.** `/dev/urandom`도, `rand` 크레이트도 필요 없습니다.
- **릴리스 프로필**: `opt-level = "z"`, `lto`, `codegen-units = 1`, `panic = "abort"`, `strip`.
- TTY가 아니면 자동으로 색/애니메이션을 끕니다. `imposter | head`는 그냥 텍스트입니다.

측정 방법은 [로컬 개발](#로컬-개발) 섹션에 있습니다.

---

## 설치

crates.io:

```bash
cargo install imposter
```

Git에서 바로:

```bash
cargo install --git https://github.com/partrita/imposter-cli
```

Nix:

```bash
nix run github:partrita/imposter-cli
```

Docker (scratch 이미지, 바이너리만 들어 있음):

```bash
docker build -t imposter .
docker run -t --rm imposter --duration 1   # 1분
```

사전 빌드된 바이너리: [Releases](https://github.com/partrita/imposter-cli/releases)에서
OS/아키텍처에 맞는 파일을 받으면 Rust 없이 실행할 수 있습니다.

### 지원 플랫폼

| OS | 아키텍처 | 배포 파일 |
| --- | --- | --- |
| Linux | x86_64 | `imposter-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | arm64 | `imposter-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| macOS | Apple Silicon | `imposter-<version>-aarch64-apple-darwin.tar.gz` |
| macOS | Intel | `imposter-<version>-x86_64-apple-darwin.tar.gz` |
| Windows | x86_64 | `imposter-<version>-x86_64-pc-windows-msvc.zip` |

- Windows 10/11: 콘솔 ANSI 처리를 자동으로 켭니다. Windows Terminal 권장.
- macOS/Linux: 추가 설정 없이 동작합니다.
- 모든 릴리스에 `SHA256SUMS.txt`가 함께 올라갑니다.
- MSRV는 1.88입니다 (TUI 의존성 기준). 코어만 빌드해도 매니페스트 선언은 동일합니다.

소스에서 빌드:

```bash
# Linux / macOS
cargo install imposter

# Windows (PowerShell)
cargo install imposter

# TUI 없이 가장 작게
cargo install imposter --no-default-features
```

---

## 사용법

```bash
imposter                                     # 첫 실행은 TUI 설정, 이후 Claude Code 스타일로 계속
imposter --duration 5                        # 5분만
imposter --classic                           # 예전 시스템 로그 스타일
imposter --dev-type backend                  # 백엔드 개발자 흉내
imposter --alerts                            # 가짜 lint 에러 → 가짜 수정
imposter --fast --seed 42                    # 애니메이션 없이, 재현 가능하게
imposter -i                                  # TUI로 설정 (TTY가 아니면 질문 모드)
imposter --style codex                       # Codex 스타일
imposter --style opencode --duration 2       # opencode 스타일, 2분
```

### 스타일

`-s, --style` 하나로 화면 전체를 고릅니다. 각 AI CLI 스타일은 실제 에이전트 TUI의
헤더·프롬프트·툴 호출·상태바 레이아웃을 그대로 재현하며, `classic`은 별도의 시스템 로그 렌더러를 씁니다.

| 값 | 화면 | 특징 |
| --- | --- | --- |
| `claude` (기본) | Claude Code v2.1 | 2단 Clawd 픽셀 마스코트 환영 박스, `───` + `❯` 프롬프트, `⏺` 불릿 + `⎿` 트리, `⏵⏵ accept edits on` |
| `codex` | OpenAI Codex (`codex-rs`) | `>_` 헤더, `›` 프롬프트, `• Explored` / `• Edited` / `• Ran` + `└` 트리, `─ Worked for Xs ───` 구분선 |
| `opencode` | OpenCode TUI (`anomalyco/opencode`) | `SplitBorder`(`┃`) + `backgroundPanel` 세션 헤더(`# title` + 토큰/비용)·프롬프트·블록 툴(`$ cmd`, `# Todo`), `┃  Thinking:`, `→ Read` / `✱ Glob` / `✱ Grep` / `← Edit`, `▣ Build · qwen3.5-27b`, 하단 프롬프트 바 + `• LSP  • MCP  1.2.14` |
| `pi` | Pi (`earendil-works/pi`) | `▀█▀ / █▀█` 로고·핫키·`[context]` 헤더, `#343541` `userMessageBg` 박스, `Thinking... (ctrl+t)`, `#283228` `toolSuccessBg` 배경 박스(`read`/`edit`/`$`), `footer.ts` 2줄 상태바 |
| `antigravity` | Google Antigravity CLI (`agy`) | `···· / ────────` 호라이즌 배너 + `Welcome to the Antigravity CLI.`, `○ Read(...)` / `○ Edit(...)` / `○ Bash(...)` / `○ Subagents(...)` + `⎿` 트래젝토리(`ctrl+o`), 10블록 `/statusline` HUD |
| `classic` | 예전 시스템 로그 | 부팅 진행바, 코드 분석/메트릭/네트워크 로그 |

각 AI CLI 스타일은 실제 TUI를 기준으로 맞췄으며, 모든 스타일에서 **다크 모드(`--dark`)와 라이트 모드(`--light`)** 팔레트를 지원합니다. 좁은 터미널에서는 상자와 푸터가 자동으로 줄어듭니다.
`-i`로 열리는 **ratatui 설정 TUI**에서는 상단 스타일 탭(`1`~`6`), 테마 전환(`t`), `jk`/`hl` 및 방향키 조작, 레벨 게이지(`■■□□`),
실시간 스타일·테마·스택·복잡도·알림이 즉시 반영되는 라이브 프리뷰, 필드별 상세 가이드를 제공합니다.

출력은 `Deck`(섞은 주머니)으로 뽑기 때문에 한 바퀴가 끝나기 전에는 같은 대사가
다시 나오지 않습니다. 파일 목록·프롬프트·툴 출력·디프도 각각 독립적인 주머니라
세션이 길어져도 같은 장면이 연속으로 반복되지 않습니다.

### 옵션

| 옵션 | 설명 |
| --- | --- |
| `-s, --style <NAME>` | `claude` `codex` `opencode` `pi` `antigravity` `classic` (기본 `claude`) |
| `--claude` / `--classic` | `--style claude` / `--style classic` 단축키 |
| `-t, --theme <MODE>` | `dark` `light` (기본 `dark`) |
| `--dark` / `--light` | `--theme dark` / `--theme light` 단축키 |
| `-d, --dev-type <TYPE>` | `backend` `frontend` `fullstack` `data-science` `devops` `blockchain` `machine-learning` `systems` `game-dev` `security` |
| `-T, --duration <MINUTES>` | N분 뒤 종료, 0~999 (0 = 무한, 기본값 0) |
| `-a, --alerts` | 가짜 lint 에러 → 가짜 수정 |
| `-j, --jargon <LEVEL>` | `low` `medium` `high` `extreme` |
| `-c, --complexity <LEVEL>` | `low` `medium` `high` `extreme` |
| `-i, --interactive` | TUI 설정 (TTY가 아니면 stdin 질문) 후 실행. 설정 파일이 없으면 인자 없이 실행해도 자동으로 열림 |
| `--save` | 현재 설정을 `~/.config/imposter/config.toml`에 저장 |
| `--fast` | 애니메이션 끔 (GIF 녹화, CI용) |
| `--seed <N>` | 출력 고정 |

---

## 로컬 개발

### 1. 클론하고 바로 실행

```bash
git clone https://github.com/partrita/imposter-cli.git
cd imposter-cli

cargo run                          # 일반 실행
cargo run -- --fast --duration 1   # 1분, 애니메이션 없이
cargo run -- --style classic --seed 7   # classic 스타일, 재현 가능
```

`cargo run`은 첫 빌드에 10~20초, 이후에는 1~2초입니다. 코어는 의존성이 0개이고,
TUI를 빼면 더 빨리 빌드됩니다:

```bash
timeout 5 cargo run --no-default-features -- --fast   # 5초만 실행
```

### 2. 코드 품질 확인

```bash
cargo fmt
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

현재 테스트는 5개(RNG 결정성·범위, 줄바꿈, 토큰 표기, CLI 기본값)로 1초 안에 끝납니다.

### 3. 릴리스 빌드 + 리소스 측정

```bash
cargo build --release
ls -lh target/release/imposter           # 약 558 KB (TUI 포함)

cargo build --release --no-default-features
ls -lh target/release/imposter           # 약 377 KB (TUI 제외, 의존성 0개)

# macOS
timeout 2 /usr/bin/time -l ./target/release/imposter --fast >/dev/null
# Linux
timeout 2 /usr/bin/time -v ./target/release/imposter --fast >/dev/null
```

`maximum resident set size` 값을 보면 됩니다. 로컬 측정값은 1.9~2.2 MB입니다.

### 4. 바이너리가 작은지 확인

```bash
cargo tree --no-default-features   # 비어 있어야 정상: imposter v0.1.0
cargo tree                        # 기본 빌드는 ratatui/crossterm만 추가
cargo bloat --release      # (선택) cargo install cargo-bloat
```

`Cargo.toml`의 `[profile.release]`를 건드리면 크기가 바로 커집니다.
`opt-level = "z"`, `lto`, `panic = "abort"`, `strip`을 유지하세요.

### 5. 코드 구조

| 파일 | 역할 | 대략적인 크기 |
| --- | --- | --- |
| `src/main.rs` | 진입점, 스타일 분기, deadline | 70줄 |
| `src/cli.rs` | 직접 만든 인자 파서 + `--help` | 170줄 |
| `src/ansi.rs` | ANSI 색, 폭 계산, 스피너, wrap | 150줄 |
| `src/transcript.rs` | 트랜스크립트 엔진 (페이싱·턴 구성·덱) | 200줄 |
| `src/style/mod.rs` | `Style`/`Skin` 정의, `Renderer` 트레이트, 공용 헬퍼 | 530줄 |
| `src/style/claude.rs` | Claude Code v2.1 (기준 레이아웃) | 35줄 |
| `src/style/codex.rs` | OpenAI Codex (`codex-rs`) 레이아웃 오버라이드 | 235줄 |
| `src/style/opencode.rs` | OpenCode TUI 레이아웃 오버라이드 | 265줄 |
| `src/style/pi.rs` | Pi (`pi-coding-agent`) 레이아웃 오버라이드 | 280줄 |
| `src/style/antigravity.rs` | Google Antigravity CLI (`agy`) 레이아웃 오버라이드 | 380줄 |
| `src/tui.rs` | ratatui 설정 화면 + 실시간 스타일 프리뷰 (선택 기능) | 620줄 |
| `src/classic.rs` | 예전 시스템 로그 스타일 | 260줄 |
| `src/jargon.rs` | jargon 레벨별 대사 | 60줄 |
| `src/content.rs` | 대사/프롬프트/디프 데이터 | 480줄 |
| `src/config.rs` | `key = value` 설정 파일 | 130줄 |
| `src/setup.rs` | `-i` 질문 흐름 | 85줄 |
| `src/rng.rs` | xorshift64\* PRNG | 75줄 |
| `src/types.rs` | 개발유형/난이도 enum | 120줄 |

새 화면을 넣고 싶으면 `src/transcript.rs`의 `run()` 루프에 블록을 추가하면 됩니다.
새 스타일은 세 단계면 됩니다.

1. `src/style/mod.rs`의 `Style`에 variant 추가 (`ALL`, `as_str`, `parse`, `renderer`)
2. `src/style/<이름>.rs`에 `Skin` 상수와 `impl Renderer` 작성
3. `src/style/mod.rs`에 `pub mod <이름>;` 추가

`Renderer`의 모든 메서드에는 기본 구현이 있으므로 `skin()`만 정의해도 동작합니다.
그 다음 `welcome`, `prompt`, `tool`, `footer`처럼 실제로 다른 부분만 하나씩
오버라이드하면 됩니다. 기준 레이아웃인 Claude는 `skin()`만 정의하고, Codex·opencode·Pi·Antigravity는
각자의 실제 CLI 레이아웃에 맞춰 훅을 오버라이드합니다.
새 대사는 `src/content.rs`에 상수로 추가하면 `Deck`이 자동으로 섞어서 소진합니다.
컴파일 타임에 정적으로 들어가서 런타임 비용이 없습니다.

### 6. 동작 확인 팁

```bash
# 색이 없는 파이프 출력 확인
./target/release/imposter --fast | cat -v | head

# 애니메이션 확인 (TTY 필요)
./target/release/imposter --duration 1

# 특정 시드로 항상 같은 출력
./target/release/imposter --classic --fast --seed 1 | md5

# 스타일별 출력 비교
for s in claude codex opencode pi antigravity; do
  ./target/release/imposter --fast --seed 1 -s "$s" | head -5
done
```

---

## crates.io 자동 배포 (GitHub Actions)

워크플로: `.github/workflows/publish.yml`

### 최초 1회 설정

1. crates.io → **Account Settings → API Tokens → New Token**에서 토큰 발급
   (scope: `publish-new`, `publish-update`)
2. GitHub 저장소 → **Settings → Secrets and variables → Actions → New repository secret**
   - 이름: `CARGO_REGISTRY_TOKEN`
   - 값: 위에서 발급한 토큰
3. (선택) **Settings → Environments**에 `crates.io` 환경을 만들고
   required reviewers나 tag 제한을 걸면 토큰 오용을 막을 수 있습니다.

### 배포 절차

```bash
# 1. 버전 올리기
#    Cargo.toml 의 version = "0.1.1"

# 2. 커밋
git commit -am "release v0.1.1"

# 3. 태그 푸시 (이 순간 워크플로가 실행됨)
git tag v0.1.1
git push origin main --tags
```

Actions가 자동으로:

1. 태그(`v0.1.1`)와 `Cargo.toml` 버전이 같은지 확인
2. `cargo test --locked`
3. `cargo package --locked`
4. `cargo publish --locked`

이미 crates.io에 같은 버전이 있으면 실패하지 않고 건너뜁니다.

### 수동 실행 (dry-run)

Actions 탭 → **publish** → **Run workflow** → `dry_run` 체크된 상태로 실행하면
실제 배포 없이 `cargo publish --dry-run`까지만 확인합니다.
체크를 해제하면 수동으로 실제 배포합니다.

### 멀티 플랫폼 바이너리

워크플로: `.github/workflows/release.yml`

같은 `v*` 태그에서 아래 5개 타깃을 빌드해 GitHub Release에 올립니다.

| 러너 | 타깃 |
| --- | --- |
| `ubuntu-latest` | `x86_64-unknown-linux-gnu` |
| `ubuntu-24.04-arm` | `aarch64-unknown-linux-gnu` |
| `macos-latest` | `aarch64-apple-darwin` |
| `macos-latest` | `x86_64-apple-darwin` |
| `windows-latest` | `x86_64-pc-windows-msvc` |

- 압축 파일과 `SHA256SUMS.txt`가 릴리스에 첨부됩니다.
- 태그와 `Cargo.toml` 버전이 다르면 빌드 전에 실패합니다.
- `publish.yml`(crates.io)과 `release.yml`(바이너리)은 같은 태그에서 각자 실행됩니다.

### 테스트 CI

워크플로: `.github/workflows/ci.yml`

`main` push와 PR마다 Ubuntu · macOS · Windows에서 `fmt`(Linux만) · `clippy`
· `test` · `--no-default-features` 빌드 · 1초 스모크 실행을 돌립니다.

---

## 크레딧

이 프로젝트의 **원작자는 [@giacomo-b](https://github.com/giacomo-b)** 입니다.
원본은 [giacomo-b/rust-stakeholder](https://github.com/giacomo-b/rust-stakeholder) (MIT)이고,
"터미널을 바쁘게 보이게 해서 일하는 척한다"는 아이디어, 농담, 기능 구성은 전부 그쪽에서 나왔습니다.

**원본 코드를 MIT 라이선스로 공개해 주셔서 감사합니다.** 덕분에 이 프로젝트를 만들고,
이름을 바꾸고, Claude Code 스타일 화면으로 다시 쓸 수 있었습니다.

이 저장소(`imposter`)에서 바뀐 부분은 이름, 화면 렌더러, 의존성 제거(224개 → 0개),
crates.io 배포 워크플로뿐입니다. 재미있는 아이디어의 출처는 원작에 있습니다.

## 라이선스

MIT. `LICENSE` 참고. 원본 저작권(`Copyright (c) 2025 giacomo-b`)도 `LICENSE`에 함께 남겨 두었습니다.

## 면책

이 도구는 풍자입니다. 화면에 뜨는 파일, 테스트 결과, PR 승인은 전부 가짜입니다.
성과 보고에 쓰지 마세요. 실제로 코드를 짜는 척하다가 실제로 코드를 짜게 되는 일은 없습니다.

---

<sub>English: `imposter` is a zero-dependency, ~345 KB Rust CLI that prints a fake Claude Code session so your terminal looks busy. It never touches the filesystem, spawns no threads, and uses ~2 MB of RAM. Satire only. `cargo install imposter`.</sub>
