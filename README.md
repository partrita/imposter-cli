# imposter

**아무것도 하지 않으면서, 최신 AI 코딩 에이전트로 바이브 코딩하는 것처럼 보이는 Rust CLI.**

터미널 화면에 실제 AI 코딩 에이전트(Claude Code, OpenAI Codex, OpenCode, Pi, Google Antigravity 등)의 작업 흐름을 사실적으로 그려줍니다. 파일은 단 하나도 변경되지 않으며, 모델 호출이나 토큰 소비도 0개입니다.

지나가는 동료나 상사가 화면을 슬쩍 보면 "저 사람 지금 고난도 분산 시스템 최적화와 리팩터링에 몰입하고 있구나"라는 인상을 줍니다.

> [!WARNING]
> **Satire / 풍자 안내** — 본 프로젝트는 유머와 풍자를 목적으로 개발되었습니다. 실제 업무 성과 증빙이나 보고 용도로 악용해서는 안 됩니다.

```text
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

## 주요 개선 및 차이점

| 항목 | 원작 (`rust-stakeholder`) | 개편 후 (`imposter`) |
| --- | --- | --- |
| **이름** | rust-stakeholder | **imposter** |
| **화면 페르소나** | 고정된 시스템 엔지니어링 로그 | **6가지 스타일** (Claude, Codex, OpenCode, Pi, Antigravity, Classic) |
| **외부 의존성** | 224개 크레이트 | **코어 0개** (TUI는 선택 기능으로 분리) |
| **바이너리 용량** | 수 MB 이상 | **약 377 KB** (TUI 제외) · **약 558 KB** (기본 빌드) |
| **메모리 점유율 (RSS)** | 미측정 | **약 2 MB 내외** |
| **설정 인터페이스** | 단순 설정 창 | **실시간 라이브 프리뷰를 지원하는 대화형 TUI** (`-i`) |
| **설정 파일 파서** | serde + toml 라이브러리 | **40줄 내외의 자체 경량 TOML 파서** |
| **자동 배포 파이프라인** | 수동 릴리스 | **GitHub Actions → crates.io 및 5대 OS 바이너리 자동 배포** |
| **지원 운영체제** | Linux 중심 | **Linux · macOS · Windows 완벽 지원** |
| **종료 동작** | 별도 핸들러 스레드 필요 | **즉각 종료** (일반 스크롤 모드는 터미널 상태를 오염시키지 않음) |

### 극단적인 경량화 비결

- **런타임 의존성 0개 (Core)**: `clap`, `serde`, `tokio`, `rand`, `regex` 등 무거운 크레이트를 배제하고 순수 Rust 표준 라이브러리(`std`)만 사용합니다. TUI만 `features = ["tui"]`로 격리되어 있습니다.
- **`--no-default-features` 지원**: TUI 기능마저 제외하면 바이너리 용량이 377 KB 수준으로 줄어들며, 외부 의존성이 완전히 0개가 됩니다.
- **스레드 및 비동기 런타임 배제**: 메인 스레드 하나가 단일 루프 안에서 정밀한 타이밍으로 I/O와 휴식을 수행합니다.
- **스트리밍 I/O 렌더러**: 화면 전체를 다시 그리는 Alternate Screen(대체 화면)이나 Raw Mode를 사용하지 않고, 8 KB 내부 고정 버퍼와 재사용 줄 버퍼를 통해 매 줄마다 단 한 번의 커널 호출(`write_all`)로 부드럽게 출력합니다.
- **16바이트 PRNG & 덱(Deck) 시스템**: `/dev/urandom`이나 무거운 난수 라이브러리 없이 초고속 `xorshift64*` 알고리즘과 Fisher-Yates 셔플 덱으로 반복 없는 다채로운 대사를 구성합니다.

---

## 설치 가이드

### 1. crates.io를 통한 설치 (권장)

```bash
cargo install imposter
```

### 2. Git 저장소에서 직접 빌드 및 설치

```bash
cargo install --git https://github.com/partrita/imposter-cli
```

### 3. 초소형 제로 의존성 빌드 (TUI 제외)

```bash
cargo install imposter --no-default-features
```

### 4. Nix 환경

```bash
nix run github:partrita/imposter-cli
```

### 5. Docker 컨테이너 (바이너리 전용 scratch 이미지)

```bash
docker build -t imposter .
docker run -t --rm imposter --duration 1   # 1분 후 자동 종료
```

### 6. 사전 빌드된 바이너리 다운로드

Rust 개발 도구가 없는 환경에서는 [GitHub Releases](https://github.com/partrita/imposter-cli/releases)에서 운영체제별 압축 파일을 내려받아 즉시 실행할 수 있습니다:

| 운영체제 | 아키텍처 | 배포 파일명 |
| --- | --- | --- |
| Linux | x86_64 | `imposter-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | arm64 | `imposter-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| macOS | Apple Silicon (M 시리즈) | `imposter-<version>-aarch64-apple-darwin.tar.gz` |
| macOS | Intel (x86_64) | `imposter-<version>-x86_64-apple-darwin.tar.gz` |
| Windows | x86_64 | `imposter-<version>-x86_64-pc-windows-msvc.zip` |

- Windows 10/11: 콘솔의 ANSI 트루컬러 처리를 자동으로 활성화합니다 (Windows Terminal 권장).
- macOS / Linux: 별도 설정 없이 바로 동작합니다.
- 모든 릴리스 아티팩트에는 무결성 검증용 `SHA256SUMS.txt`가 함께 제공됩니다.

---

## 기본 사용법

```bash
# 기본 실행 (설정 파일이 없으면 첫 1회 TUI 마법사 자동 실행, 이후 Claude 스타일)
imposter

# 5분 동안 실행 후 깔끔하게 종료 (자동 정지 타이머)
imposter --duration 5

# 대화형 설정 마법사 직접 실행
imposter -i

# 스타일 선택 실행
imposter --style opencode                     # OpenCode 스타일
imposter --style antigravity                  # Google Antigravity CLI 스타일
imposter --style pi                           # Pi Agent 스타일
imposter --style codex                        # OpenAI Codex 스타일
imposter --classic                            # 원작 레거시 시스템 로그 스타일

# 라이트 모드(밝은 터미널 배경용 고대비 테마)로 실행
imposter --style opencode --light

# 가짜 린트/컴파일 에러 자동 복구 시나리오 활성화
imposter --alerts

# 백엔드 또는 프론트엔드 개발자 페르소나 설정
imposter --dev-type backend
imposter --dev-type frontend

# 타이핑 딜레이를 끄고 초고속으로 출력 (GIF 녹화 또는 CI 검증용)
imposter --fast --seed 42
```

---

## 지원 스타일 상세

`-s, --style <이름>` 옵션을 통해 원하는 AI 에이전트의 화면을 선택할 수 있습니다. 각 스타일은 실제 CLI 도구의 폰트 기호, 박스 보더, 툴 트리, 상태바 배치를 정밀하게 모사했습니다:

| 스타일 식별자 | 대상 CLI 에이전트 | 주요 시각적 특징 |
|---|---|---|
| `claude` *(기본)* | Anthropic Claude Code v2.1 | 2단 Clawd 마스코트 환영 박스, `───` 구분선과 `❯` 프롬프트, `⏺` 불릿과 `⎿` 들여쓰기 트리, `⏵⏵ accept edits on` |
| `codex` | OpenAI Codex (`codex-rs`) | `>_` 헤더, `›` 프롬프트, `• Explored` / `• Edited` / `• Ran` 3단계 블록과 `└` 트리, `─ Worked for Xs ──` 분할선 |
| `opencode` | OpenCode TUI (`anomalyco/opencode`) | SolidJS 기반 TUI의 `SplitBorder`(`┃`) + 전체 폭 `backgroundPanel`, `Thinking:`, `→ Read` 인라인 툴, `$ cmd` / `# Todo` 블록 툴, 4개 구역 상태바 |
| `pi` | Pi Agent (`earendil-works/pi`) | `▀█▀ / █▀█` 블록 헤더, 단일 열 핫키 안내, `#343541` 사용자 메시지 박스 및 `#283228` 성공 툴 박스, `footer.ts` 토큰/비용 2줄 바 |
| `antigravity` | Google Antigravity CLI (`agy`) | `····` 점선 호라이즌 배너, `○ Read(...)` / `○ Edit(...)` 트래젝토리 로그, 10개 구역 `/statusline` 실시간 HUD |
| `classic` | 시스템 엔지니어 로그 | 원작 `rust-stakeholder`의 프로그레스 바 부팅 화면, 심볼 링커, p99 레이턴시 및 기술 용어 로그 |

모든 스타일은 다크 모드(`--dark`)와 라이트 모드(`--light`)를 완벽하게 지원하며, 터미널 가로 폭이 좁아지면 레이아웃이 유연하게 축소됩니다.

---

## 커맨드라인 옵션 레퍼런스

| 옵션 | 설명 | 기본값 |
|---|---|---|
| `-s, --style <NAME>` | 시각적 스타일 선택 (`claude`, `codex`, `opencode`, `pi`, `antigravity`, `classic`) | `claude` |
| `--claude` / `--classic` | `--style claude` / `--style classic` 단축 플래그 | - |
| `-t, --theme <MODE>` | 색상 테마 선택 (`dark`, `light`) | `dark` |
| `--dark` / `--light` | `--theme dark` / `--theme light` 단축 플래그 | - |
| `-d, --dev-type <TYPE>` | 대상 코드베이스 기술 스택 (`backend`, `frontend`, `fullstack`, `data-science`, `devops`, `blockchain`, `machine-learning`, `systems`, `game-dev`, `security`) | `backend` |
| `-T, --duration <MINUTES>` | 자동 종료 타이머 (분 단위, `0`은 무한 지속) | `0` |
| `-a, --alerts` | 주기적 린트/컴파일 에러 발생 및 자동 수정 시나리오 활성화 | 끔 |
| `-j, --jargon <LEVEL>` | 기술 전문 용어 난이도 (`low`, `medium`, `high`, `extreme`) | `medium` |
| `-c, --complexity <LEVEL>` | 턴 전개 깊이 및 복잡도 (`low`, `medium`, `high`, `extreme`) | `medium` |
| `-i, --interactive` | 대화형 TUI 설정 마법사 실행 (비-TTY 환경에서는 텍스트 문답으로 대체) | - |
| `--save` | 현재 전달된 인자값들을 `~/.config/imposter/config.toml`에 기본값으로 저장 | - |
| `--fast` | 타이핑 딜레이와 대기 시간을 끄고 초고속 출력 (GIF/CI용) | - |
| `--seed <N>` | 출력 시나리오를 결정론적으로 고정하는 PRNG 시드값 | 무작위 |
| `-h, --help` | 도움말 출력 | - |
| `-v, --version` | 버전 정보 출력 | - |

---

## 로컬 개발 및 기여 가이드

### 1. 프로젝트 빌드 및 실행

```bash
git clone https://github.com/partrita/imposter-cli.git
cd imposter-cli

# 일반 실행
cargo run

# 1분 동안 애니메이션 없이 실행
cargo run -- --fast --duration 1

# OpenCode 스타일로 실행
cargo run -- --style opencode
```

### 2. 코드 품질 검사

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

### 3. 릴리스 빌드 및 리소스 점유율 측정

```bash
cargo build --release
ls -lh target/release/imposter           # 약 558 KB (TUI 포함)

cargo build --release --no-default-features
ls -lh target/release/imposter           # 약 377 KB (TUI 제외)

# 최대 메모리 점유율(RSS) 측정 (macOS 기준 약 1.9~2.2 MB)
/usr/bin/time -l ./target/release/imposter --fast >/dev/null
```

---

## 크레딧 및 원작 안내

이 프로젝트는 **[@giacomo-b](https://github.com/giacomo-b)** 님의 **[`giacomo-b/rust-stakeholder`](https://github.com/giacomo-b/rust-stakeholder)** (MIT License)를 기반으로 발전되었습니다.

"터미널 화면을 바쁘게 움직여 일하는 것처럼 보이게 한다"는 기발하고 유쾌한 아이디어와 기본 구성의 출처는 모두 원작자에게 있습니다. 멋진 프로젝트를 오픈소스로 공개해 주신 원작자께 깊은 감사를 드립니다.

`imposter` 포크에서는 현대적인 AI 에이전트 CLI 스타일 5종 추가, 224개 외부 의존성의 완전 제거(코어 0개), 300KB대 초경량화, 다크/라이트 테마 체계, 대화형 라이브 프리뷰 TUI 및 자동 배포 파이프라인 구축을 새롭게 수행했습니다.

---

## 라이선스

본 프로젝트는 **MIT 라이선스**에 따라 배포됩니다. 자세한 내용은 [`LICENSE`](LICENSE) 파일을 참고해 주세요.
