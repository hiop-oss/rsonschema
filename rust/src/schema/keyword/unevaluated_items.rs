use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::Value;

fn error_map(error: error::ValidationError, index: usize) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(index, error.pointer);
    let type_ = error::type_::ValidationErrorType::UnevaluatedItems {
        unevaluated_item: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_unevaluated_items(
        &self,
        instance: &[Value],
        evaluated_items: &report::evaluated::EvaluatedItems,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.unevaluated_items.as_ref() {
            Some(unevaluated_items) => {
                let mut report = ValidationReport::default();
                for (index, item) in instance.iter().enumerate() {
                    if !evaluated_items.contains_key(&index) {
                        let unevaluated_item_report = unevaluated_items
                            .validate(item, state, relative_schemas, parent_id)
                            .map_errors(error_map, index);
                        let evaluated_key = Some(report::evaluated::EvaluatedKey::Item(index));
                        report.extend(unevaluated_item_report, evaluated_key);
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluateditems/)
    #[rstest]
    #[case::valid_simple(
        json!({"John": 46 }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "unevaluatedItems": false
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            "foo",
            "bar"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "unevaluatedItems": false
        }),
        Some(vec![
            error::ValidationError {
                instance: json!("foo"),
                pointer: vec![error::pointer::ValidationErrorPointer::Index(0)],
                type_: error::type_::ValidationErrorType::UnevaluatedItems {
                    unevaluated_item: Box::new(error::type_::ValidationErrorType::FalseSchema)
                },
            },
            error::ValidationError {
                instance: json!("bar"),
                pointer: vec![error::pointer::ValidationErrorPointer::Index(1)],
                type_: error::type_::ValidationErrorType::UnevaluatedItems {
                    unevaluated_item: Box::new(error::type_::ValidationErrorType::FalseSchema)
                },
            },
        ])
    )]
    #[case::valid_complex(
        json!([
            "foo",
            101,
            false
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "prefixItems": [
                {"type": "string"}
            ],
            "contains": {"type": "number"},
            "unevaluatedItems": {"type": "boolean"}
        }),
        None
    )]
    #[case::invalid_complex(
        json!([
            "foo",
            101,
            [false]
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "prefixItems": [
                {"type": "string"}
            ],
            "contains": {"type": "number"},
            "unevaluatedItems": {"type": "boolean"}
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                false
            ]),
            pointer: vec![error::pointer::ValidationErrorPointer::Index(2)],
            type_: error::type_::ValidationErrorType::UnevaluatedItems {
                unevaluated_item: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Boolean
                    )
                })
            },
        }])
    )]
    fn test_unevaluated_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
