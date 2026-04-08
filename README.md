<!-- markdownlint-disable MD013 -->
# rsonschema

[![Crates.io](https://img.shields.io/crates/v/rsonschema)](https://crates.io/crates/rsonschema)
[![docs.rs](https://docs.rs/rsonschema/badge.svg)](https://docs.rs/rsonschema)
[![PyPI](https://img.shields.io/pypi/v/rsonschema)](https://pypi.org/project/rsonschema/)
[![CI](https://github.com/hiop-oss/rsonschema/actions/workflows/validate.yml/badge.svg)](https://github.com/hiop-oss/rsonschema/actions/workflows/validate.yml)
[![License](https://img.shields.io/crates/l/rsonschema)](https://github.com/hiop-oss/rsonschema/blob/master/LICENSE)

A fast, simple, and user-friendly
[JSON Schema](https://json-schema.org/) validator for Rust,
with Python bindings.

## Prologue

In the world of data validation,
ensuring your data conforms to a specified structure is crucial.

At [hiop](https://hiop.io),
we sought a language-agnostic format to define how data should be structured,
and JSON Schema stood out as the perfect solution.

This inspired the creation of `rsonschema`, a fast,
simple, and user-friendly JSON Schema validator for Rust.

### Why Rust?

Rust is celebrated for its performance and safety capabilities.
These attributes make it an excellent choice for building a fast,
user-friendly, secure, and efficient validator.

### Alternatives

- **[jsonschema](https://docs.rs/jsonschema/latest/jsonschema/)**:
was previously our choice,
offering robust validation but suffering from complex error handling. For example:
    1. `jsonschema::error::ValidationError` borrows the `instance` attribute,
    adding complexity.
    2. it lacks useful error messages for end users,
    especially when validating schemas with
    [Schema Composition](https://json-schema.org/understanding-json-schema/reference/combining)
    failures.

- **[valico](https://docs.rs/valico/latest/valico/)**:
like `jsonschema`,
it has complex error handling.
Moreover it is not actively maintained.

- **[schemars](https://docs.rs/schemars/latest/schemars/)**:
a _de facto_ standard for schema generation with over 19 million downloads.
However, it lacks validation APIs.

## Usage

### Rust

Add `rsonschema` to your `Cargo.toml`:

```sh
cargo add rsonschema
```

Here's how you can start using `rsonschema` in your Rust project:

```rust
let schema = serde_json::json!({
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "minLength": 3
});

let instance = serde_json::json!("foo");
let report = rsonschema::validate(
    &instance,
    schema.clone(),
);
assert!(report.is_valid());

let instance = serde_json::json!("a");
let report = rsonschema::validate(
    &instance,
    schema,
);
assert_eq!(
    report,
    rsonschema::ValidationReport {
        errors: Some(
            rsonschema::error::ValidationErrors::from([
                rsonschema::error::ValidationError {
                    instance: serde_json::json!("a"),
                    type_: rsonschema::error::type_::ValidationErrorType::MinLength {
                        limit: 3.into(),
                    },
                    ..Default::default()
                }
            ])
        ),
        ..Default::default()
    }
);
```

### Python

Install from PyPI (requires Python >= 3.10):

```sh
pip install rsonschema
```

```python
import rsonschema

schema = {"$schema": "https://json-schema.org/draft/2020-12/schema", "minLength": 3}

# validate(instance, schema, pointer=None, ref_resolver=None)
errors = rsonschema.validate("foo", schema, None, None)
assert errors == []

errors = rsonschema.validate("a", schema, None, None)
assert len(errors) == 1
assert errors[0].message  # human-readable error description
```

<!-- markdownlint-enable MD013 -->

## Performance

<!-- markdownlint-disable MD013 -->

`rsonschema` is benchmarked against [`jsonschema`](https://docs.rs/jsonschema) across representative scenarios.
Selected results on Apple M3 (lower is better):

| Scenario | rsonschema | jsonschema (cold) |
| --- | --- | --- |
| Simple string validation | 738 ns | 2.14 µs |
| Complex object (5 fields) | 6.85 µs | 8.95 µs |
| Array of 50 objects | 54.0 µs | 7.74 µs |
| `anyOf` composition | 3.25 µs | 4.91 µs |

_Cold_ means the competitor also compiles the schema on every call, matching `rsonschema`'s usage model.
See [BENCHMARKS.md](./BENCHMARKS.md) for the full methodology and results, including Python bindings.

<!-- markdownlint-enable MD013 -->

## Scope

`rsonschema` targets a specific, well-defined subset of JSON Schema:

- **Draft**: only the latest
  ([`2020-12`](https://json-schema.org/draft/2020-12/release-notes))
  specification is supported. Older drafts are not.
- **Validation only**: the library validates instances against schemas
  and reports errors — it does not generate schemas or produce
  annotation output.
- **All standard keywords** are implemented, including schema composition
  (`allOf`, `anyOf`, `oneOf`, `not`), conditionals (`if`/`then`/`else`),
  references (`$ref`, `$anchor`), unevaluated keywords
  (`unevaluatedProperties`, `unevaluatedItems`), and format assertions.
- **Intentionally unsupported**: dynamic keywords
  [`$dynamicAnchor`](https://www.learnjsonschema.com/2020-12/core/dynamicanchor/)
  and
  [`$dynamicRef`](https://www.learnjsonschema.com/2020-12/core/dynamicref/)
  are excluded because they introduce significant complexity
  with limited practical benefit.

All [official JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite) tests,
located in the [`tests`](https://github.com/hiop-oss/rsonschema/tree/master/rust/tests) folder,
pass — except for the unsupported dynamic keywords above.

## Community

### Contribution

We firmly believe that collaboration is the key to innovation!

If you find a bug or have a feature request, please open an issue.
If you want to go further and tackle it,
open a pull request on our GitHub [repository](https://github.com/hiop-oss/rsonschema).

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

### License

`rsonschema` is licensed under the Apache-2.0 License.
See the [LICENSE](./LICENSE) file for more details.
