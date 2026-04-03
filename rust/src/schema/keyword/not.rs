use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_not(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.not.as_ref() {
            Some(not_schema) => {
                let mut report = not_schema.validate(instance, state, relative_schemas, parent_id);
                // `not` is an assertion keyword, not an applicator, so it never contributes
                // to evaluated properties/items regardless of whether it passes or fails
                report.evaluated = Default::default();
                let is_valid = report.is_valid();
                if is_valid {
                    let error = error::ValidationError {
                        instance: instance.clone(),
                        type_: error::type_::ValidationErrorType::Not,
                        ..Default::default()
                    };
                    report.push_error(error);
                } else {
                    report.errors = None;
                }
                report
            }
            None => Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/not/)
    #[rstest]
    #[case::valid_one(
        json!(77),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {
                "type": "string"
            }
        }),
        None
    )]
    #[case::invalid_one(
        json!("foo"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {
                "type": "string"
            }
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Not,
            ..Default::default()
        }])
    )]
    fn test_not_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
