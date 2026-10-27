# 설정 파일 (`config.toml`)

`imposter`는 매번 긴 명령행 인자를 입력하지 않아도 되도록, 사용자가 선호하는 기본값을 파일에 저장하고 자동으로 불러옵니다.

---

## 1. 저장 경로

운영체제별 표준 설정 디렉터리에 자동으로 생성됩니다:

- **Linux**: `~/.config/imposter/config.toml` (또는 `$XDG_CONFIG_HOME/imposter/config.toml`)
- **macOS**: `~/.config/imposter/config.toml` (또는 `~/Library/Application Support/imposter/config.toml`)
- **Windows**: `%APPDATA%\imposter\config.toml` (예: `C:\Users\<사용자>\AppData\Roaming\imposter\config.toml`)

---

## 2. 파일 형식 및 예시

표준적인 TOML 문법의 `key = "value"` 형식으로 구성됩니다:

```toml
# imposter 세션 기본 설정 파일
style = "opencode"
theme = "dark"
dev_type = "backend"
duration = 0
jargon = "medium"
complexity = "medium"
alerts = true
```

### 각 항목의 의미

- `style`: 기본 실행 스타일 (`claude`, `codex`, `opencode`, `pi`, `antigravity`, `classic`)
- `theme`: 기본 색상 테마 (`dark`, `light`)
- `dev_type`: 대상 코드베이스 분야 (`backend`, `frontend`, `fullstack`, `data-science`, `devops`, `blockchain`, `machine-learning`, `systems`, `game-dev`, `security`)
- `duration`: 자동 종료 타이머 분 단위 (`0`은 무한 지속)
- `jargon`: 기술 전문 용어 수준 (`low`, `medium`, `high`, `extreme`)
- `complexity`: 턴 구성 복잡도 (`low`, `medium`, `high`, `extreme`)
- `alerts`: 가짜 린트/컴파일 에러 자동 복구 시나리오 활성화 여부 (`true`, `false`)

---

## 3. 초경량 전용 파서의 특징

`imposter`는 이 설정 파일을 읽고 쓰기 위해 `serde`나 무거운 `toml` 크레이트를 링크하지 않습니다. 내부적으로 단 40줄 내외의 자체 작성 파서를 사용하여 메모리 낭비와 컴파일 시간을 최소화했습니다.

> [!TIP]
> **우선순위 규칙**
> 커맨드라인에서 직접 전달한 플래그(예: `imposter -s pi`)는 설정 파일에 저장된 값보다 항상 최우선으로 적용됩니다.
