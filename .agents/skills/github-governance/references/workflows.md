# Workflows

Use latest stable major action versions. Keep workflows thin and call `mise run` tasks.

Required workflows:

```text
.github/workflows/ci.yml
.github/workflows/pr-title.yml
.github/workflows/release.yml
```

CI baseline:

```yaml
name: CI
on:
  pull_request:
  push:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true
jobs:
  rust:
    name: Rust CI
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: jdx/mise-action@v3
        with:
          install: true
          cache: true
      - run: mise run rust:ci
```

PR title baseline:

```yaml
name: PR Title
on:
  pull_request:
    types: [opened, edited, synchronize, reopened, ready_for_review]
permissions:
  contents: read
  pull-requests: read
concurrency:
  group: pr-title-${{ github.event.pull_request.number }}
  cancel-in-progress: true
jobs:
  validate:
    name: Validate PR title
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: jdx/mise-action@v3
        with:
          install: true
          cache: true
      - env:
          PR_TITLE: ${{ github.event.pull_request.title }}
        run: mise run pr:title:validate
```
