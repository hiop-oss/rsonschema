from collections.abc import Callable
from typing import Any

import pytest

from rsonschema import ValidationError, validate

TEST_REF_KEY = "test_ref"

TEST_SCHEMA = {
    "type": "string",
    "minLength": 5,
}

TEST_ROOT_SCHEMA = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    **TEST_SCHEMA,
}


def resolver(ref_: str) -> dict[str, Any] | None:
    if ref_ == TEST_REF_KEY:
        return TEST_ROOT_SCHEMA


@pytest.mark.parametrize(
    argnames=[
        "instance",
        "schema",
        "pointer",
        "ref_resolver",
        "expected",
    ],
    argvalues=[
        [
            "valid",
            TEST_ROOT_SCHEMA,
            None,
            None,
            [],
        ],
        [
            "valid",
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "$defs": {"long_string": TEST_SCHEMA},
            },
            "$defs/long_string",
            None,
            [],
        ],
        [
            "valid",
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "$ref": TEST_REF_KEY,
            },
            None,
            resolver,
            [],
        ],
        [
            {"property": "not"},
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "properties": {"property": TEST_SCHEMA},
            },
            None,
            None,
            [
                ValidationError(
                    message='"not" at `property`: must be longer than `5` characters',
                    pointer=["property"],
                    instance="not",
                ),
            ],
        ],
    ],
)
def test_validate(
    instance: Any,
    schema: dict[str, Any],
    pointer: str | None,
    ref_resolver: Callable | None,
    expected: list,
):
    actual = validate(instance, schema, pointer, ref_resolver)
    assert actual == expected
