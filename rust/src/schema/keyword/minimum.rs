use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_minimum(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.minimum.as_ref(),
            instance,
            schema::common::number::Number::le,
            |limit| error::type_::ValidationErrorType::Minimum { limit },
            true,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::{Value, json};

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minimum/)
    #[rstest]
    #[case::valid_number(
        json!(6.1),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "minimum": 6.1
        }),
        None
    )]
    #[case::invalid_integer(
        json!(4),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "minimum": 6
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Minimum {
                limit: 6.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_integer(
        json!(15),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "null",
                "number"
            ],
            "minimum": 15
        }),
        None
    )]
    #[case::valid_null(
        Value::Null,
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "null",
                "number"
            ],
            "minimum": 10.99
        }),
        None
    )]
    fn test_minimum_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
