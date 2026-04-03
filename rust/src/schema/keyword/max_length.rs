use crate::{error, schema};

fn condition(limit: &schema::common::number::Number, instance: &str) -> bool {
    let len = schema::keyword::get_str_len(instance);
    schema::common::number::Number::ge(limit, &len)
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_max_length(&self, instance: &String) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.max_length.as_ref(),
            instance,
            |limit, string| condition(limit, string.as_str()),
            |limit| error::type_::ValidationErrorType::MaxLength { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxlength/)
    #[rstest]
    #[case::valid_length(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "maxLength": 10
        }),
        None
    )]
    #[case::invalid_length(
        json!("This is an invalid string"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "maxLength": 10
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MaxLength {
                limit: 10.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_length(
        json!("This is valid"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "maxLength": 20
        }),
        None
    )]
    #[case::invalid_length(
        json!("This description is too long"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "string",
                "number"
            ],
            "maxLength": 20
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MaxLength {
                limit: 20.into()
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
            "maxLength": 20
        }),
        None
    )]
    fn test_max_length_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
