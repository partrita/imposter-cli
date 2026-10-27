# 스타일 가이드

`imposter`는 다양한 최신 AI 코딩 에이전트 CLI의 고유한 TUI 디자인을 그대로 재현합니다.

## 지원 스타일 목록

| 스타일 이름 | 원본 에이전트 CLI | 주요 특징 |
|---|---|---|
| `claude` (기본) | Anthropic Claude Code v2.1 | Clawd 픽셀 마스코트 환영 박스, `❯` 프롬프트, `⏺` 불릿과 `⎿` 트리 |
| `codex` | OpenAI Codex (`codex-rs`) | `>_` 헤더, `›` 프롬프트, `• Explored` / `• Edited` / `• Ran` |
| `opencode` | OpenCode TUI (`anomalyco/opencode`) | `SplitBorder`(`┃`) + `backgroundPanel`, `Thinking:`, `→ Read`, `← Edit` |
| `pi` | Pi Agent (`earendil-works/pi`) | `▀█▀ / █▀█` 헤더, `#343541` / `#283228` 전체 배경 박스, `footer.ts` |
| `antigravity` | Google Antigravity CLI (`agy`) | `····` 호라이즌 배너, `○ Read(...)` 트래젝토리, 10블록 `/statusline` HUD |
| `classic` | 시스템 엔지니어 로그 | 레거시 부팅 프로그레스 바, 심볼 링커 및 레이턴시 메트릭 |

각 스타일은 `--style <이름>` 또는 축약 플래그(예: `--claude`, `--classic`)로 선택할 수 있습니다.
모든 스타일은 다크 모드(`--dark`)와 라이트 모드(`--light`)를 완벽하게 지원합니다.
