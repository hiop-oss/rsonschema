use crate::{Schemas, Validable, ValidationReport, error, schema};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A JSON Schema
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum InnerSchema {
    /// A trivial boolean JSON Schema
    ///
    /// The schema `true` matches everything (always passes validation),
    /// whereas the schema `false` matches nothing (always fails validation)
    Bool(bool),

    /// A JSON Schema object
    Object(Box<schema::object::ObjectSchema>),
}

impl Validable for InnerSchema {
    /// Validates the given JSON instance against the schema
    fn validate(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match self {
            Self::Bool(true) => Default::default(),
            Self::Bool(false) => {
                let error = error::ValidationError {
                    instance: instance.clone(),
                    type_: error::type_::ValidationErrorType::FalseSchema,
                    ..Default::default()
                };
                ValidationReport {
                    errors: Some(vec![error]),
                    ..Default::default()
                }
            }
            Self::Object(object) => object.validate(instance, state, relative_schemas, parent_id),
        }
    }

    fn get_schemas(
        &self,
        parent_id: Option<&schema::common::id::Id>,
        is_absolute: bool,
    ) -> Schemas {
        match self {
            Self::Bool(_) => Default::default(),
            Self::Object(object) => object.get_schemas(parent_id, is_absolute),
        }
    }
}
