# OpenAI Codex 스타일 (`codex`)

OpenAI의 `codex-rs` CLI 인터페이스를 재현합니다.

## 주요 화면 구성

- **헤더**: `>_ OpenAI Codex (codex-rs) · gpt-5-codex`
- **프롬프트**: `›` 프롬프트 타이핑 애니메이션
- **툴 실행**:
  - `• Explored` (파일 읽기 및 심볼 검색)
  - `• Edited <file> (+add -del)` (코드 패치)
  - `• Ran <command>` (명령어 실행 및 결과 트리 `└`)
- **턴 구분선**: `─ Worked for 48s ─────────────────`

```bash
imposter --style codex
```
