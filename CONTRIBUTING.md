# Contributing to rsonschema

## Before you submit

1. **Tests must pass.** From the repo root run:

   ```bash
   cargo test
   ```

2. **Format and lint.** Run:

   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

3. **Pre-commit.** We use [pre-commit](https://pre-commit.com/). Run
   `pre-commit install` once; hooks then run `cargo fmt`, `cargo
   clippy`, and `cargo test` on commit, and `markdownlint --fix` on Markdown.
   See [AGENTS.md](AGENTS.md) and
   [.pre-commit-config.yaml](.pre-commit-config.yaml).

## Full guidelines

- **Humans:** Follow the checks above and the structure/conventions described
  in the [Rust docs](https://docs.rs/rsonschema) and in the code (e.g.
  `rust/src/lib.rs`).
- **Automation and agents:** See [AGENTS.md](AGENTS.md) for the full rules.
  Those guidelines are enforced by pre-commit (on commit) and by CI (validate
  workflow).
