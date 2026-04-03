<!-- markdownlint-disable MD013 -->
# rsonschema

## Prologue

In the world of data validation,
ensuring your data conforms to a specified structure is crucial.

At [hiop](https://hiop.io),
we sought a language-agnostic format to define how data should be structured,
and JSON Schema stood out as the perfect solution.

This inspired the creation of `rsonschema`, a fast,
simple, and user-friendly JSON Schema validator for Rust

### Why Rust?

Rust is celebrated for its performance and safety capabilities.
These attributes make it an excellent choice for building a fast,
user-friendly, secure, and efficient validator

### Alternatives

- **[jsonschema](https://docs.rs/jsonschema/latest/jsonschema/)**:
was previously our choice,
offering robust validation but suffering from complex error handling. For example:
    1. `jsonschema::error::ValidationError` borrows the `instance` attribute,
    adding complexity.
    2. it lacks of useful error messages for end users,
    especially when validating schemas with
    [Schema Composition](https://json-schema.org/understanding-json-schema/reference/combining)
    failures.

- **[valico](https://docs.rs/valico/latest/valico/)**:
like `jsonschema`,
it has complex error handling.
Moreover it is not actively maintained.

- **[schemars](https://docs.rs/schemars/latest/schemars/)**:
a _de facto_ standard  which inspired us with over 19 million downloads.
However, it lacks of validation APIs

## Usage

### Installation

Add `rsonschema` to your `Cargo.toml`:

```sh
cargo add rsonschema
```

### Example

Here's how you can start using `rsonschema` in your Rust project:

<!-- markdownlint-disable MD013 -->
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
<!-- markdownlint-enable MD013 -->

### Performance

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

### Reliability

Currently,
`rsonschema` supports only the latest
([`2020-12`](https://json-schema.org/draft/2020-12/release-notes))
JSON Schema specification.

All official tests, located in the
[`tests`](https://github.com/hiop-oos/rsonschema/tree/master/tests) folder,
are passing, except for features that are not yet supported

### Compatibility

Currently, `rsonschema` intentionally does not support dynamic keywords such as
[`$dynamicAnchor`](https://www.learnjsonschema.com/2020-12/core/dynamicanchor/)
and
[`$dynamicRef`](https://www.learnjsonschema.com/2020-12/core/dynamicref/),
as these introduce complexity

## Community

### Contribution

We firmly believe that collaboration is the key to innovation!

If you find a bug or have a feature request, please open an issue.
If you want to go further and tackle it,
open a pull request on our GitHub [repository](<https://github.com/hiop-oos/rsonschema>).

### License

`rsonschema` is licensed under the Apache-2.0 License.
See the [LICENSE](./LICENSE) file for more details.
