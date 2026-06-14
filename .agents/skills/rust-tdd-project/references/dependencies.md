# Dependencies

Use current stable versions and adapt to current APIs. Prefer the Rust standard library when sufficient.

Preferred categories:

```toml
[dependencies]
anyhow = "1"
thiserror = "2"
rustyline = "18"
nix = { version = "0.30", features = ["process", "fs", "term", "signal"] }

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

Rules:

1. Avoid unnecessary dependencies.
2. Avoid wildcard dependencies.
3. Review breaking changes before upgrading.
4. Commit `Cargo.lock` changes.
5. Run `mise run rust:ci` after dependency changes.
