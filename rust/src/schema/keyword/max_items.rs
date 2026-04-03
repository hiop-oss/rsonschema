use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_max_items(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.max_items.as_ref(),
            instance,
            schema::common::number::Number::ge,
            |limit| error::type_::ValidationErrorType::MaxItems { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxitems/)
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
            "maxItems": 3
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            1,
            2,
            "apple",
            "banana",
            true
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "maxItems": 3
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                1,
                2,
                "apple",
                "banana",
                true
            ]),
            type_: error::type_::ValidationErrorType::MaxItems {
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
            "maxItems": 3
        }),
        None
    )]
    #[case::invalid_complex(
        json!([
            1,
            "John",
            "Doe",
            false
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "number"},
                {"type": "string"}
            ],
            "maxItems": 3
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                1,
                "John",
                "Doe",
                false
            ]),
            type_: error::type_::ValidationErrorType::MaxItems {
                limit: 3.into()
            },
            ..Default::default()
        }])
    )]
    fn test_max_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
