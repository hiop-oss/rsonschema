use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_dependent_schemas(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match &self.dependent_schemas {
            Some(dependent_schemas) => _validate_dependent_schemas(
                dependent_schemas,
                instance,
                state,
                relative_schemas,
                parent_id,
            ),
            None => Default::default(),
        }
    }
}

fn error_map(error: error::ValidationError, property_key: &str) -> error::ValidationError {
    let type_ = error::type_::ValidationErrorType::DependentSchema {
        dependent_property: property_key.to_string(),
        schema: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer: error.pointer,
        type_,
    }
}

pub(crate) fn _validate_dependent_schemas(
    dependent_schemas: &schema::common::dependencies::DependentSchemas,
    instance: &Value,
    state: &mut schema::common::state::State,
    relative_schemas: &Schemas,
    parent_id: Option<&schema::common::id::Id>,
) -> ValidationReport {
    let mut report = ValidationReport::default();
    for (property_key, dependent_schema) in dependent_schemas {
        if instance.get(property_key).is_some() {
            let dependent_schema_report = dependent_schema
                .validate(instance, state, relative_schemas, parent_id)
                .map_errors(error_map, property_key);
            report.extend(dependent_schema_report, None);
        }
    }
    report
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/dependentschemas/)
    #[rstest]
    #[case::valid_simple(
        json!({
            "name": "John",
            "age": 25,
            "license": "XYZ123"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "license": {"type": "string"}
            },
            "dependentSchemas": {
                "license": {
                    "properties": {
                    "age": {"type": "number"}
                },
                "required": ["age"]
                }
            }
        }),
        None
    )]
    #[case::invalid_simple(
        json!({
            "name": "John",
            "age": "25",
            "license": "XYZ123"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "license": {"type": "string"}
            },
            "dependentSchemas": {
                "license": {
                    "properties": {
                        "age": {"type": "number"}
                    },
                    "required": [
                        "age"
                    ]
                }
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!("25"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("age".to_string())],
            type_: error::type_::ValidationErrorType::DependentSchema {
                dependent_property: "license".to_string(),
                schema: Box::new(error::type_::ValidationErrorType::Properties {
                    property: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::Number
                        )
                    })
                })
            },
        }])
    )]
    #[case::valid_nested(
        json!({
            "name": "John",
            "age": 15,
            "eligible": false
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "dependentSchemas": {
                "name": {
                    "properties": {
                        "age": {"type": "number"}
                    },
                    "dependentSchemas": {
                        "age": {
                            "properties": {
                                "eligible": {"type": "boolean"}
                            },
                            "required": [
                                "eligible"
                            ]
                        }
                    },
                    "required": [
                        "age"
                    ]
                }
            }
        }),
        None
    )]
    #[case::invalid_nested(
        json!({
            "name": "manager",
            "age": 25
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "dependentSchemas": {
                "name": {
                    "properties": {
                        "age": {"type": "number"}
                    },
                    "dependentSchemas": {
                        "age": {
                            "properties": {
                                "eligible": {"type": "boolean"}
                            },
                            "required": [
                                "eligible"
                            ]
                        }
                    },
                    "required": [
                        "age"
                    ]
                }
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "name": "manager",
                "age": 25
            }),
            type_: error::type_::ValidationErrorType::DependentSchema {
                dependent_property: "name".to_string(),
                schema: Box::new(error::type_::ValidationErrorType::DependentSchema {
                    dependent_property: "age".to_string(),
                    schema: Box::new(error::type_::ValidationErrorType::Required {
                        property_names: Vec::from(["eligible".to_string()])
                    })
                })
            },
            ..Default::default()
        }])
    )]
    fn test_dependent_schemas_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
