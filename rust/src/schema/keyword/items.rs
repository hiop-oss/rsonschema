use crate::{Schemas, Validable, ValidationReport, error, report, schema};

use serde_json::Value;

fn error_map(error: error::ValidationError, index: usize) -> error::ValidationError {
    let pointer = error::pointer::ValidationErrorPointer::prepend(index, error.pointer);
    let type_ = error::type_::ValidationErrorType::Items {
        item: Box::new(error.type_),
    };
    error::ValidationError {
        instance: error.instance,
        pointer,
        type_,
    }
}

impl schema::object::ObjectSchema {
    pub(crate) fn validate_items(
        &self,
        instance: &[Value],
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
        offset: usize,
    ) -> ValidationReport {
        match self.items.as_ref() {
            Some(items) => {
                let mut report = ValidationReport::default();
                for (index, item) in instance.iter().skip(offset).enumerate() {
                    let index = index + offset;
                    let item_report = items
                        .validate(item, state, relative_schemas, parent_id)
                        .map_errors(error_map, index);
                    let evaluated_key = Some(report::evaluated::EvaluatedKey::Item(index));
                    report.extend(item_report, evaluated_key);
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/items/)
    #[rstest]
    #[case::valid_schema(
        json!([
            2,
            3,
            44,
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "items": {"type": "number"}
        }),
        None
    )]
    #[case::invalid_schema(
        json!([
            2,
            3,
            "44",
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "items": {"type": "number"}
        }),
        Some(vec![error::ValidationError {
            instance: json!("44"),
            pointer: vec![error::pointer::ValidationErrorPointer::Index(2)],
            type_: error::type_::ValidationErrorType::Items {
                item: Box::new(error::type_::ValidationErrorType::Type {
                    expected: schema::common::type_::SingleOrMultiple::Single(
                        schema::common::type_::Type::Number
                    )
                })
            },
        }])
    )]
    #[case::valid_boolean(
        json!([
            2,
            3,
            "44",
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "items": true
        }),
        None
    )]
    fn test_items_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
