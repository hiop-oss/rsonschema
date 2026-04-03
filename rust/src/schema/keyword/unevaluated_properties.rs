use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::{Map, Value};

fn error_map(error: error::ValidationError, property_key: &str) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(
        error::pointer::ValidationErrorPointer::Key(property_key.to_string()),
        error.pointer,
    );
    let type_ = error::type_::ValidationErrorType::UnevaluatedProperties {
        unevaluated_property: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_unevaluated_properties(
        &self,
        instance: &Map<String, Value>,
        evaluated_properties: &report::evaluated::EvaluatedProperties,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.unevaluated_properties.as_ref() {
            Some(unevaluated_properties) => {
                let mut report = ValidationReport::default();
                for (property_key, property_instance) in instance {
                    if !evaluated_properties.contains_key(property_key) {
                        let property_report = unevaluated_properties
                            .validate(property_instance, state, relative_schemas, parent_id)
                            .map_errors(error_map, property_key);
                        let evaluated_key = Some(report::evaluated::EvaluatedKey::Property(
                            property_key.clone(),
                        ));
                        report.extend(property_report, evaluated_key);
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluatedproperties/)
    #[rstest]
    #[case::valid_simple(
        json!([
            "John",
            46,
            false
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "unevaluatedProperties": false
        }),
        None
    )]
    #[case::invalid_simple(
        json!({"foo": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "unevaluatedProperties": false
        }),
        Some(vec![error::ValidationError {
            instance: json!("bar"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
            type_: error::type_::ValidationErrorType::UnevaluatedProperties {
                unevaluated_property: Box::new(error::type_::ValidationErrorType::FalseSchema)
            },
        }])
    )]
    #[case::valid_complex(
        json!({"foo": "foo", "bar": 36, "fooBar": false }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"type": "string"}
            },
            "patternProperties": {
                "^b": {"type": "number"}
            },
            "unevaluatedProperties": {"type": "boolean"}
        }),
        None
    )]
    #[case::invalid_complex(
        json!({"foo": "foo", "bar": 36, "fooBar": "string"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"type": "string"}
            },
            "patternProperties": {
                "^b": {"type": "number"}
            },
            "unevaluatedProperties": {"type": "boolean"}
        }),
        Some(vec![error::ValidationError {
            instance: json!("string"),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("fooBar".to_string())],
            type_: error::type_::ValidationErrorType::UnevaluatedProperties {
                unevaluated_property: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Boolean
                    )
                })
            },
        }])
    )]
    fn test_unevaluated_properties_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
