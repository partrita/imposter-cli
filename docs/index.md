# imposter 매뉴얼

**아무것도 하지 않으면서, AI 코딩 에이전트로 바이브 코딩하는 것처럼 보이는 Rust CLI.**

터미널에 실제 AI 코딩 세션(Claude Code, OpenAI Codex, OpenCode, Pi, Google Antigravity 등)의 작업 흐름을 그려줍니다. 파일은 단 하나도 변경되지 않으며, 토큰도 단 1개도 소비하지 않습니다.

> **SATIRE ALERT** — 이 프로젝트는 풍자(Satire)용 유머 도구입니다. 성과 보고나 실제 작업 증빙 용도로 사용하지 마세요.

---

## 주요 특징

1. **실제 AI CLI TUI 정밀 재현**
   - 6가지 스타일(`claude`, `codex`, `opencode`, `pi`, `antigravity`, `classic`) 지원.
   - 다크 모드(`--dark`) 및 라이트 모드(`--light`) 팔레트 지원.
2. **극단적인 경량 설계**
   - 코어 런타임 의존성 **0개** (`clap`, `serde`, `tokio`, `rand` 없음).
   - 약 345 KB ~ 550 KB 바이너리, RSS ~2 MB.
   - 메인 스레드 1개에서 버퍼 재사용 및 단일 `write_all` 스트리밍.
3. **대화형 TUI 설정 마법사 (`-i`)**
   - `ratatui` 기반 실시간 프리뷰, 상단 스타일 단축키(`1`~`6`), 테마 전환(`t`).
4. **반복 없는 생생한 출력**
   - `xorshift64*` PRNG와 셔플 `Deck` 구조로 한 사이클 내 동일 대사 미반복.
