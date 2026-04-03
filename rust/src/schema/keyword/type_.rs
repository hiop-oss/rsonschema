use crate::{error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_type(&self, instance: &Value) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.type_.as_ref(),
            instance,
            |type_, instance| type_.has_type_of(instance),
            |expected| error::type_::ValidationErrorType::Type { expected },
            true,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/type/)
    #[rstest]
    #[case::valid_integer(
        json!(42),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number"
        }),
        None
    )]
    #[case::valid_number(
        json!(3.15),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number"
        }),
        None
    )]
    #[case::valid_string(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Type {
                expected: schema::common::type_::SingleOrMultiple::Single(
                    schema::common::type_::Type::Number
                )
            },
            ..Default::default()
        }])
    )]
    #[case::valid_boolean(
        json!(true),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "boolean",
                "array"
            ]
        }),
        None
    )]
    #[case::invalid_number(
        json!(1234),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "boolean",
                "array"
            ]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Type {
                expected: schema::common::type_::SingleOrMultiple::Multiple(vec![
                    schema::common::type_::Type::Boolean,
                    schema::common::type_::Type::Array
                ])
            },
            ..Default::default()
        }])
    )]
    #[case::valid_array(
        json!(vec![1, 2, 3]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": [
                "boolean",
                "array"
            ]
        }),
        None
    )]
    fn test_type_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
