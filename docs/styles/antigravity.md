# Google Antigravity 스타일 (`antigravity`)

Google의 차세대 AI 코딩 터미널인 `Antigravity CLI` (`agy`) 인터페이스를 사실감 있게 모사한 스타일입니다.

---

## 1. 주요 레이아웃 특징

### 상단 호라이즌 배너 (Horizon Dotted Banner)
Google Antigravity의 아이덴티티인 점선 호라이즌 배너와 함께 공식 환영 메시지가 푸른색 액센트로 나타납니다:
```text
   ····  ······  ────────
   Welcome to the Antigravity CLI. Gemini 3.1 Pro
```

### 트래젝토리(Trajectory) 궤적 기반 도구 렌더링
Antigravity는 에이전트의 행동을 시간의 흐름(Trajectory)으로 기록합니다:
- **도구 호출**: 원형 불릿(`○`)과 괄호 인수 형식으로 나타납니다: `○ Read(src/lib.rs:1-140)`, `○ Edit(src/api.rs)`.
- **실행 결과**: `⎿` 기호와 함께 접기/펼치기 단축키 힌트(`ctrl+o to expand`)가 붙은 트래젝토리 분기선으로 연결됩니다.
- **서브에이전트 병렬 작업 블록**: 복잡도(Complexity) 설정 시 `○ Subagents(3 active · /agents)` 블록이 생성되어 `[research]`, `[verify]` 등 다중 에이전트가 협업하는 로그가 연출됩니다.

### 10개 구역의 정밀한 `/statusline` HUD 바
턴 하단에는 실제 Antigravity CLI의 고유 상태선이 터미널 하단에 정확히 렌더링됩니다:
```text
   READY │ Gemini 3.1 Pro │ Ctx [██░░░░░░░░] 18% │ 3 agents │ 4.1k tokens │ $0.04
```
- 준비 상태 배지(`READY`), 모델명, 10칸 텍스트 프로그레스 바 형태의 컨텍스트 점유율 게이지, 활성 서브에이전트 수, 누적 토큰이 한눈에 표시됩니다.

---

## 2. 실행 명령어

```bash
# Antigravity 스타일 실행
imposter --style antigravity

# 별칭(alias) 사용
imposter --style agy

# 라이트 모드로 실행
imposter -s antigravity --light
```
