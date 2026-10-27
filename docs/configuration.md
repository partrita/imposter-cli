# 설정 파일 (`config.toml`)

설정은 플랫폼별 표준 경로에 자동 저장됩니다.

- **Linux / macOS**: `~/.config/imposter/config.toml`
- **Windows**: `%APPDATA%\imposter\config.toml`

## 파일 형식 예시

```toml
# imposter configuration
style = "opencode"
theme = "dark"
dev_type = "backend"
duration = 0
jargon = "medium"
complexity = "medium"
alerts = true
```

설정 파일 파서는 외부 의존성(`serde`, `toml` 등) 없이 단 40줄의 순수 Rust 문자열 파서로 구현되어 있어 극도로 가볍습니다.
CLI 옵션을 지정하면 설정 파일의 기본값보다 우선 적용됩니다.
