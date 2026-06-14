# Releases

Use GitHub CLI for releases. Do not use custom release actions unless `gh` is insufficient.

Required flow:

```bash
git tag v0.1.0
git push origin v0.1.0
mise run release:dry-run -- --tag v0.1.0
mise run release:create -- --tag v0.1.0
```

Release workflow should set `GH_TOKEN`, `RELEASE_TAG`, `RELEASE_TITLE`, and `RELEASE_NOTES`, then call:

```bash
mise run release:create
```

Rules:

- Release artifacts must be built by `mise run release:package`.
- Run `mise run rust:ci` before packaging.
- Verify the tag exists.
- Do not publish artifacts from untested code.
