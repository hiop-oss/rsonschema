use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_exclusive_minimum(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.exclusive_minimum.as_ref(),
            instance,
            schema::common::number::Number::lt,
            |limit| error::type_::ValidationErrorType::ExclusiveMinimum { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusiveminimum/)
    #[rstest]
    #[case::invalid_integer(
        json!(5),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "exclusiveMinimum": 5
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::ExclusiveMinimum {
                limit: 5.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_number(
        json!(5.01),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "exclusiveMinimum": 5
        }),
        None
    )]
    #[case::valid_integer(
        json!(15),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "exclusiveMinimum": 10.2
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
            "exclusiveMinimum": 10.2
        }),
        None
    )]
    fn test_exclusive_minimum_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
