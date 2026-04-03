from typing import Any

import jsonschema as jsonschema_lib

from rsonschema import validate

DRAFT = "https://json-schema.org/draft/2020-12/schema"

SIMPLE_SCHEMA: dict[str, Any] = {
    "$schema": DRAFT,
    "type": "string",
    "minLength": 3,
}

COMPLEX_OBJECT_SCHEMA: dict[str, Any] = {
    "$schema": DRAFT,
    "type": "object",
    "required": ["id", "name", "age", "email", "active"],
    "additionalProperties": False,
    "properties": {
        "id": {"type": "integer", "minimum": 1},
        "name": {"type": "string", "minLength": 1},
        "age": {"type": "integer", "minimum": 0, "maximum": 150},
        "email": {"type": "string", "format": "email"},
        "active": {"type": "boolean"},
    },
}

ARRAY_SCHEMA: dict[str, Any] = {
    "$schema": DRAFT,
    "type": "array",
    "minItems": 10,
    "items": {
        "type": "object",
        "required": ["id", "value"],
        "properties": {
            "id": {"type": "integer"},
            "value": {"type": "string"},
        },
    },
}

ANY_OF_SCHEMA: dict[str, Any] = {
    "$schema": DRAFT,
    "anyOf": [
        {"type": "string"},
        {"type": "integer"},
        {"type": "object", "required": ["id"]},
    ],
}

ALL_OF_SCHEMA: dict[str, Any] = {
    "$schema": DRAFT,
    "allOf": [
        {"type": "object"},
        {"required": ["name"]},
        {"properties": {"name": {"type": "string", "minLength": 1}}},
    ],
}

ARRAY_INSTANCE: list[dict[str, Any]] = [
    {"id": i, "value": f"item-{i}"} for i in range(50)
]

# Pre-compiled validators used for the warm-path benchmarks.
_SIMPLE_VALIDATOR = jsonschema_lib.Draft202012Validator(SIMPLE_SCHEMA)
_COMPLEX_VALIDATOR = jsonschema_lib.Draft202012Validator(COMPLEX_OBJECT_SCHEMA)
_ARRAY_VALIDATOR = jsonschema_lib.Draft202012Validator(ARRAY_SCHEMA)
_ANY_OF_VALIDATOR = jsonschema_lib.Draft202012Validator(ANY_OF_SCHEMA)
_ALL_OF_VALIDATOR = jsonschema_lib.Draft202012Validator(ALL_OF_SCHEMA)

_VALID_OBJECT: dict[str, Any] = {
    "id": 1,
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com",
    "active": True,
}
_INVALID_OBJECT: dict[str, Any] = {"id": 1, "name": "Alice", "age": "thirty"}


def _jsonschema_cold(schema: dict[str, Any], instance: Any) -> bool:
    """Compile a fresh validator and validate in one call (cold path)."""
    return jsonschema_lib.Draft202012Validator(schema).is_valid(instance)


# ── simple_string_valid ───────────────────────────────────────────────────────


def test_rsonschema_simple_string_valid(benchmark) -> None:
    benchmark(validate, "hello", SIMPLE_SCHEMA)


def test_jsonschema_cold_simple_string_valid(benchmark) -> None:
    benchmark(_jsonschema_cold, SIMPLE_SCHEMA, "hello")


def test_jsonschema_warm_simple_string_valid(benchmark) -> None:
    benchmark(_SIMPLE_VALIDATOR.is_valid, "hello")


# ── simple_string_invalid ─────────────────────────────────────────────────────


def test_rsonschema_simple_string_invalid(benchmark) -> None:
    benchmark(validate, "hi", SIMPLE_SCHEMA)


def test_jsonschema_cold_simple_string_invalid(benchmark) -> None:
    benchmark(_jsonschema_cold, SIMPLE_SCHEMA, "hi")


def test_jsonschema_warm_simple_string_invalid(benchmark) -> None:
    benchmark(_SIMPLE_VALIDATOR.is_valid, "hi")


# ── complex_object_valid ──────────────────────────────────────────────────────


def test_rsonschema_complex_object_valid(benchmark) -> None:
    benchmark(validate, _VALID_OBJECT, COMPLEX_OBJECT_SCHEMA)


def test_jsonschema_cold_complex_object_valid(benchmark) -> None:
    benchmark(_jsonschema_cold, COMPLEX_OBJECT_SCHEMA, _VALID_OBJECT)


def test_jsonschema_warm_complex_object_valid(benchmark) -> None:
    benchmark(_COMPLEX_VALIDATOR.is_valid, _VALID_OBJECT)


# ── complex_object_invalid ────────────────────────────────────────────────────


def test_rsonschema_complex_object_invalid(benchmark) -> None:
    benchmark(validate, _INVALID_OBJECT, COMPLEX_OBJECT_SCHEMA)


def test_jsonschema_cold_complex_object_invalid(benchmark) -> None:
    benchmark(_jsonschema_cold, COMPLEX_OBJECT_SCHEMA, _INVALID_OBJECT)


def test_jsonschema_warm_complex_object_invalid(benchmark) -> None:
    benchmark(_COMPLEX_VALIDATOR.is_valid, _INVALID_OBJECT)


# ── array_of_objects ──────────────────────────────────────────────────────────


def test_rsonschema_array_of_objects(benchmark) -> None:
    benchmark(validate, ARRAY_INSTANCE, ARRAY_SCHEMA)


def test_jsonschema_cold_array_of_objects(benchmark) -> None:
    benchmark(_jsonschema_cold, ARRAY_SCHEMA, ARRAY_INSTANCE)


def test_jsonschema_warm_array_of_objects(benchmark) -> None:
    benchmark(_ARRAY_VALIDATOR.is_valid, ARRAY_INSTANCE)


# ── any_of_composition ────────────────────────────────────────────────────────


def test_rsonschema_any_of_composition(benchmark) -> None:
    benchmark(validate, "a string", ANY_OF_SCHEMA)


def test_jsonschema_cold_any_of_composition(benchmark) -> None:
    benchmark(_jsonschema_cold, ANY_OF_SCHEMA, "a string")


def test_jsonschema_warm_any_of_composition(benchmark) -> None:
    benchmark(_ANY_OF_VALIDATOR.is_valid, "a string")


# ── all_of_composition ────────────────────────────────────────────────────────


def test_rsonschema_all_of_composition(benchmark) -> None:
    benchmark(validate, {"name": "Alice"}, ALL_OF_SCHEMA)


def test_jsonschema_cold_all_of_composition(benchmark) -> None:
    benchmark(_jsonschema_cold, ALL_OF_SCHEMA, {"name": "Alice"})


def test_jsonschema_warm_all_of_composition(benchmark) -> None:
    benchmark(_ALL_OF_VALIDATOR.is_valid, {"name": "Alice"})
