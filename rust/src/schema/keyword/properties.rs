use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::{Map, Value};
use std::collections;

fn error_map(error: error::ValidationError, property_key: &str) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(
        error::pointer::ValidationErrorPointer::Key(property_key.to_string()),
        error.pointer,
    );
    let type_ = error::type_::ValidationErrorType::Properties {
        property: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_properties<'a>(
        &self,
        instance: &'a Map<String, Value>,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> (ValidationReport, collections::HashSet<&'a String>) {
        match &self.properties {
            Some(properties) => {
                let mut report = ValidationReport::default();
                let mut matched_keys = collections::HashSet::new();
                for (property_key, property_instance) in instance {
                    if let Some(property_schema) = properties.get(property_key) {
                        matched_keys.insert(property_key);
                        let property_report = property_schema
                            .validate(property_instance, state, relative_schemas, parent_id)
                            .map_errors(error_map, property_key);
                        let evaluated_key = Some(report::evaluated::EvaluatedKey::Property(
                            property_key.clone(),
                        ));
                        report.extend(property_report, evaluated_key);
                    }
                }
                (report, matched_keys)
            }
            None => (Default::default(), Default::default()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/properties/)
    #[rstest]
    #[case::valid_simple(
        json!({"name": "John Doe", "age": 21 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"}
            }
        }),
        None
    )]
    #[case::invalid_simple(
        json!({"name": "John Doe", "age": "21"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"}
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!("21"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("age".to_string())],
            type_: error::type_::ValidationErrorType::Properties {
                property: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Number
                    )
                })
            },
        }])
    )]
    #[case::valid_boolean(
        json!({"baz": "baz"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": true,
                "bar": false
            }
        }),
        None
    )]
    #[case::invalid_boolean(
        json!({"foo": "foo", "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": true,
                "bar": false
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!("bar"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("bar".to_string())],
            type_: error::type_::ValidationErrorType::Properties {
                property: Box::new(error::type_::ValidationErrorType::FalseSchema)
            },
        }])
    )]
    #[case::invalid_complex(
        json!({
            "name": [
                "John",
                "Doe"
            ],
            "Age": 21,
            "email": "foo@bar.com"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "patternProperties": {
                "[Aa]ge$": {"type": "number"}
            },
            "additionalProperties": true
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                "John",
                "Doe"
            ]),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("name".to_string())],
            type_: error::type_::ValidationErrorType::Properties {
                property: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::String
                    )
                })
            },
        }])
    )]
    fn test_properties_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
