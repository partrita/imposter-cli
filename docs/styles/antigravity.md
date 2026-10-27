# Google Antigravity 스타일 (`antigravity`)

Google의 `Antigravity CLI` (`agy`) 인터페이스를 재현합니다.

## 주요 화면 구성

- **배너**: `···· / ······ / ────────` 호라이즌 점선 배너 및 `Welcome to the Antigravity CLI.`
- **프롬프트**: `❯` 프롬프트 타이핑 애니메이션
- **트래젝토리 도구**:
  - `○ Read(...)` 및 `⎿ Read 140 lines (ctrl+o to expand)`
  - `○ Edit(...)` 및 패치 디프
  - `○ Subagents(...)` 서브에이전트 병렬 작업 트리
- **하단 HUD**: `READY │ gemini-3.1-pro │ Ctx [██░░░░░░░░] 18% │ 3 agents` 등 10개 블록의 실시간 상태선.

```bash
imposter --style antigravity
```
