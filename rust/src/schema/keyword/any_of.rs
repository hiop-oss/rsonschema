use crate::{Schemas, ValidationReport, error, schema};

use serde_json::Value;

struct AnyOfValidator;

impl schema::keyword::ValidableSubSchema for AnyOfValidator {
    fn is_valid(n_schemas: usize, n_wrong_schemas: usize) -> bool {
        n_schemas > n_wrong_schemas
    }

    fn get_error_type(
        schema: Box<error::type_::ValidationErrorType>,
    ) -> error::type_::ValidationErrorType {
        error::type_::ValidationErrorType::AnyOf { schema }
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_any_of(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        schema::keyword::validate_subschema::<AnyOfValidator>(
            self.any_of.as_ref(),
            instance,
            state,
            relative_schemas,
            parent_id,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/anyof/)
    #[rstest]
    #[case::valid_one(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "anyOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": [
                    "foo"
                ]
            }]
        }),
        None
    )]
    #[case::invalid_one(
        json!({
            "foo": [
                "foo"
            ]
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "anyOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": [
                    "foo"
                ]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                "foo"
            ]),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
            type_: error::type_::ValidationErrorType::AnyOf {
                schema: Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                })
            },
        }])
    )]
    #[case::valid_multiple(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "anyOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": [
                    "foo"
                ]
            }, {
                "properties": {
                    "bar": {"type": "number"}
                },
                "required": [
                    "bar"
                ]
            }]
        }),
        None
    )]
    #[case::invalid_multiple(
        json!({"foo": 33, "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "anyOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": [
                    "foo"
                ]
            }, {
                "properties": {
                    "bar": {"type": "number"}
                },
                "required": [
                    "bar"
                ]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: json!(33),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
            type_: error::type_::ValidationErrorType::AnyOf {
                schema: Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                })
            },
        }])
    )]
    #[case::valid_nested(
        json!(25),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "anyOf": [{
                "anyOf": [{"type": "number"}]
            },
            {
                "anyOf": [{"minimum": 18 }]
            }]
        }),
        None
    )]
    fn test_any_of_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
