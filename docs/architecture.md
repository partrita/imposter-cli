# 아키텍처 및 원리

`imposter`는 의존성을 최소화하고 메모리 및 바이너리 크기를 극한으로 최적화한 순수 Rust 아키텍처를 가집니다.

## 1. 제로 런타임 의존성 (Core)

- 일반 CLI 모드는 `clap`, `serde`, `tokio`, `rand`, `regex` 등 외부 크레이트를 전혀 사용하지 않습니다.
- 표준 라이브러리(`std::io`, `std::thread`, `std::time`, `std::env`)만으로 동작합니다.
- TUI 화면(`ratatui`, `crossterm`)은 선택적 기능(`features = ["tui"]`)으로 분리되어 있으며, `--no-default-features` 빌드 시 약 345 KB 크기의 초소형 바이너리가 됩니다.

## 2. 스트리밍 ANSI 렌더러 (`src/ansi.rs`)

- 화면 전체를 다시 그리는 Alternate screen이나 Raw mode를 일반 출력 모드에서는 사용하지 않습니다.
- 8 KB 고정 용량의 `BufWriter` 버퍼와 단일 라인 재사용 버퍼를 통해, 매 라인마다 단 한 번의 `write_all` syscall로 출력합니다.
- 색상 SGR 코드는 스타일이나 배경이 실제로 변경될 때만 최소한으로 방출합니다.

## 3. 난수 및 반복 방지 덱 (`src/rng.rs`)

- 16바이트 크기의 초고속 `xorshift64*` PRNG를 직접 내장했습니다.
- 파일명, 프롬프트, 툴 출력, 디프 패치 데이터는 Fisher-Yates 알고리즘으로 셔플된 `Deck` 구조체로 관리되어, 한 주기가 끝나기 전에는 동일한 대사가 절대로 반복되지 않습니다.
