use crate::{error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_const(&self, instance: &Value) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.const_.as_ref(),
            instance,
            schema::keyword::is_equal,
            |const_| error::type_::ValidationErrorType::Const { const_ },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/const/)
    #[rstest]
    #[case::valid_string(
        json!("hello"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": "hello"
        }),
        None
    )]
    #[case::invalid_string(
        json!("world"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": "hello"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Const {
                const_: json!("hello")
            },
            ..Default::default()
        }])
    )]
    #[case::valid_number(
        json!(3.15159),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": 3.15159
        }),
        None
    )]
    #[case::invalid_number(
        json!("pi"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": 3.15159
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Const {
                const_: json!(3.15159)
            },
            ..Default::default()
        }])
    )]
    #[case::invalid_empty_object(
        json!({}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": {"name": "John Doe", "age": 30 }
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Const {
                const_: json!({"name": "John Doe", "age": 30 })
            },
            ..Default::default()
        }])
    )]
    #[case::valid_object(
        json!({"name": "John Doe", "age": 30 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": {"name": "John Doe", "age": 30 }
        }),
        None
    )]
    #[case::invalid_object(
        json!({"name": "Robert", "age": 30 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "const": {"name": "John Doe", "age": 30 }
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Const {
                const_: json!({"name": "John Doe", "age": 30 })
            },
            ..Default::default()
        }])
    )]
    fn test_const_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
