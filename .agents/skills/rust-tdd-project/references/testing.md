# Testing

Use both unit tests and integration tests.

Unit tests cover lexer, parser, expansion, and small pure behavior. Integration tests cover user-visible shell behavior with `assert_cmd`.

Use `tempfile` for filesystem tests. Avoid hardcoded machine paths.

Avoid long-running or network-dependent commands. Prefer `echo`, `true`, `false`, and `printf`.

Always run:

```bash
mise run rust:ci
```
