use crate::{error, schema};

fn condition(limit: &schema::common::number::Number, instance: &str) -> bool {
    let len = schema::keyword::get_str_len(instance);
    schema::common::number::Number::le(limit, &len)
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_min_length(&self, instance: &String) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.min_length.as_ref(),
            instance,
            |limit, string| condition(limit, string.as_str()),
            |limit| error::type_::ValidationErrorType::MinLength { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minlength/)
    #[rstest]
    #[case::valid_length(
        json!("This is a valid string"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "minLength": 5
        }),
        None
    )]
    #[case::invalid_length(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "minLength": 5
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MinLength {
                limit: 5.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_length(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "minLength": 3
        }),
        None
    )]
    #[case::invalid_length(
        json!("hi"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "minLength": 3
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MinLength {
                limit: 3.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_length(
        json!(55),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "minLength": 3
        }),
        None
    )]
    fn test_min_length_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
