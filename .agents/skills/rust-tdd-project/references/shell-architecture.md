# Shell Architecture

Prefer this flow:

```text
input string
  -> lexer
  -> tokens
  -> parser
  -> AST
  -> expander
  -> executor
```

Suggested modules:

```text
src/main.rs
src/lib.rs
src/ast.rs
src/lexer.rs
src/parser.rs
src/executor.rs
src/builtins.rs
src/env.rs
src/error.rs
```

Keep `main.rs` small. Builtins that mutate shell state, such as `cd`, `exit`, `export`, `jobs`, and `fg`, must run in the shell process.

Use `std::process::Command` first. Use `nix` only for lower-level Unix process, signal, terminal, or file descriptor behavior.

Exit codes:

```text
0    success
1    general error
2    parse error or invalid shell usage
126  command found but not executable
127  command not found
130  interrupted by Ctrl-C
```
