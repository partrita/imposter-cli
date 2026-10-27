# OpenCode 스타일 (`opencode`)

`anomalyco/opencode` v1.2+의 현대적인 SolidJS + `@opentui/core` TUI 세션 뷰를 정밀 재현합니다.

## 주요 화면 구성

- **세션 헤더**: `SplitBorder`(`┃`)와 `backgroundPanel`로 감싼 세션 제목 `# distributed-cluster` 및 우측 토큰/비용 원격 메트릭(`39,413 20% ($0.29)`).
- **프롬프트**: 에이전트 액센트 색상의 좌측 `┃` 보더 패널에서 타이핑.
- **추론 블록**: `┃  Thinking: ...` 흐린 좌측 보더 추론 블록.
- **인라인 툴**: `   → Read ... [offset=..., limit=...]`, `   ✱ Glob ...`, `   ✱ Grep ...`.
- **블록 툴**:
  - `   ← Edit ... (+5 -2)` 및 `┃` 패널 디프
  - `┃  $ cargo test --lib` 셸 실행 블록
  - `┃  # Todo` 및 `[✓]` / `[•]` 할 일 목록
- **푸터**: `   ▣ Build · qwen3.5-27b · 3.8s`, 하단 프롬프트 패널, 단축키 힌트 바, `~/code/... (main)  • LSP  • MCP  1.2.14` 상태바.

```bash
imposter --style opencode
```
