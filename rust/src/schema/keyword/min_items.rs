use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_min_items(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.min_items.as_ref(),
            instance,
            schema::common::number::Number::le,
            |limit| error::type_::ValidationErrorType::MinItems { limit },
            false,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::{Value, json};

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minitems/)
    #[rstest]
    #[case::valid_simple(
        json!([
            1,
            true,
            "hello"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "minItems": 3
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            1,
            "apple"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "minItems": 3
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                1,
                "apple"
            ]),
            type_: error::type_::ValidationErrorType::MinItems {
                limit: 3.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!([
            1,
            "John",
            false
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "number"},
                {"type": "string"}
            ],
            "minItems": 3
        }),
        None
    )]
    #[case::invalid_complex(
        json!([
            1,
            "John"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "number"},
                {"type": "string"}
            ],
            "minItems": 3
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                1,
                "John"
            ]),
            type_: error::type_::ValidationErrorType::MinItems {
                limit: 3.into()
            },
            ..Default::default()
        }])
    )]
    fn test_min_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
