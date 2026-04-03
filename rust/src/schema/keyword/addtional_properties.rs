use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::{Map, Value};
use std::collections;

fn error_map(error: error::ValidationError, property_key: &str) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(
        error::pointer::ValidationErrorPointer::Key(property_key.to_string()),
        error.pointer,
    );
    let type_ = error::type_::ValidationErrorType::AdditionalProperties {
        additional_property: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_additional_properties(
        &self,
        instance: &Map<String, Value>,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
        matched_keys: collections::HashSet<&String>,
    ) -> ValidationReport {
        match &self.additional_properties {
            Some(additional_properties) => {
                let mut report = ValidationReport::default();
                for (property_key, property_instance) in instance {
                    if !matched_keys.contains(property_key) {
                        let additional_properties_report = additional_properties
                            .validate(property_instance, state, relative_schemas, parent_id)
                            .map_errors(error_map, property_key);
                        let evaluated_key = Some(report::evaluated::EvaluatedKey::Property(
                            property_key.clone(),
                        ));
                        report.extend(additional_properties_report, evaluated_key);
                    }
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/additionalproperties/)
    #[rstest]
    #[case::valid_boolean(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"type": "string"}
            },
            "additionalProperties": false
        }),
        None
    )]
    #[case::invalid_boolean(
        json!({"foo": "foo", "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"type": "string"}
            },
            "additionalProperties": false
        }),
        Some(vec![error::ValidationError {
            instance: json!("bar"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("bar".to_string())],
            type_: error::type_::ValidationErrorType::AdditionalProperties {
                additional_property: Box::new(error::type_::ValidationErrorType::FalseSchema)
            },
        }])
    )]
    #[case::valid_schema(
        json!({"name": "John Doe", "age": 21 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "name": {"type": "string"}
            },
            "additionalProperties": {
                "type": "number"
            }
        }),
        None
    )]
    #[case::invalid_schema(
        json!({"name": "John Doe", "age": "21"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "name": {"type": "string"}
            },
            "additionalProperties": {
                "type": "number"
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!("21"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("age".to_string())],
            type_: error::type_::ValidationErrorType::AdditionalProperties {
                additional_property: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Number
                    )
                })
            },
        }])
    )]
    fn test_additional_properties_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
