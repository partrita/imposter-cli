# OpenCode 스타일 (`opencode`)

차세대 오픈소스 코딩 에이전트인 `anomalyco/opencode` v1.2+의 SolidJS 기반 `@opentui/core` 세션 화면을 고스란히 재현한 스타일입니다.

---

## 1. 주요 레이아웃 특징

### `SplitBorder`와 `backgroundPanel` 컨테이너 구조
OpenCode 스타일의 가장 큰 시각적 정체성은 4면 박스 대신, 어두운 패널 배경색(`#1a1b26`, 라이트 모드 시 `#e6e9ef`) 위에 좌측 굵은 세로선(`┃`) 하나로 영역을 나누는 현대적인 분할 구조입니다:

```text
 ┃
 ┃  # distributed-cluster                                  39,413  20% ($0.29)
 ┃
```
- **세션 헤더**: 프로젝트 이름(`# distributed-cluster`)이 굵게 표시되며, 우측 끝에는 실시간 컨텍스트 토큰 소비량과 비용 메트릭이 배치됩니다.
- **사용자 질의창**: 에이전트 액센트 색상의 좌측 `┃` 보더 패널 안에서 사용자 입력 텍스트가 자연스럽게 타이핑됩니다.

### 추론 영역 (`Thinking: ...`)
턴이 시작되면 도구 호출에 앞서 OpenCode 특유의 흐린 좌측 보더와 함께 사고 전개 과정이 나타납니다:
```text
 ┃  Thinking: Let me see how the sibling module solved this.
```

### 인라인 도구 및 블록 도구 분리
- **인라인 도구 (InlineTool)**: 파일 읽기와 검색은 3칸 들여쓰기 후 방향 화살표와 심볼로 깔끔하게 한 줄로 출력됩니다.
  - `   → Read src/service.rs [offset=1, limit=140]`
  - `   ✱ Glob "src/**/*metrics*" (5 matches)`
  - `   ✱ Grep "timeout" in src (21 matches)`
- **블록 도구 (BlockTool)**: 코드 수정, 셸 명령 실행, 할 일 목록은 각각 독립된 `SplitBorder` 패널 컨테이너로 감싸져 출력됩니다.
  - `   ← Edit src/service.rs (+4 -2)` 헤더 아래 패널 디프
  - `┃  $ cargo test --all-features` 셸 실행 및 출력 블록
  - `┃  # Todo` 및 `[✓]`, `[•]` 체크리스트 블록

### 턴 완료 배지 및 다층 푸터
턴이 마무리되면 에이전트 배지가 출력된 뒤, 하단 프롬프트 입력 바와 상태줄이 차례로 표시됩니다:
```text
   ▣ Build · qwen3.5-27b · 3m 11s (2.4k tokens · 6% · $0.02)

 ┃
 ┃  Build  qwen3.5-27b  OpenRouter
 ⬝⬝⬝⬝⬝⬝⬝⬝ esc interrupt            ctrl+t variants  tab agents  ctrl+p commands
 ~/code/distributed-cluster (main)                         • LSP  • MCP  1.2.14
```

---

## 2. 실행 명령어

```bash
# OpenCode 스타일 실행
imposter --style opencode

# 라이트 모드로 실행
imposter --style opencode --light
```
