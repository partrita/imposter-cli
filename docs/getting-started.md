# 빠른 시작 가이드

이 문서에서는 `imposter`의 다양한 설치 방법과 일상적인 기본 사용 흐름을 안내합니다.

---

## 1. 설치 방법

### 방법 A: crates.io를 통한 설치 (가장 권장)

Rust 개발 환경이 구축되어 있다면 다음 명령어로 간편하게 설치할 수 있습니다:

```bash
cargo install imposter
```

### 방법 B: Git 저장소에서 최신 코드 빌드

깃허브 저장소의 최신 변경 사항을 즉시 설치하려면 다음과 같이 실행합니다:

```bash
cargo install --git https://github.com/partrita/imposter-cli
```

### 방법 C: 초경량 최소 의존성 빌드 (TUI 제외)

터미널 마법사(`ratatui` + `crossterm`) 없이 순수 CLI 기능만 370 KB 안팎의 초소형 바이너리로 내려받으려면 `--no-default-features` 플래그를 사용합니다:

```bash
cargo install imposter --no-default-features
```

### 방법 D: 사전 빌드된 바이너리 다운로드

Rust 툴체인이 설치되어 있지 않은 환경이라면 [GitHub Releases](https://github.com/partrita/imposter-cli/releases) 페이지에서 본인의 운영체제 및 아키텍처에 맞는 압축 파일(`tar.gz` 또는 `zip`)을 내려받아 압축 해제 후 곧바로 실행할 수 있습니다.

- **Linux**: x86_64, aarch64
- **macOS**: Apple Silicon (M1/M2/M3/M4), Intel (x86_64)
- **Windows**: x86_64 MSVC (`imposter.exe`)

---

## 2. 첫 실행 및 기본 사용 흐름

### 첫 실행과 대화형 설정

설정 파일이 없는 상태에서 인자 없이 실행하면, 사용자 친화적인 대화형 설정 창이 자동으로 열립니다:

```bash
imposter
```

화면에서 원하는 CLI 스타일(예: `opencode`, `claude`, `antigravity`), 색상 테마(다크/라이트), 개발 분야(백엔드, 프론트엔드 등)를 둘러보고 `Enter` 또는 `Ctrl+R`을 누르면 즉시 가짜 코딩 세션이 시작됩니다.

### 대표적인 실행 예시

```bash
# 1. 5분 동안 실행 후 깔끔하게 종료 (자동 정지 타이머)
imposter --duration 5

# 2. OpenCode 스타일로 실행
imposter --style opencode

# 3. Google Antigravity CLI 스타일을 라이트 모드로 실행
imposter --style antigravity --light

# 4. 백엔드 개발자 시나리오로 실행
imposter --dev-type backend

# 5. 주기적으로 가짜 컴파일러 경고/린트 에러를 발생시키고 스스로 수정하게 하기
imposter --alerts

# 6. 타자 지연과 프레임 대기를 모두 끄고 초고속으로 출력 (GIF 녹화 또는 CI 검증용)
imposter --fast --seed 42
```

세션을 중단하고 싶을 때는 언제든지 `Ctrl + C`를 누르면 즉시 터미널 기본 상태로 안전하게 복귀합니다. 일반 출력 모드는 터미널의 대체 화면(Alternate Screen)을 오염시키지 않으므로 흔적이 남지 않습니다.
