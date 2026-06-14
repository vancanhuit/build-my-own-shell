# Script Logging

Native `mise` file tasks must include traceable logs. `mise run <task>` already shows the task name or file-task path, so scripts do not need to log the `mise` task name again.

Logs should include:

- timestamp
- log level
- module or script name
- file name
- function name
- line number
- message

Do not log secrets, tokens, full environment dumps, or sensitive values.

Preferred shape:

```text
2026-06-12T10:30:15Z INFO module=create file=create.py function=main line=42 message="Starting release:create"
2026-06-12T10:30:16Z INFO module=command file=command.py function=run line=12 message="+ mise run rust:ci"
```

## Bash Pattern

Bash source locations are best effort. Prefer Python when exact traceability matters.

```bash
log_timestamp() {
  date -u '+%Y-%m-%dT%H:%M:%SZ'
}

log_source_file() {
  local index="${1:-2}"
  basename "${BASH_SOURCE[$index]:-${BASH_SOURCE[1]:-unknown}}"
}

log_source_function() {
  local index="${1:-2}"
  printf '%s' "${FUNCNAME[$index]:-main}"
}

log_source_line() {
  local index="${1:-1}"
  printf '%s' "${BASH_LINENO[$index]:-0}"
}

log_message() {
  local level="$1"
  shift
  printf '%s %s file=%s function=%s line=%s message="%s"\n' \
    "$(log_timestamp)" "$level" "$(log_source_file 3)" \
    "$(log_source_function 3)" "$(log_source_line 2)" "$*" >&2
}

log_info() { log_message "INFO" "$@"; }
log_warn() { log_message "WARN" "$@"; }
log_error() { log_message "ERROR" "$@"; }
fail() { log_error "$*"; exit 1; }
```

Command helper:

```bash
run_cmd() {
  log_info "+ $*"
  "$@"
}
```

Do not use `set -x` by default.

## Python Pattern

```python
import logging

def configure_logging(level: int = logging.INFO) -> None:
    logging.basicConfig(
        level=level,
        format=(
            "%(asctime)sZ %(levelname)s "
            "module=%(module)s "
            "file=%(filename)s "
            "function=%(funcName)s "
            "line=%(lineno)d "
            'message="%(message)s"'
        ),
        datefmt="%Y-%m-%dT%H:%M:%S",
    )
```
