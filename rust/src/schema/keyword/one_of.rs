use crate::{Schemas, ValidationReport, error, schema};

use serde_json::Value;

struct OneOfValidator;

impl schema::keyword::ValidableSubSchema for OneOfValidator {
    fn is_valid(n_schemas: usize, n_wrong_schemas: usize) -> bool {
        n_schemas == n_wrong_schemas + 1
    }

    fn get_error_type(
        schema: Box<error::type_::ValidationErrorType>,
    ) -> error::type_::ValidationErrorType {
        error::type_::ValidationErrorType::OneOf {
            schema: Some(schema),
        }
    }

    fn get_error(
        n_schemas: usize,
        n_schemas_errors: usize,
        instance: &Value,
    ) -> Option<error::ValidationError> {
        if n_schemas > n_schemas_errors + 1 {
            let error = error::ValidationError {
                instance: instance.clone(),
                type_: error::type_::ValidationErrorType::OneOf { schema: None },
                ..Default::default()
            };
            Some(error)
        } else {
            None
        }
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_one_of(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        schema::keyword::validate_subschema::<OneOfValidator>(
            self.one_of.as_ref(),
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
            "oneOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": ["foo"]
            }]
        }),
        None
    )]
    #[case::invalid_one(
        json!({"foo": ["foo"] }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "oneOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": ["foo"]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: json!(["foo"]),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
            type_: error::type_::ValidationErrorType::OneOf {
                schema: Some(Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                }))
            },
        }])
    )]
    #[case::valid_multiple(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "oneOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": ["foo"]
            }, {
                "properties": {
                    "bar": {"type": "number"}
                },
                "required": ["bar"]
            }]
        }),
        None
    )]
    #[case::invalid_multiple_less_than_one(
        json!({"foo": 33, "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "oneOf": [{
                "properties": {
                    "foo": {"type": "string"}
                },
                "required": ["foo"]
            }, {
                "properties": {
                    "bar": {"type": "number"}
                },
                "required": ["bar"]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: json!(33),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
            type_: error::type_::ValidationErrorType::OneOf {
                schema: Some(Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                }))
            },
        }])
    )]
    #[case::invalid_multiple_more_than_one(
        json!({"foo": 33, "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "oneOf": [{
                "properties": {
                    "foo": {"type": "number"}
                },
                "required": ["foo"]
            }, {
                "properties": {
                    "bar": {"type": "string"}
                },
                "required": ["bar"]
            }, {
                "properties": {
                    "foo": {
                        "type": "number",
                        "maximum": 25
                    }
                },
            }
        ]}),
        Some(vec![error::ValidationError {
            instance: json!({
                "bar": "bar",
                "foo": 33
            }),
            type_: error::type_::ValidationErrorType::OneOf {
                schema: Default::default()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_nested(
        json!(25),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "oneOf": [{
                "oneOf": [{"type": "number"}]
            }, {
                "oneOf": [{"type": "string"}]
            }]
        }),
        None
    )]
    #[case::invalid_nested(
        json!(["25"]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "oneOf": [{
                "oneOf": [{"type": "number"}]
            }, {
                "oneOf": [{"type": "string"}]
            }]
        }),
        Some(vec![error::ValidationError {
            instance: json!(["25"]),
            type_: error::type_::ValidationErrorType::OneOf {
                schema: Some(Box::new(error::type_::ValidationErrorType::OneOf {
                    schema: Some(Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                           schema::common::type_::Type::Number
                        )
                    }))
                }))
            },
            ..Default::default()
        }])
    )]
    fn test_one_of_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
