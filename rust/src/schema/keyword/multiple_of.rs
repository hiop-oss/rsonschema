use crate::{error, schema};

use bigdecimal::{BigDecimal, Zero};

impl schema::object::ObjectSchema {
    pub(crate) fn validate_multiple_of(
        &self,
        instance: &schema::common::number::Number,
    ) -> Option<error::ValidationError> {
        schema::keyword::simple_validate(
            self.multiple_of.as_ref(),
            instance,
            |a, b| a % b == BigDecimal::zero(),
            |multiple_of| error::type_::ValidationErrorType::MultipleOf { multiple_of },
            true,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::{Number, Value, json};

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/multipleof/)
    #[rstest]
    #[case::valid_integer(
        json!(10),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "multipleOf": 5
        }),
        None
    )]
    #[case::invalid_integer(
        json!(8),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "multipleOf": 5
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MultipleOf {
                multiple_of: 5.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_string(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "multipleOf": 5
        }),
        None
    )]
    #[case::valid_number(
        json!(-8.2),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "multipleOf": 4.1
        }),
        None
    )]
    #[case::invalid_number(
        json!(2.01),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "number",
            "multipleOf": 4.1
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::MultipleOf {
                multiple_of: Number::from_f64(4.1).unwrap().into()
            },
            ..Default::default()
        }])
    )]
    fn test_multiple_of_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
