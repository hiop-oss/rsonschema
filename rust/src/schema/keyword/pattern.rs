use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_pattern(&self, instance: &str) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.pattern.as_ref(),
            &instance,
            |pattern, instance| pattern.is_match(instance),
            |pattern| error::type_::ValidationErrorType::Pattern { pattern },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/pattern/)
    #[rstest]
    #[case::valid_pattern(
        json!("john.doe@example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
        }),
        None
    )]
    #[case::invalid_pattern(
        json!("invalid@yahoo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Pattern {
                pattern: r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".try_into().unwrap()
            },
            ..Default::default()
        }])
    )]
    fn test_pattern_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
