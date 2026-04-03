use crate::{error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_unique_items(
        &self,
        instance: &[Value],
    ) -> Option<error::ValidationErrors> {
        if Some(true) == self.unique_items {
            let mut unique_items = Vec::new();
            let mut errors = Vec::new();
            for (index, item) in instance.iter().enumerate() {
                if unique_items.contains(&item) {
                    let error = error::ValidationError {
                        instance: item.clone(),
                        pointer: vec![error::pointer::ValidationErrorPointer::Index(index)],
                        type_: error::type_::ValidationErrorType::UniqueItems,
                    };
                    errors.push(error)
                } else {
                    unique_items.push(item);
                }
            }
            if errors.is_empty() {
                None
            } else {
                Some(errors)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/uniqueitems/)
    #[rstest]
    #[case::valid_simple(
        json!([
            1,
            "hello",
            true
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "uniqueItems": true
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            false,
            "world",
            2,
            2
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "uniqueItems": true
        }),
        Some(vec![error::ValidationError {
            instance: json!(2),
            pointer: vec![error::pointer::ValidationErrorPointer::Index(3)],
            type_: error::type_::ValidationErrorType::UniqueItems,
        }])
    )]
    #[case::valid_complex(
        json!([
            {
                "id": 1,
                "name": "John"
            },
            {
                "id": 2,
                "name": "Doe"
            }
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"}
                },
                "required": [
                    "id",
                    "name"
                ]
            },
            "uniqueItems": true
        }),
        None
    )]
    #[case::invalid_complex(
        json!([
            {
                "id": 1,
                "name": "Jane"
            },
            {
                "id": 1,
                "name": "Jane"
            }
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"}
                },
                "required": [
                    "id",
                    "name"
                ]
            },
            "uniqueItems": true
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "id": 1,
                "name": "Jane"
            }),
            pointer: vec![error::pointer::ValidationErrorPointer::Index(1)],
            type_: error::type_::ValidationErrorType::UniqueItems,
        }])
    )]
    fn test_unique_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
