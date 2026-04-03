/// The module containing the definition of validation pointer
pub mod pointer;
/// The module containing the types of errors that may occur during validation
pub mod type_;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{cmp, fmt};

/// The validation errors
pub type ValidationErrors = Vec<ValidationError>;

/// The validation error
#[derive(Clone, Debug, Default, Deserialize, Eq, thiserror::Error, PartialEq, Serialize)]
pub struct ValidationError {
    /// The instance that caused the error
    pub instance: Value,

    /// The pointer to the instance that caused the error
    pub pointer: Vec<pointer::ValidationErrorPointer>,

    /// The type of the error
    pub type_: type_::ValidationErrorType,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pointer_string = if self.pointer.is_empty() {
            None
        } else {
            let pointer = self
                .pointer
                .iter()
                .map(|pointer| format!("`{pointer}`"))
                .collect::<Vec<_>>()
                .join(" -> ");
            let pointer_string = format!("at {}", pointer.trim());
            Some(pointer_string)
        };
        let instance_str = match &self.instance {
            Value::Null => None,
            value => Some(value.to_string()),
        };
        let prefix = match (instance_str, pointer_string) {
            (Some(instance), Some(pointer_string)) => format!("{instance} {pointer_string}"),
            (Some(instance), None) => instance,
            (None, Some(pointer_string)) => pointer_string,
            (None, None) => Default::default(),
        };
        if prefix.is_empty() {
            write!(f, "{}", self.type_,)
        } else {
            write!(f, "{prefix}: {}", self.type_,)
        }
    }
}

impl cmp::PartialOrd for ValidationError {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for ValidationError {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let self_pointer_len = self.pointer.len();
        let other_pointer_len = other.pointer.len();
        if self_pointer_len == other_pointer_len {
            let self_similarity = self.get_similarity();
            let other_similarity = other.get_similarity();
            if self_similarity == other_similarity {
                other.type_.cmp(&self.type_)
            } else {
                other_similarity.partial_cmp(&self_similarity).unwrap()
            }
        } else {
            other_pointer_len.cmp(&self_pointer_len)
        }
    }
}

impl ValidationError {
    fn get_similarity(&self) -> f64 {
        self.type_.get_similarity(&self.instance)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ValidationReport;

    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case(
        ValidationReport {
            errors: Some(ValidationErrors::from([
                ValidationError {
                    instance: "abd".into(),
                    type_: type_::ValidationErrorType::Const {
                        const_: "xyz".into()
                    },
                    ..Default::default()
                },
                ValidationError {
                    instance: "abd".into(),
                    type_: type_::ValidationErrorType::Const {
                        const_: "abc".into()
                    },
                    ..Default::default()
                },
            ])),
            ..Default::default()
        },
        ValidationError {
            instance: "abd".into(),
            type_: type_::ValidationErrorType::Const {
                const_: "abc".into()
            },
            ..Default::default()
        },
    )]
    #[case(
        ValidationReport {
            errors: Some(ValidationErrors::from([
                ValidationError {
                    pointer: vec![pointer::ValidationErrorPointer::Key("key".into())],
                    type_: type_::ValidationErrorType::FalseSchema,
                    ..Default::default()
                },
                ValidationError {
                    type_: type_::ValidationErrorType::FalseSchema,
                    ..Default::default()
                },
            ])),
            ..Default::default()
        },
        ValidationError {
            pointer: vec![pointer::ValidationErrorPointer::Key("key".into())],
            type_: type_::ValidationErrorType::FalseSchema,
            ..Default::default()
        },
    )]
    #[case(
        ValidationReport {
            errors: Some(ValidationErrors::from([
                ValidationError {
                    type_: type_::ValidationErrorType::FalseSchema,
                    ..Default::default()
                },
                ValidationError {
                    type_: type_::ValidationErrorType::Contains,
                    ..Default::default()
                },
            ])),
            ..Default::default()
        },
        ValidationError {
            type_: type_::ValidationErrorType::Contains,
            ..Default::default()
        },
    )]
    #[case(
        ValidationReport {
            errors: Some(ValidationErrors::from([
                ValidationError {
                    instance: json!({"abd": {"key": "value"}}),
                    type_: type_::ValidationErrorType::Required {
                        property_names: Vec::from(["abc".to_string()])
                    },
                    ..Default::default()
                },
                ValidationError {
                    instance: json!({"xyz": {"key": "value"}}),
                    type_: type_::ValidationErrorType::Required {
                        property_names: Vec::from(["abc".to_string()])
                    },
                    ..Default::default()
                },
            ])),
            ..Default::default()
        },
        ValidationError {
            instance: json!({"abd": {"key": "value"}}),
            type_: type_::ValidationErrorType::Required {
                property_names: Vec::from(["abc".to_string()])
            },
            ..Default::default()
        },
    )]
    fn test_cmp(#[case] report: ValidationReport, #[case] expected: ValidationError) {
        let actual = report.errors.unwrap().into_iter().min();
        assert_eq!(actual, Some(expected));
    }

    #[rstest]
    #[case(
        ValidationError {
            instance: Value::Null,
            pointer: vec![],
            type_: type_::ValidationErrorType::FalseSchema,
        },
        "is not allowed",
    )]
    #[case(
        ValidationError {
            instance: "foo".into(),
            pointer: vec![],
            type_: type_::ValidationErrorType::FalseSchema,
        },
        "\"foo\": is not allowed",
    )]
    #[case(
        ValidationError {
            instance: Value::Null,
            pointer: vec![pointer::ValidationErrorPointer::Key("items".into())],
            type_: type_::ValidationErrorType::Contains,
        },
        "at `items`: does not contain the required schema",
    )]
    #[case(
        ValidationError {
            instance: json!({"a": 1}),
            pointer: vec![
                pointer::ValidationErrorPointer::Key("items".into()),
                pointer::ValidationErrorPointer::Index(0),
            ],
            type_: type_::ValidationErrorType::Const {
                const_: "expected".into(),
            },
        },
        "{\"a\":1} at `items` -> `0`: does not match the constant `\"expected\"`",
    )]
    fn test_validation_error_display(#[case] err: ValidationError, #[case] expected: &str) {
        assert_eq!(err.to_string(), expected);
    }
}
