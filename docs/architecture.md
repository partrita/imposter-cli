# 내부 아키텍처 및 설계 원리

`imposter`는 의존성을 최소화하고, CPU 및 메모리 사용량을 극한으로 낮추면서도 끊김 없는 부드러운 텍스트 애니메이션을 제공하기 위해 세심하게 설계되었습니다.

---

## 1. 극단적인 최소 의존성 설계 (Minimal Dependencies)

일반적인 Rust CLI 프로젝트들이 의례적으로 도입하는 대형 라이브러리들을 과감히 배제했습니다:

- `clap`, `structopt` 대신: 170줄의 순수 문자열 기반 인자 파서 ([src/cli.rs](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/cli.rs))
- `serde`, `toml` 대신: 40줄의 정밀 설정 파서 ([src/config.rs](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/config.rs))
- `rand` 크레이트 대신: 16바이트 메모리만 쓰는 고속 `xorshift64*` PRNG ([src/rng.rs](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/rng.rs))
- `tokio`, 비동기 런타임, 백그라운드 스레드 일절 없음: 단일 메인 스레드 기반 동기 I/O 루프

선택 기능인 TUI 마법사(`ratatui` + `crossterm`)는 `features = ["tui"]`로 격리되어 있으며, 이를 끈 `--no-default-features` 빌드는 바이너리 크기가 약 370 KB에 불과합니다.

---

## 2. 스트리밍 ANSI 렌더러 ([src/ansi.rs](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/ansi.rs))

일반적인 TUI 툴들이 터미널의 대체 화면(Alternate Screen)을 차지하고 전체 캔버스를 매 프레임 다시 그리는 것과 달리, `imposter`는 일반 스크롤 버퍼에 표준 텍스트를 흘려보내는 스트리밍 방식을 취합니다:

1. **단일 I/O 버퍼 재사용**: 8 KB 고정 크기의 `BufWriter`와 단일 라인 조립용 `String` 버퍼(512바이트 캐파)를 영구 재사용하여 힙 할당을 극도로 줄입니다.
2. **단일 시스템 콜 (`write_all`)**: 조합된 한 라인은 단 한 번의 커널 쓰기 시스템 콜로 출력됩니다.
3. **상태 기반 최소 ANSI 방출**: 색상이나 스타일(SGR) 코드는 이전 텍스트와 실제로 스타일이 달라질 때만 선택적으로 방출하여 대역폭 낭비를 막습니다.
4. **파이프 감지 및 즉각 종료**: 표준 출력이 파이프(`imposter | head`)로 연결되어 있고 리더 프로세스가 닫히면, 무한 루프에 빠져 CPU를 낭비하지 않고 즉각 종료됩니다. 비-TTY 환경에서는 ANSI 색상과 타이핑 딜레이를 자동으로 비활성화합니다.

---

## 3. 셔플 덱 (`Deck`) 콘텐츠 관리 시스템 ([src/rng.rs](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/rng.rs))

출력이 단조롭게 반복되는 것을 막기 위해 모든 대사와 파일 목록, 디프 패치는 `Deck` 구조체로 관리됩니다:

- 초기 기동 시 배열의 인덱스를 Fisher-Yates 알고리즘으로 셔플합니다.
- 카드를 한 장씩 뽑아 소진하듯 순서대로 반환하며, 모든 아이템이 소진되면 다시 전체를 셔플하여 새 사이클을 시작합니다.
- 따라서 사용자가 장시간 화면을 켜두더라도 동일한 프롬프트나 동일한 파일명이 연속으로 나타나지 않고 생동감 있게 변화합니다.

---

## 4. 모듈식 스타일 확장 구조 ([src/style/](file:///Users/fkt/Downloads/PROJECT/imposter-cli/src/style/))

모든 에이전트 스타일은 `Renderer` 트레이트를 기반으로 구현됩니다:

- 트레이트의 기본 메서드들이 표준 Claude 레이아웃을 기본으로 제공하므로, 새로운 스타일을 추가할 때는 `skin()` 정의와 차별화가 필요한 레이아웃 훅(예: OpenCode의 `SplitBorder` 패널, Pi의 배경 박스, Antigravity의 10블록 HUD)만 오버라이드하면 됩니다.
- 스타일 선택은 `--style` 단일 셀렉터로 통합되어 있어 유지보수와 확장이 용이합니다.
