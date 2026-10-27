# Claude Code 스타일 (`claude`)

Anthropic의 공식 `Claude Code v2.1` 인터페이스를 재현합니다.

## 주요 화면 구성

- **환영 박스**: 2단 Clawd 픽셀 마스코트 아스키 아트 및 세션 팁.
- **프롬프트**: 상하 구분선 `────` 및 `❯` 프롬프트 타이핑 애니메이션.
- **툴 실행**: `⏺ Read(...)` / `⏺ Update(...)` 불릿과 `⎿` 들여쓰기 트리 출력.
- **상태바**: `✻ Worked for 1m 12s · 3.4k tokens · $0.03` 및 `⏵⏵ accept edits on`.

```bash
imposter --style claude
```
