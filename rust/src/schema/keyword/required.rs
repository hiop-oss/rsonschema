use crate::{ValidationReport, error, schema};

use serde_json::{Map, Value};
use std::collections;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_required(&self, instance: &Map<String, Value>) -> ValidationReport {
        match self.required.as_ref() {
            Some(required) => {
                let mut report = ValidationReport::default();
                let mut property_names = collections::HashSet::new();
                for required_property in required {
                    report
                        .evaluated
                        .properties
                        .insert(required_property.clone(), Default::default());
                    if !instance.contains_key(required_property) {
                        property_names.insert(required_property);
                    }
                }
                if !property_names.is_empty() {
                    let property_names = property_names
                        .into_iter()
                        .map(ToString::to_string)
                        .collect();
                    let error = error::ValidationError {
                        instance: Value::Object(instance.clone()),
                        type_: error::type_::ValidationErrorType::Required { property_names },
                        ..Default::default()
                    };
                    report.push_error(error);
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/required/)
    #[rstest]
    #[case::valid_simple(
        json!({
            "foo": [
                "bar"
            ],
            "baz": 13
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "required": [
                "foo"
            ]
        }),
        None
    )]
    #[case::invalid_simple(
        json!({"bar": false }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "required": [
                "foo"
            ]
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "bar": false
            }),
            type_: error::type_::ValidationErrorType::Required {
                property_names: Vec::from(["foo".to_string()])
            },
            ..Default::default()
        }])
    )]
    #[case::valid_nested(
        json!({
            "name": "John",
            "address": {
                "city": "New York",
                "country": "USA"
            }
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "address": {
                    "type": "object",
                    "properties": {
                        "city": {"type": "string"},
                        "country": {"type": "string"}
                    },
                    "required": [
                        "city",
                        "country"
                    ]
                }
            },
            "required": [
                "address"
            ]
        }),
        None
    )]
    #[case::invalid_nested(
        json!({
            "name": "Doe",
            "address": {
                "city": "Dallas"
            }
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "address": {
                    "type": "object",
                    "properties": {
                        "city": {"type": "string"},
                        "country": {"type": "string"}
                    },
                    "required": [
                        "city",
                        "country"
                    ]
                }
            },
            "required": [
                "address"
            ]
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "city": "Dallas"
            }),
            pointer: vec![
                error::pointer::ValidationErrorPointer::Key("address".to_string()),
            ],
            type_: error::type_::ValidationErrorType::Properties {
                property: Box::new(error::type_::ValidationErrorType::Required {
                    property_names: Vec::from(["country".to_string()])
                })
            },
        }])
    )]
    fn test_required_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
