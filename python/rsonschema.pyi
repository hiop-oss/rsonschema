from collections.abc import Callable
from typing import Any

class ValidationError:
    """
    The validation error

    :param pointer: The pointer to the value that caused the error
    :param message: The error message
    :param instance: The value that caused the error
    """
    def __init__(
        self,
        pointer: list[str],
        message: str,
        instance: Any,
    ): ...

def validate(
    instance: Any,
    schema: dict[str, Any],
    pointer: str | None,
    ref_resolver: Callable | None,
) -> list[ValidationError]:
    """
    Validate the given JSON instance against the schema
    """

    ...
