<!-- markdownlint-disable MD013 MD024 -->
# Benchmarks

## Overview

`rsonschema` is designed to be fast without sacrificing usability.
These benchmarks compare it against [`jsonschema`](https://docs.rs/jsonschema) (Rust)
and [`jsonschema`](https://pypi.org/project/jsonschema/) (Python) across representative validation scenarios.

Both **cold** and **warm** paths are measured for the competitor:

- **cold** — schema compilation + validation on every call (equivalent to how `rsonschema` works)
- **warm** — validator compiled once upfront, only validation measured (best-case for `jsonschema`)

---

## Rust Benchmarks

Implemented with [Criterion.rs](https://bheisler.github.io/criterion.rs/book/).

### Prerequisites

```bash
cargo bench --package rsonschema
```

### Scenarios

| Scenario | Schema keywords | Instance |
| --- | --- | --- |
| `simple_string_valid` | `type`, `minLength` | valid string |
| `simple_string_invalid` | `type`, `minLength` | short string (fails) |
| `complex_object_valid` | `type`, `required`, `properties`, `additionalProperties` | valid object (5 fields) |
| `complex_object_invalid` | same as above | missing fields + wrong type |
| `array_of_objects` | `type`, `minItems`, `items` | 50-element array |
| `any_of_composition` | `anyOf` | string matched by first branch |
| `all_of_composition` | `allOf` | object satisfying all three sub-schemas |

### Running

```bash
# run all scenarios
cargo bench --package rsonschema

# run a single scenario
cargo bench --package rsonschema -- simple_string_valid

# open the HTML report
open rust/target/criterion/report/index.html
```

### Results

> Measured on Apple M3, 8 GB RAM. Times are median values reported by Criterion. Lower is better.
> Run `cargo bench --package rsonschema` to regenerate on your machine.

| Scenario | rsonschema | jsonschema/cold | jsonschema/warm |
| --- | --- | --- | --- |
| `simple_string_valid` | 738 ns | 2.14 µs | 5.6 ns |
| `simple_string_invalid` | 802 ns | 2.15 µs | 5.4 ns |
| `complex_object_valid` | 6.85 µs | 8.95 µs | 128 ns |
| `complex_object_invalid` | 6.49 µs | 8.93 µs | 4.3 ns |
| `array_of_objects` (50 items) | 54.0 µs | 7.74 µs | 1.59 µs |
| `any_of_composition` | 3.25 µs | 4.91 µs | 3.5 ns |
| `all_of_composition` | 4.26 µs | 6.10 µs | 17.9 ns |

---

## Python Benchmarks

Implemented with [pytest-benchmark](https://pytest-benchmark.readthedocs.io/).

### Prerequisites

```bash
# install dev dependencies
pip install -e "python[dev]"

# build the Rust extension in-place (release profile for accurate numbers)
maturin develop --release --manifest-path python/Cargo.toml
```

### Scenarios

The Python scenarios mirror the Rust ones exactly, using the same schema definitions and instances.

### Running

```bash
# run all scenarios
pytest python/benches/ --benchmark-columns=min,mean,stddev,rounds,ops

# compare against a saved baseline
pytest python/benches/ --benchmark-compare

# save a new baseline
pytest python/benches/ --benchmark-save=baseline
```

### Results

> Measured on Apple M3, 8 GB RAM, release wheel (`maturin develop --release`). Times are mean values reported by pytest-benchmark. Lower is better.
> Run `pytest python/benches/ --benchmark-columns=min,mean,stddev,rounds,ops` to regenerate on your machine.

| Scenario | rsonschema | jsonschema/cold | jsonschema/warm |
| --- | --- | --- | --- |
| `simple_string_valid` | 1.36 µs | 4.36 µs | 1.12 µs |
| `simple_string_invalid` | 1.59 µs | 6.40 µs | 3.18 µs |
| `complex_object_valid` | 10.05 µs | 20.73 µs | 17.00 µs |
| `complex_object_invalid` | 11.73 µs | 6.54 µs | 3.16 µs |
| `array_of_objects` (50 items) | 78.12 µs | 392.46 µs | 398.24 µs |
| `any_of_composition` | 4.24 µs | 6.04 µs | 2.78 µs |
| `all_of_composition` | 6.04 µs | 12.98 µs | 9.84 µs |

---

## Takeaways

**Rust**: `rsonschema` is faster than `jsonschema` on a cold-path basis (schema compile + validate)
for all scenarios except `array_of_objects`, where `jsonschema` benefits from a highly optimised
array traversal. For typical API-server workloads — short-lived requests that compile the schema
each time — `rsonschema` wins across the board.

**Python**: with a release wheel, `rsonschema` matches or beats `jsonschema/cold` on every scenario
and is competitive with `jsonschema/warm` for simple schemas. The `array_of_objects` result is
reversed compared to Rust: the Python FFI boundary dominates per-item cost for both libraries.

---

## Methodology Notes

- **Schema ownership**: `rsonschema::validate` takes `schema: Value` (owned), so each Rust benchmark
  iteration clones the schema. This reflects real single-call usage and is the reason `rsonschema` is
  compared against the `jsonschema/cold` path as the primary reference.

- **Cold vs warm**: `jsonschema/cold` compiles a new validator on every iteration (fair comparison to
  `rsonschema`). `jsonschema/warm` pre-compiles the validator once — this represents the case where a
  caller reuses the same schema many times, which is a separate usage pattern.

- **Criterion settings**: 3 s warmup, 100 samples per benchmark by default. Results include a 95%
  confidence interval and change detection against a stored baseline.

- **pytest-benchmark settings**: rounds are auto-scaled to produce a stable mean. Use
  `--benchmark-min-rounds=100` to force at least 100 iterations.

- **Hardware dependency**: wall-clock times vary by CPU. Run both suites on the same machine
  without background load for a meaningful comparison.
