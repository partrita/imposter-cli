# 스타일 가이드

`imposter`는 단일 선택자 `-s, --style <이름>` 옵션을 통해 화면 전체의 시각적 페르소나를 전환합니다. 각 스타일은 단순한 텍스트 흉내가 아니라, 실제 대상 AI CLI의 배너 구조, 프롬프트 입력창, 툴 실행 인라인/블록 박스, 상태 표시줄을 충실히 재현합니다.

---

## 1. 지원하는 스타일 요약

| 스타일 식별자 | 대상 CLI 에이전트 | 주요 시각적 특징 | 단축 플래그 |
|---|---|---|---|
| [`claude`](styles/claude.md) *(기본값)* | Anthropic Claude Code v2.1 | 2단 Clawd 픽셀 마스코트 환영 박스, `❯` 프롬프트 타이핑, `⏺` 불릿과 `⎿` 들여쓰기 트리, `⏵⏵ accept edits on` | `--claude` |
| [`codex`](styles/codex.md) | OpenAI Codex (`codex-rs`) | 상단 `>_` 헤더, `›` 프롬프트, `• Explored` / `• Edited` / `• Ran` 블록, `─ Worked for Xs ──` 구분선 | - |
| [`opencode`](styles/opencode.md) | OpenCode TUI (`anomalyco/opencode`) | SolidJS 기반 TUI의 `SplitBorder`(`┃`) + 전체 폭 `backgroundPanel`, `Thinking:`, `→ Read` 인라인 툴, `$ cmd` / `# Todo` 블록 툴, 4개 구역 상태바 | - |
| [`pi`](styles/pi.md) | Pi Agent (`earendil-works/pi`) | `▀█▀ / █▀█` 블록 헤더, 단일 열 핫키 안내, `#343541` 사용자 메시지 박스 및 `#283228` 성공 툴 박스, `footer.ts` 토큰/비용 2줄 바 | - |
| [`antigravity`](styles/antigravity.md) | Google Antigravity CLI (`agy`) | `····` 점선 호라이즌 배너, `○ Read(...)` / `○ Edit(...)` 트래젝토리 로그, 10개 구역 `/statusline` 실시간 HUD | - |
| [`classic`](styles/classic.md) | 원작 시스템 엔지니어링 로그 | 원작 `rust-stakeholder`의 프로그레스 바 부팅 화면, 심볼 링커, p99 레이턴시 측정 및 네트워크 패킷 분석 로그 | `--classic` |

---

## 2. 색상 테마 지원: 다크 모드와 라이트 모드

각 스타일은 터미널 배경색에 맞춰 최적의 가독성을 제공하도록 두 가지 테마 팔레트를 갖추고 있습니다:

- **다크 모드 (`-t dark` 또는 `--dark`)** *(기본값)*: 어두운 터미널 배경에서 눈이 피로하지 않도록 채도와 명도를 조율한 24비트 트루컬러 팔레트를 적용합니다.
- **라이트 모드 (`-t light` 또는 `--light`)**: 흰색이나 밝은 회색 계열 배경에서도 텍스트와 보더가 흐릿해지지 않도록 고대비 전경색과 파스텔톤 배경색을 적용합니다.

스타일과 테마는 자유롭게 조합할 수 있습니다:

```bash
# OpenCode 스타일을 라이트 모드로 실행
imposter -s opencode --light

# Pi 스타일을 다크 모드로 실행
imposter -s pi --dark
```

각 스타일별 세부 디자인과 렌더링 방식은 하위 문서에서 상세히 다룹니다.
