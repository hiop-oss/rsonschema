use crate::{error, schema};

use serde_json::Value;

fn condition(enum_: &Vec<Value>, instance: &Value) -> bool {
    for item in enum_ {
        if schema::keyword::is_equal(item, instance) {
            return true;
        }
    }
    false
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_enum(&self, instance: &Value) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.enum_.as_ref(),
            instance,
            condition,
            |enum_| error::type_::ValidationErrorType::Enum { enum_ },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/enum/)
    #[rstest]
    #[case::valid_string(
        json!("green"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                "red",
                "green",
                "blue"
            ]
        }),
        None
    )]
    #[case::valid_string(
        json!("black"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                "red",
                "green",
                "blue"
            ]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Enum {
                enum_: vec![
                    json!("red"),
                    json!("green"),
                    json!("blue")
                ]
            },
            ..Default::default()
        }])
    )]
    #[case::valid_integer(
        json!(45),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                2,
                45,
                100
            ]
        }),
        None
    )]
    #[case::valid_integer(
        json!(70),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                2,
                45,
                100
            ]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Enum {
                enum_: vec![
                    json!(2),
                    json!(45),
                    json!(100)
                ]
            },
            ..Default::default()
        }])
    )]
    #[case::valid_string(
        json!("2"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                2,
                45,
                100
            ]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Enum {
                enum_: vec![
                    json!(2),
                    json!(45),
                    json!(100),
                ]
            },
            ..Default::default()
        }])
    )]
    #[case::valid_boolean(
        json!(true),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                "red",
                123,
                true,
                {"foo": "bar"},
                [1, 2],
                null
            ]
        }),
        None
    )]
    #[case::invalid_object(
        json!({"foo": "baz"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "enum": [
                "red",
                123,
                true,
                {"foo": "bar"},
                [1, 2],
                null
            ]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Enum {
                enum_: vec![
                    json!("red"),
                    json!(123),
                    json!(true),
                    json!({"foo": "bar"}),
                    json!(vec![1, 2]),
                    Value::Null
                    ]
            },
            ..Default::default()
        }])
    )]
    fn test_enum_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
