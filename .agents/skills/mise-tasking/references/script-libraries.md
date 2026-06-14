# Script Libraries

Create reusable helper libraries only when shared logic appears in two or more file tasks.

Bash helpers live under:

```text
mise/lib/bash/
```

Python helpers live under:

```text
mise/lib/python/mise_tasks/
```

Libraries must avoid side effects at source/import time. They should define small functions for logging, command execution, project root handling, release artifact discovery, or GitHub CLI helpers.

Python helpers should use only the standard library unless a dependency is explicitly justified.
