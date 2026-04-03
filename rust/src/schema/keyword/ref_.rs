use crate::{Schemas, ValidationReport, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_ref(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.ref_.as_ref() {
            Some(ref_) => schema::common::ref_::validate_ref(
                ref_,
                instance,
                state,
                relative_schemas,
                parent_id,
            ),
            None => Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{error, tests};

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/ref/)
    #[rstest]
    #[case::valid_simple(
        json!({"name": "John Doe"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com",
            "type": "object",
            "properties": {
                "name": {"$ref": "#/$defs/string"}
            },
            "required": [
                "name"
            ],
            "$defs": {
                "string": {"type": "string"}
            }
        }),
        None
     )]
    #[case::invalid_simple(
        json!({"name": true }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com",
            "type": "object",
            "properties": {
                "name": {"$ref": "#/$defs/string"}
            },
            "required": [
                "name"
            ],
            "$defs": {
                "string": {"type": "string"}
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!(true),
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
    fn test_ref_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
