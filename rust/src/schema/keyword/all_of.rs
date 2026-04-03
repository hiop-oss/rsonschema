use crate::{Schemas, ValidationReport, error, schema};

use serde_json::Value;

struct AllOfValidator;

impl schema::keyword::ValidableSubSchema for AllOfValidator {
    fn is_valid(_: usize, n_wrong_schemas: usize) -> bool {
        n_wrong_schemas == 0
    }

    fn get_error_type(
        schema: Box<error::type_::ValidationErrorType>,
    ) -> error::type_::ValidationErrorType {
        error::type_::ValidationErrorType::AllOf { schema }
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_all_of(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        schema::keyword::validate_subschema::<AllOfValidator>(
            self.all_of.as_ref(),
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/oneof/)
    #[rstest]
    #[case::valid_one(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "allOf": [{
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
            "allOf": [{
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
            type_: error::type_::ValidationErrorType::AllOf {
                schema: Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                }),
            },
        }])
    )]
    #[case::valid_multiple(
        json!({"foo": "foo", "bar": 33 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "allOf": [{
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
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "allOf": [{
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
            instance: json!({
                "foo": "foo"
            }),
            type_: error::type_::ValidationErrorType::AllOf {
                schema: Box::new(error::type_::ValidationErrorType::Required {
                    property_names: Vec::from(["bar".to_string()])
                })
            },
            ..Default::default()
        }])
    )]
    #[case::valid_nested(
        json!(25),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "allOf": [{
                "allOf": [{"type": "number"}]
            },
            {
                "allOf": [{"minimum": 18 }]
            }]
        }),
        None
    )]
    #[case::invalid_nested(
        json!(10),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "allOf": [{
                "allOf": [{"type": "number"}]
            }, {
                "allOf": [{"minimum": 18 }]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::AllOf {
                schema: Box::new(error::type_::ValidationErrorType::AllOf {
                    schema: Box::new(error::type_::ValidationErrorType::Minimum {
                        limit: 18.into()
                    })
                }),
            },
            ..Default::default()
        }])
    )]
    fn test_all_of_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
