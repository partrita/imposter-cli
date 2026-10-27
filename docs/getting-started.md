# 빠른 시작

## 설치

### crates.io에서 설치

```bash
cargo install imposter
```

### Git 저장소에서 빌드 및 설치

```bash
cargo install --git https://github.com/partrita/imposter-cli
```

### 최소 의존성 (TUI 제외, 의존성 0개)

```bash
cargo install imposter --no-default-features
```

---

## 기본 실행

```bash
# 기본 세션 시작 (설정 파일이 없으면 첫 1회 TUI 마법사 실행)
imposter

# 5분 동안 실행 후 자동 종료
imposter --duration 5

# TUI 설정 마법사 직접 실행
imposter -i

# 스타일 선택 실행
imposter --style opencode
imposter --style antigravity
imposter --style pi
imposter --style codex

# 라이트 모드로 실행
imposter --light
```
