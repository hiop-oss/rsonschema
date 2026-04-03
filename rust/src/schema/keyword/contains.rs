use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_contains(
        &self,
        instance: &[Value],
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self.contains.as_ref() {
            Some(contains) => {
                let mut counter: usize = 0;
                let mut report = ValidationReport::default();
                for (index, item) in instance.iter().enumerate() {
                    let contains_report =
                        contains.validate(item, state, relative_schemas, parent_id);
                    if contains_report.is_valid() {
                        counter += 1;
                        report
                            .evaluated
                            .items
                            .insert(index, contains_report.evaluated.items);
                    }
                }
                let number_counter = schema::common::number::Number::from(counter);
                let instance_value = Value::Array(instance.to_vec());
                [
                    schema::keyword::simple_validate(
                        self.min_contains.as_ref(),
                        &number_counter,
                        schema::common::number::Number::le,
                        |limit| error::type_::ValidationErrorType::MinContains { limit },
                        false,
                    ),
                    schema::keyword::simple_validate(
                        self.max_contains.as_ref(),
                        &number_counter,
                        schema::common::number::Number::ge,
                        |limit| error::type_::ValidationErrorType::MaxContains { limit },
                        false,
                    ),
                ]
                .into_iter()
                .flatten()
                .map(|mut error| {
                    error.instance = instance_value.clone();
                    error
                })
                .for_each(|error| report.push_error(error));
                if counter == 0 && self.min_contains != Some(0.into()) {
                    let error = error::ValidationError {
                        instance: instance_value,
                        type_: error::type_::ValidationErrorType::Contains,
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

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/contains/)
    #[rstest]
    #[case::valid_simple(
        json!([
            "foo",
            3,
            false,
            [
                "bar"
            ],
            -5
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "number"}
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            "foo",
            true
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "number"}
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                "foo",
                true
            ]),
            type_: error::type_::ValidationErrorType::Contains,
            ..Default::default()
        }])
    )]
    fn test_contains_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxcontains/)
    #[rstest]
    #[case::valid_simple(
        json!([
            "Car",
            "Bus",
            1,
            2
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "string"},
            "maxContains": 2
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            "Car",
            "Bus",
            1,
            2,
            "Bike"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "string"},
            "maxContains": 2
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                "Car",
                "Bus",
                1,
                2,
                "Bike"
            ]),
            type_: error::type_::ValidationErrorType::MaxContains {
                limit: 2.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!([
            "John",
            false,
            29,
            {"foo": "bar"},
            [
                5,
                7
            ]
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "maxContains": 2
        }),
        None
    )]
    fn test_max_contains_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/mincontains/)
    #[rstest]
    #[case::valid_simple(
        json!([
            "Car",
            "Bus",
            1,
            2,
            "Bike"
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "string"},
            "minContains": 2
        }),
        None
    )]
    #[case::invalid_simple(
        json!([
            "Car",
            1
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": {"type": "string"},
            "minContains": 2
        }),
        Some(vec![error::ValidationError {
            instance: json!([
                "Car",
                1
            ]),
            type_: error::type_::ValidationErrorType::MinContains {
                limit: 2.into()
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!([
            "John",
            false,
            29,
            {"foo": "bar"},
            [
                5,
                7
            ]
        ]),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "minContains": 2
        }),
        None
    )]
    fn test_min_contains_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
