use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_maximum(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.maximum.as_ref(),
            instance,
            schema::common::number::Number::ge,
            |limit| error::type_::ValidationErrorType::Maximum { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maximum/)
    #[rstest]
    #[case::valid_number(
        json!(9.5),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "maximum": 9.5
        }),
        None
    )]
    #[case::invalid_integer(
        json!(15),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "maximum": 10
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Maximum {
                limit: 10.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_integer(
        json!(20),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "boolean",
                "number"
            ],
            "maximum": 20
        }),
        None
    )]
    #[case::valid_boolean(
        json!(true),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "boolean",
                "number"
            ],
            "maximum": 20.99
        }),
        None
    )]
    fn test_maximum_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
