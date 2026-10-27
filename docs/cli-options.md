# CLI 옵션 레퍼런스

`imposter`의 모든 커맨드라인 플래그와 옵션입니다.

```text
Usage: imposter [OPTIONS]

Options:
  -s, --style <NAME>        Visual persona: claude, codex, opencode, pi, antigravity, classic
      --claude              Alias for --style claude
      --classic             Alias for --style classic
  -t, --theme <MODE>        Color theme: dark, light
      --dark                Alias for --theme dark
      --light               Alias for --theme light
  -d, --dev-type <TYPE>     Target codebase persona: backend, frontend, fullstack,
                            data-science, devops, blockchain, machine-learning,
                            systems, game-dev, security
  -T, --duration <MINUTES>  Auto-stop timer in minutes (0 = forever, default 0)
  -a, --alerts              Simulate occasional compiler/lint errors and auto-fixes
  -j, --jargon <LEVEL>      Jargon intensity: low, medium, high, extreme
  -c, --complexity <LEVEL>  Turn complexity & plans: low, medium, high, extreme
  -i, --interactive         Launch the interactive configuration setup
      --save                Save current arguments as defaults to config.toml
      --fast                Disable character typing delays and pacing (for GIF/CI)
      --seed <N>            Fixed PRNG seed for deterministic output
  -h, --help                Print help information
  -v, --version             Print version
```
