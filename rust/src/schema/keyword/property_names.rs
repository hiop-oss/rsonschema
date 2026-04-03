use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

fn error_map(error: error::ValidationError, property_key: &str) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(
        error::pointer::ValidationErrorPointer::Key(property_key.to_string()),
        error.pointer,
    );
    let type_ = error::type_::ValidationErrorType::PropertyName {
        property_name: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_property_names<'a, Keys: Iterator<Item = &'a String>>(
        &self,
        instance: Keys,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match &self.property_names {
            Some(property_names) => {
                let mut report = ValidationReport::default();
                for property_key in instance {
                    let property_key_instance = Value::String(property_key.clone());
                    let property_names_report = property_names
                        .validate(&property_key_instance, state, relative_schemas, parent_id)
                        .map_errors(error_map, property_key);
                    report.extend(property_names_report, None);
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/propertynames/)
    #[rstest]
    #[case::valid_schema(
        json!({"foo": "foo", "bar": 33 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": {"maxLength": 3 }
        }),
        None
    )]
    #[case::invalid_schema(
        json!({"name": "John Doe", "age": 21 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": {"maxLength": 3 }
        }),
        Some(vec![error::ValidationError {
            instance: Value::String("name".to_string()),
            pointer: vec![error::pointer::ValidationErrorPointer::Key("name".to_string())],
            type_: error::type_::ValidationErrorType::PropertyName {
                property_name: Box::new(error::type_::ValidationErrorType::MaxLength {
                    limit: 3.into()
                })
            },
        }])
    )]
    #[case::valid_boolean(
        json!({"foo": "foo", "bar": 33 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": true
        }),
        None
    )]
    fn test_property_names_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
