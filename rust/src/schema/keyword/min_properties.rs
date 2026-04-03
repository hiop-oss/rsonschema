use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_min_properties(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.min_properties.as_ref(),
            instance,
            schema::common::number::Number::le,
            |limit| error::type_::ValidationErrorType::MinProperties { limit },
            false,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::{Value, json};

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minproperties/)
    #[rstest]
    #[case::valid_simple(
        json!({"foo": 3, "bar": "hi"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "minProperties": 1
        }),
        None
    )]
    #[case::invalid_simple(
        json!({}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "minProperties": 1
        }),
        Some(vec![error::ValidationError {
            instance: json!({}),
            type_: error::type_::ValidationErrorType::MinProperties {
                limit: 1.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!({"Age": 22, "name": "John"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "patternProperties": {
                "^[Aa]ge$": {"type": "integer"}
            },
            "additionalProperties": {"type": "string"},
            "minProperties": 2
        }),
        None
    )]
    #[case::invalid_complex(
        json!({"Age": 67 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "patternProperties": {
                "^[Aa]ge$": {"type": "integer"}
            },
            "additionalProperties": {"type": "boolean"},
            "minProperties": 2
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "Age": 67
            }),
            type_: error::type_::ValidationErrorType::MinProperties {
                limit: 2.into()
            },
            ..Default::default()
        }])
    )]
    fn test_min_properties_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
