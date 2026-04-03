use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

fn error_then_map(error: error::ValidationError, _: ()) -> error::ValidationError {
    let type_ = error::type_::ValidationErrorType::Then {
        then: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer: error.pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_if(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.if_.as_ref() {
            Some(if_schema) => {
                let mut report = ValidationReport::default();
                let if_report = if_schema.validate(instance, state, relative_schemas, parent_id);
                match if_report.errors {
                    Some(if_errors) => {
                        if let Some(else_schema) = self.else_.as_ref() {
                            let else_report =
                                else_schema.validate(instance, state, relative_schemas, parent_id);
                            match else_report.errors {
                                Some(else_errors) => {
                                    let error = error::ValidationError {
                                        instance: instance.clone(),
                                        type_: error::type_::ValidationErrorType::Else {
                                            if_: if_errors,
                                            else_: else_errors,
                                        },
                                        ..Default::default()
                                    };
                                    report.push_error(error);
                                }
                                None => {
                                    report.extend(else_report, None);
                                }
                            }
                        }
                    }
                    None => {
                        report.extend(if_report, None);
                        if let Some(then_schema) = self.then.as_ref() {
                            let then_report = then_schema
                                .validate(instance, state, relative_schemas, parent_id)
                                .map_errors(error_then_map, ());
                            report.extend(then_report, None);
                        }
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/if/)
    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/then/)
    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/else/)
    #[rstest]
    #[case::valid_then(
        json!({"foo": "foo", "bar": "bar"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "properties":{
                    "foo": {"const": "foo"}
                }
            },
            "then": {
                "required": [
                    "bar"
                ]
            },
            "else": {
                "required": [
                    "baz"
                ]
            }
        }),
        None
    )]
    #[case::invalid_then(
        json!({"foo": "foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "properties":{
                    "foo": {"const": "foo"}
                }
            },
            "then": {
                "required": [
                    "bar"
                ]
            },
            "else": {
                "required": [
                    "baz"
                ]
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "foo": "foo"
            }),
            type_: error::type_::ValidationErrorType::Then {
                then: Box::new(error::type_::ValidationErrorType::Required {
                    property_names: Vec::from(["bar".to_string()])
                })
            },
            ..Default::default()
        }])
    )]
    #[case::valid_else(
        json!({"foo": "not foo", "baz": "baz"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "properties":{
                    "foo": {"const": "foo"}
                }
            },
            "then": {
                "required": [
                    "bar"
                ]
            },
            "else": {
                "required": [
                    "baz"
                ]
            }
        }),
        None
    )]
    #[case::invalid_else(
        json!({"foo": "not foo"}),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "properties":{
                    "foo": {"const": "foo"}
                }
            },
            "then": {
                "required": [
                    "bar"
                ]
            },
            "else": {
                "required": [
                    "baz"
                ]
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "foo": "not foo"
            }),
            type_: error::type_::ValidationErrorType::Else {
                if_: vec![error::ValidationError {
                    instance: json!("not foo"),
                    pointer: vec![error::pointer::ValidationErrorPointer::Key("foo".to_string())],
                    type_: error::type_::ValidationErrorType::Properties {
                        property: Box::new(
                            error::type_::ValidationErrorType::Const {
                                const_: json!("foo")
                            }
                        )
                    },
                }],
                else_: vec![error::ValidationError {
                    instance: json!({
                        "foo": "not foo"
                    }),
                    type_: error::type_::ValidationErrorType::Required {
                        property_names: Vec::from(["baz".to_string()])
                    },
                    ..Default::default()
                }]
            },
            ..Default::default()
        }])
    )]
    fn test_if(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected);
    }
}
