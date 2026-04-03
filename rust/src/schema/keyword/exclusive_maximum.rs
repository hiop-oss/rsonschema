use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_exclusive_maximum(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.exclusive_maximum.as_ref(),
            instance,
            schema::common::number::Number::gt,
            |limit| error::type_::ValidationErrorType::ExclusiveMaximum { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusivemaximum/)
    #[rstest]
    #[case::invalid_integer(
        json!(10),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "exclusiveMaximum": 10
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::ExclusiveMaximum {
                limit: 10.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_integer(
        json!(9),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "exclusiveMaximum": 10
        }),
        None
    )]
    #[case::valid_number(
        json!(15.67),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "exclusiveMaximum": 20.99
        }),
        None
    )]
    #[case::valid_string(
        json!("Hello World!"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "exclusiveMaximum": 20.99
        }),
        None
    )]
    fn test_exclusive_maximum_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
