# Native File Tasks

Use inline `mise.toml` tasks for short commands. Use native file tasks under `mise/tasks/` for long, complex, or reusable tasks.

Example layout:

```text
mise/tasks/release/package
mise/tasks/release/create
mise/tasks/release/dry-run
mise/tasks/git/hooks/install
mise/tasks/skills/package
```

Use Bash for simple orchestration and Python for complex logic.

Bash header:

```bash
#!/usr/bin/env bash
set -euo pipefail
```

Python tasks should use `argparse`, `logging`, and `subprocess.run(..., check=True)`. Avoid `shell=True` unless strongly justified.

Use `MISE_PROJECT_ROOT` for root detection:

```bash
repo_root="${MISE_PROJECT_ROOT:?MISE_PROJECT_ROOT is not set. Run this task through mise.}"
```
