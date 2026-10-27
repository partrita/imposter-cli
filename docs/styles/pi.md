# Pi Agent 스타일 (`pi`)

`earendil-works/pi` (`@earendil-works/pi-coding-agent`) 인터페이스를 재현합니다.

## 주요 화면 구성

- **헤더**: `▀█▀ / █▀█` 블록 로고, 단일 열 단축키 표, `[context] AGENTS.md`.
- **전체 폭 배경 박스**:
  - 사용자 메시지 박스 (`#343541`)
  - 툴 성공/실행 박스 (`#283228`)
  - 대기/진행 박스 (`#282832`)
  - 에러 박스 (`#3c2828`)
- **추론**: `Thinking... (ctrl+t to expand)`
- **푸터 (`footer.ts`)**: 액센트 구분선, 작업 경로, `↑in ↓out Rcache Wcache $cost ctx% (auto)  model • high`.

```bash
imposter --style pi
```
