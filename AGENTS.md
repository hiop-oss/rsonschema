# Agent guidelines for rsonschema

When editing this repo, follow these rules.
The same requirements apply to human contributors; see [CONTRIBUTING.md](CONTRIBUTING.md).

## Tests and checks are mandatory

- **Do not break tests.** Every change must leave the test suite passing.
- **Before considering a change done**, run from the repo root (in this order):

  ```bash
  cargo fmt
  cargo clippy --all-targets -- -D warnings
  cargo test
  ```

  Fix any failures before marking the work complete. Pre-commit runs these same
  checks on commit; run `pre-commit install` once to enable.

- Tests live in:
  - `rust/src/**/*.rs`: `#[cfg(test)] mod tests { ... }` with unit tests (often
    using `rstest` and `#[case]`).
  - `rust/tests/official.rs`: JSON Schema Test Suite (draft2020-12).
- If you add or change validation behaviour, add or update the relevant tests in
  the same file (or in `rust/tests/` for integration-style tests).

## Codebase style and compliance

- **Match existing style.** Prefer the same patterns, naming, and structure as
  the surrounding code.
- **Documentation:** The crate uses `#![deny(missing_docs)]`. Document public
  items (modules, types, functions, methods) with `///` doc comments.
- **Warnings:** The crate uses `#![deny(warnings)]`. Code must compile with no
  warnings.
- **Formatting:** Use `rustfmt`. Run `cargo fmt` before committing. Pre-commit
  runs `cargo fmt` on commit.
- **Linting:** Run `cargo clippy --all-targets -- -D warnings` and fix any
  reported issues. `--all-targets` includes test code so `#[cfg(test)]` modules
  are linted too. Pre-commit runs this on commit.
- **Markdown:** Repo Markdown (excluding the JSON-Schema-Test-Suite submodule)
  must pass [markdownlint](.markdownlint.yaml); pre-commit runs `markdownlint
  --fix` on commit.
- **Structure:**
  - Schema keywords live under `rust/src/schema/keyword/` as separate modules
    (e.g. `one_of.rs`, `any_of.rs`).
  - Each keyword typically defines a validator type, implements
    `schema::keyword::ValidableSubSchema`, and adds a `validate_*` method on
    `schema::object::ObjectSchema`.
  - Use `crate::` and the existing `error`, `schema`, `ValidationReport`,
    `Schemas` types; follow the same import and type patterns as in neighbouring
    files.
- **Errors:** Use the existing `crate::error` types (e.g. `ValidationError`,
  `ValidationErrorType`); do not introduce new error styles without good reason.
- **Expected errors in tests:** Be consistent with other keyword tests. When
  building an expected `ValidationError` in test cases, set only the fields that
  differ from the default (`instance`, `type_`, and `pointer` when non-empty), and
  use `..Default::default()` for the rest so that default fields (e.g. empty
  `pointer`) are not written explicitly. See e.g. `max_length`, `minimum`, `format`,
  or `dependent_required` tests for the pattern.

## Where this is enforced

- **Pre-commit** (see [.pre-commit-config.yaml](.pre-commit-config.yaml)):
  - **Rust:** `cargo fmt`, `cargo clippy --all-targets -- -D
    warnings`, `cargo test` (each runs once per commit; no filenames passed).
  - **Markdown:** `markdownlint --fix` (submodule
    `rust/tests/JSON-Schema-Test-Suite` is ignored).
  Run `pre-commit install` in the repo root to enable hooks.
- **CI:** The [validate](../.github/workflows/validate.yml) workflow runs the
  shared maturin validate pipeline (build, test, and quality checks).

In short: keep tests green and keep the code consistent with the existing style
and layout.
