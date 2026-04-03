use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::Value;

fn error_map(error: error::ValidationError, index: usize) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(index, error.pointer);
    let type_ = error::type_::ValidationErrorType::PrefixItems {
        prefix_item: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_prefix_items(
        &self,
        instance: &[Value],
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> (ValidationReport, usize) {
        match self.prefix_items.as_ref() {
            Some(prefix_items) => {
                let mut report = ValidationReport::default();
                let offset = prefix_items.len();
                for (index, (prefix_item, item)) in prefix_items.iter().zip(instance).enumerate() {
                    let prefix_item_report = prefix_item
                        .validate(item, state, relative_schemas, parent_id)
                        .map_errors(error_map, index);
                    let evaluated_key = Some(report::evaluated::EvaluatedKey::Item(index));
                    report.extend(prefix_item_report, evaluated_key);
                }
                (report, offset)
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/prefixitems/)
    #[rstest]
    #[case::valid_simple(
        json!([
            2,
            false
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "number"}
            ]
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            "2",
            3
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "number"}
            ]
        }),
        Some(vec![error::ValidationError {
            instance: json!("2"),
            pointer: vec![error::pointer::ValidationErrorPointer::Index(0)],
            type_: error::type_::ValidationErrorType::PrefixItems {
                prefix_item: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Number
                    )
                })
            },
        }])
    )]
    #[case::valid_complex(
        json!([
            false,
            "44",
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "boolean"},
                {"type": "string"}
            ],
            "items": {"type": "number"}
        }),
        None
    )]
    #[case::invalid_complex(
        json!([
            2,
            3,
            "44",
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "boolean"},
                {"type": "string"}
            ],
            "items": {"type": "number"}
        }),
        Some(vec![
            error::ValidationError {
                instance: json!(2),
                pointer: vec![error::pointer::ValidationErrorPointer::Index(0)],
                type_: error::type_::ValidationErrorType::PrefixItems {
                    prefix_item: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::Boolean
                        )
                    })
                },
            },
            error::ValidationError {
                instance: json!(3),
                pointer: vec![error::pointer::ValidationErrorPointer::Index(1)],
                type_: error::type_::ValidationErrorType::PrefixItems {
                    prefix_item: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::String
                        )
                    })
                },
            },
            error::ValidationError {
                instance: json!("44"),
                pointer: vec![error::pointer::ValidationErrorPointer::Index(2)],
                type_: error::type_::ValidationErrorType::Items {
                    item: Box::new(error::type_::ValidationErrorType::Type {
                        expected: schema::common::type_::SingleOrMultiple::Single(
                            schema::common::type_::Type::Number
                        )
                    })
                },
            }
        ])
    )]
    fn test_prefix_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
