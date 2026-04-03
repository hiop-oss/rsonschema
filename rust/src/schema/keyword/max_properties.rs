use crate::{error, schema};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_max_properties(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.max_properties.as_ref(),
            instance,
            schema::common::number::Number::ge,
            |limit| error::type_::ValidationErrorType::MaxProperties { limit },
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxproperties/)
    #[rstest]
    #[case::valid_simple(
        json!({"foo": 3, "bar": "hi"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "maxProperties": 2
        }),
        None
    )]
    #[case::invalid_simple(
        json!({
            "foo": 3,
            "bar": "hi",
            "baz": true
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "maxProperties": 2
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "foo": 3,
                "bar": "hi",
                "baz": true
            }),
            type_: error::type_::ValidationErrorType::MaxProperties {
                limit: 2.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!({
            "Age": 21,
            "eligible": true
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "patternProperties": {
                "^[Aa]ge$": {"type": "integer"}
            },
            "additionalProperties": {"type": "boolean"},
            "maxProperties": 2
        }),
        None
    )]
    #[case::invalid_complex(
        json!({
            "Age": 21,
            "eligible": true,
            "isGraduated": true
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "patternProperties": {
                "^[Aa]ge$": {"type": "integer"}
            },
            "additionalProperties": {"type": "boolean"},
            "maxProperties": 2
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "Age": 21,
                "eligible": true,
                "isGraduated": true
            }),
            type_: error::type_::ValidationErrorType::MaxProperties {
                limit: 2.into()
            },
            ..Default::default()
        }])
    )]
    fn test_max_properties_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
