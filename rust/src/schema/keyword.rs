mod addtional_properties;
mod all_of;
mod any_of;
mod const_;
mod contains;
mod dependencies;
mod dependent_required;
mod dependent_schemas;
mod enum_;
mod exclusive_maximum;
mod exclusive_minimum;
mod format;
mod if_;
mod items;
mod max_items;
mod max_length;
mod max_properties;
mod maximum;
mod min_items;
mod min_length;
mod min_properties;
mod minimum;
mod multiple_of;
mod not;
mod one_of;
mod pattern;
mod pattern_properties;
mod prefix_items;
mod properties;
mod property_names;
mod ref_;
mod required;
mod type_;
mod unevaluated_items;
mod unevaluated_properties;
mod unique_items;

use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde_json::Value;

trait ValidableSubSchema {
    fn is_valid(n_schemas: usize, n_wrong_schemas: usize) -> bool;

    fn get_error_type(
        inner_error: Box<error::type_::ValidationErrorType>,
    ) -> error::type_::ValidationErrorType;

    fn get_error(_: usize, _: usize, _instance: &Value) -> Option<error::ValidationError> {
        None
    }

    fn get_best_report(reports: Vec<ValidationReport>) -> ValidationReport {
        let mut best_report = reports
            .into_iter()
            .min_by(|x, y| x.errors.cmp(&y.errors))
            .unwrap_or_default();
        let best_errors = best_report
            .errors
            .unwrap_or_default()
            .into_iter()
            .map(|error| {
                let type_ = Self::get_error_type(Box::new(error.type_));
                error::ValidationError {
                    instance: error.instance,
                    pointer: error.pointer,
                    type_,
                }
            })
            .collect();
        best_report.errors = Some(best_errors);
        best_report
    }
}

fn validate_subschema<V: ValidableSubSchema>(
    inner_schemas: Option<&Vec<schema::inner::InnerSchema>>,
    instance: &Value,
    state: &mut schema::common::state::State,
    relative_schemas: &Schemas,
    parent_id: Option<&schema::common::id::Id>,
) -> ValidationReport {
    match inner_schemas {
        Some(inner_schemas) => {
            let mut report = ValidationReport::default();
            let mut failed_inner_reports = Vec::new();
            let mut inner_schemas_report = ValidationReport::default();
            for inner_schema in inner_schemas {
                let schema_report =
                    inner_schema.validate(instance, state, relative_schemas, parent_id);
                if schema_report.is_valid() {
                    inner_schemas_report.extend(schema_report, None);
                } else {
                    failed_inner_reports.push(schema_report);
                }
            }
            let n_inner_schemas = inner_schemas.len();
            let n_inner_schemas_reports = failed_inner_reports.len();
            if V::is_valid(n_inner_schemas, n_inner_schemas_reports) {
                report.extend(inner_schemas_report, None);
            } else {
                let inner_error = V::get_error(n_inner_schemas, n_inner_schemas_reports, instance);
                match inner_error {
                    Some(error) => {
                        report.push_error(error);
                    }
                    None => {
                        let best_report = V::get_best_report(failed_inner_reports);
                        report.extend(best_report, None);
                    }
                }
            }
            report
        }
        None => Default::default(),
    }
}

/// Check if two values are equal
fn is_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
        (Value::Object(a), Value::Object(b)) => {
            if a.len() == b.len() {
                for (k, v1) in a {
                    match b.get(k) {
                        Some(v2) => {
                            if !is_equal(v1, v2) {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }
                true
            } else {
                false
            }
        }
        (Value::Array(a), Value::Array(b)) => a.iter().zip(b).all(|(a, b)| is_equal(a, b)),
        _ => a == b,
    }
}

fn get_str_len(instance: &str) -> schema::common::number::Number {
    instance.chars().count().into()
}

/// Validate a value against a condition
fn simple_validate<
    L: Clone,
    R: Clone + Into<Value>,
    ConditionFn: Fn(&L, &R) -> bool,
    ErrorFn: Fn(L) -> error::type_::ValidationErrorType,
>(
    left: Option<&L>,
    right: &R,
    condition: ConditionFn,
    type_fn: ErrorFn,
    error_with_instance: bool,
) -> Option<error::ValidationError> {
    left.filter(|left| !condition(left, right)).map(|left| {
        let instance = if error_with_instance {
            right.clone().into()
        } else {
            Default::default()
        };
        let type_ = type_fn(left.clone());
        error::ValidationError {
            instance,
            type_,
            ..Default::default()
        }
    })
}
