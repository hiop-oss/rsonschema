use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;

const DRAFT: &str = "https://json-schema.org/draft/2020-12/schema";

// ── Scenario helpers ─────────────────────────────────────────────────────────

fn simple_string_schema() -> serde_json::Value {
    json!({"$schema": DRAFT, "type": "string", "minLength": 3})
}

fn complex_object_schema() -> serde_json::Value {
    json!({
        "$schema": DRAFT,
        "type": "object",
        "required": ["id", "name", "age", "email", "active"],
        "additionalProperties": false,
        "properties": {
            "id":     {"type": "integer", "minimum": 1},
            "name":   {"type": "string",  "minLength": 1},
            "age":    {"type": "integer", "minimum": 0, "maximum": 150},
            "email":  {"type": "string",  "format": "email"},
            "active": {"type": "boolean"}
        }
    })
}

fn array_schema() -> serde_json::Value {
    json!({
        "$schema": DRAFT,
        "type": "array",
        "minItems": 10,
        "items": {
            "type": "object",
            "required": ["id", "value"],
            "properties": {
                "id":    {"type": "integer"},
                "value": {"type": "string"}
            }
        }
    })
}

fn any_of_schema() -> serde_json::Value {
    json!({
        "$schema": DRAFT,
        "anyOf": [
            {"type": "string"},
            {"type": "integer"},
            {"type": "object", "required": ["id"]}
        ]
    })
}

fn all_of_schema() -> serde_json::Value {
    json!({
        "$schema": DRAFT,
        "allOf": [
            {"type": "object"},
            {"required": ["name"]},
            {"properties": {"name": {"type": "string", "minLength": 1}}}
        ]
    })
}

fn array_instance() -> serde_json::Value {
    let items: Vec<serde_json::Value> = (0..50)
        .map(|i| json!({"id": i, "value": format!("item-{i}")}))
        .collect();
    json!(items)
}

// ── Benchmark groups ──────────────────────────────────────────────────────────

fn bench_simple_string_valid(c: &mut Criterion) {
    let schema = simple_string_schema();
    let instance = json!("hello");
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("simple_string_valid");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_simple_string_invalid(c: &mut Criterion) {
    let schema = simple_string_schema();
    let instance = json!("hi");
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("simple_string_invalid");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_complex_object_valid(c: &mut Criterion) {
    let schema = complex_object_schema();
    let instance = json!({
        "id": 1,
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com",
        "active": true
    });
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("complex_object_valid");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_complex_object_invalid(c: &mut Criterion) {
    let schema = complex_object_schema();
    // missing "email" and "active", wrong type for "age"
    let instance = json!({"id": 1, "name": "Alice", "age": "thirty"});
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("complex_object_invalid");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_array_of_objects(c: &mut Criterion) {
    let schema = array_schema();
    let instance = array_instance();
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("array_of_objects");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_any_of_composition(c: &mut Criterion) {
    let schema = any_of_schema();
    let instance = json!("a string");
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("any_of_composition");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

fn bench_all_of_composition(c: &mut Criterion) {
    let schema = all_of_schema();
    let instance = json!({"name": "Alice"});
    let compiled = jsonschema::validator_for(&schema).expect("valid schema");

    let mut group = c.benchmark_group("all_of_composition");

    group.bench_function("rsonschema", |b| {
        b.iter(|| rsonschema::validate(&instance, black_box(schema.clone())))
    });

    group.bench_function("jsonschema/cold", |b| {
        b.iter(|| {
            jsonschema::validator_for(black_box(&schema))
                .expect("valid schema")
                .is_valid(&instance)
        })
    });

    group.bench_function("jsonschema/warm", |b| {
        b.iter(|| compiled.is_valid(&instance))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_string_valid,
    bench_simple_string_invalid,
    bench_complex_object_valid,
    bench_complex_object_invalid,
    bench_array_of_objects,
    bench_any_of_composition,
    bench_all_of_composition,
);
criterion_main!(benches);
