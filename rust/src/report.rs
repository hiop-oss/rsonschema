/// The module containing the validation definitions for an evaluated value
pub mod evaluated;

use crate::error;

use serde::{Deserialize, Serialize};

/// The report of the validation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationReport {
    /// The errors of the validation
    pub errors: Option<error::ValidationErrors>,

    /// The ids of the schemas encountered during the validation
    pub ids: Vec<String>,

    /// The evaluated values of the validation
    pub evaluated: evaluated::Evaluated,
}

impl ValidationReport {
    /// Whether the validation is valid
    pub fn is_valid(&self) -> bool {
        self.errors.is_none()
    }

    pub(crate) fn extend(&mut self, other: Self, evaluated_key: Option<evaluated::EvaluatedKey>) {
        self.extend_errors(other.errors);
        self.evaluated.extend(other.evaluated, evaluated_key);
        self.ids.extend(other.ids);
    }

    pub(crate) fn push_error(&mut self, error: error::ValidationError) {
        match &mut self.errors {
            Some(errors) => {
                errors.push(error);
            }
            None => {
                self.errors = Some(vec![error]);
            }
        }
    }

    pub(crate) fn extend_errors(&mut self, other_errors: Option<error::ValidationErrors>) {
        if let Some(other_errors) = other_errors {
            match &mut self.errors {
                Some(errors) => {
                    errors.extend(other_errors);
                }
                None => {
                    self.errors = Some(other_errors);
                }
            }
        }
    }

    pub(crate) fn map_errors<
        T: Copy,
        F: Fn(error::ValidationError, T) -> error::ValidationError,
    >(
        mut self,
        map_fn: F,
        key: T,
    ) -> Self {
        if let Some(errors) = self.errors {
            self.errors = Some(errors.into_iter().map(|err| map_fn(err, key)).collect());
        }
        self
    }
}
