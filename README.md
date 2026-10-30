# imposter

> [!NOTE]
> 아무것도 하지 않으면서, 최신 AI 코딩 에이전트로 바이브 코딩하는 것처럼 보이는 CLI.

![demo](demo.gif)

터미널 화면에 실제 AI 코딩 에이전트(Claude Code, OpenAI Codex, OpenCode, Pi, Google Antigravity 등)의 작업 흐름을 사실적으로 그려줍니다. 파일은 단 하나도 변경되지 않으며, 모델 호출이나 토큰 소비도 (당연히) 없습니다.

지나가는 동료나 상사가 화면을 슬쩍 보면 "저 사람 지금 고난도 분산 시스템 최적화와 리팩터링에 몰입하고 있구나"라는 인상을 줍니다.


## 기본 사용법

```bash
# 기본 실행
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

> [!TIP]
> 실행 중 언제든지 <kbd>Ctrl</kbd> + <kbd>C</kbd>를 누르면 커서와 터미널 색상이 복원되며 즉시 안전하게 종료됩니다.

## 특징

- 최소한의 의존성: `clap`, `serde`, `tokio`, `rand`, `regex` 등 무거운 크레이트를 배제하고 자체 파서 및 경량 PRNG를 사용합니다. TUI는 `features = ["tui"]`로 격리되어 있습니다.
- `--no-default-features` 지원: TUI 기능마저 제외하면 바이너리 용량이 370 KB 수준으로 줄어들며, 최소한의 런타임 의존성(`ctrlc`)만 유지됩니다.
- 스레드 및 비동기 런타임 배제: 메인 스레드 하나가 단일 루프 안에서 정밀한 타이밍으로 I/O와 휴식을 수행합니다.
- 스트리밍 I/O 렌더러: 화면 전체를 다시 그리는 Alternate Screen(대체 화면)이나 Raw Mode를 사용하지 않고, 8 KB 내부 고정 버퍼와 재사용 줄 버퍼를 통해 매 줄마다 단 한 번의 커널 호출(`write_all`)로 부드럽게 출력합니다.
- 16바이트 PRNG & 덱(Deck) 시스템: `/dev/urandom`이나 무거운 난수 라이브러리 없이 초고속 `xorshift64*` 알고리즘과 Fisher-Yates 셔플 덱으로 반복 없는 다채로운 대사를 구성합니다.

## 설치 가이드

### crates.io를 통한 설치 (권장)

```bash
cargo install imposter
```

### Git 저장소에서 직접 빌드 및 설치

```bash
cargo install --git https://github.com/partrita/imposter-cli
```

### Nix 환경

```bash
nix run github:partrita/imposter-cli
```

### Docker 컨테이너 (바이너리 전용 scratch 이미지)

```bash
docker build -t imposter .
docker run -t --rm imposter --duration 1   # 1분 후 자동 종료
```

### 사전 빌드된 바이너리 다운로드

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


## 로컬 개발방법

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

# 최대 메모리 점유율(RSS) 측정 (macOS 기준 약 1.9~2.2 MB)
/usr/bin/time -l ./target/release/imposter --fast >/dev/null
```

---

## 크레딧

이 프로젝트는 [`giacomo-b/rust-stakeholder`](https://github.com/giacomo-b/rust-stakeholder) (MIT License)를 기반으로 발전되었습니다.

"터미널 화면을 바쁘게 움직여 일하는 것처럼 보이게 한다"는 기발하고 유쾌한 아이디어와 기본 구성의 출처는 모두 원작자에게 있습니다. 멋진 프로젝트를 오픈소스로 공개해 주신 원작자께 깊은 감사를 드립니다.

## 라이선스

본 프로젝트는 MIT 라이선스에 따라 배포됩니다. 자세한 내용은 [`LICENSE`](LICENSE) 파일을 참고해 주세요.
