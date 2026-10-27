# OpenAI Codex 스타일 (`codex`)

OpenAI의 터미널 에이전트 CLI인 `codex-rs` 인터페이스를 사실감 있게 모사한 스타일입니다.

---

## 1. 주요 레이아웃 특징

### 상단 헤더
간결하면서도 개발자 친화적인 터미널 엠블럼 `>_`과 함께 모델 식별자가 출력됩니다:
```text
>_ OpenAI Codex (codex-rs) · gpt-5-codex
```
하단에는 세션 조작 힌트(`/help for commands · ! for bash`)가 간결하게 배치됩니다.

### 앵글 브래킷 프롬프트
Claude의 `❯`와 달리, Codex 고유의 파란색 앵글 브래킷(`›`) 프롬프트와 함께 사용자 명령이 타이핑됩니다.

### 간결한 3단계 동작 블록
Codex 스타일은 복잡한 도구 호출명을 단순화하여 직관적인 3가지 동사 블록으로 정리합니다:
- `• Explored`: 파일 읽기 및 코드베이스 검색 결과 (`└ Read src/lib.rs (lines 1–140)`)
- `• Edited <file> (+add -del)`: 파일 수정 내역 및 증감 통계, 그리고 깔끔한 인라인 디프
- `• Ran <command>`: 터미널 명령 실행 및 완료 상태 트리 (`└ test result: ok. 42 passed`)

### 사고 중 점자 스피너 (Braille Spinner)
사고 중에는 부드럽게 회전하는 점자(Braille) 10프레임 스피너(`⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`)와 함께 대기 시간이 표시됩니다.

### 작업 시간 분할선 푸터
턴이 종료되면 좌우가 대시 라인으로 채워진 작업 시간 표시선이 나타납니다:
```text
─ Worked for 48s ──────────────────────────────────────────────────────────
   tab to switch agents · esc to interrupt
```

---

## 2. 실행 명령어

```bash
# Codex 스타일로 실행
imposter --style codex

# Codex 스타일을 라이트 모드로 실행
imposter -s codex --light
```
