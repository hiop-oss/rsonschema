#![deny(missing_docs)]
#![deny(warnings)]

//! # **rsonschema**
//!
//! A fast, simple, user-friendly JSON Schema validator for Rust
//!
//! ## Usage
//!
//! To use this crate, add it to your `Cargo.toml` running:
//!
//! ```bash
//! cargo add rsonschema
//! ```
//!
//! Then start using it in your code:
//!
//! ```rust
//! let schema = serde_json::json!({
//!     "$schema": "https://json-schema.org/draft/2020-12/schema",
//!     "minLength": 3
//! });
//!
//! let instance = serde_json::json!("foo");
//! let report = rsonschema::validate(
//!     &instance,
//!     schema.clone(),
//! );
//! assert!(report.is_valid());
//!
//! let instance = serde_json::json!("a");
//! let report = rsonschema::validate(
//!     &instance,
//!     schema,
//! );
//! assert_eq!(
//!     report,
//!     rsonschema::ValidationReport {
//!         errors: Some(
//!             rsonschema::error::ValidationErrors::from([
//!                 rsonschema::error::ValidationError {
//!                     instance: serde_json::json!("a"),
//!                     type_: rsonschema::error::type_::ValidationErrorType::MinLength {
//!                         limit: 3.into(),
//!                     },
//!                     ..Default::default()
//!                 }
//!             ])
//!         ),
//!         ..Default::default()
//!     }
//! );
//! ```
//!
//! ## Contribute
//!
//! ### FAQ
//!
//! #### Too many arguments?
//!
//! We doubted whether having such a large number of arguments on `validate` method was a correct choice.
//! Currently we suppose this is the best option because every argument has its own different lifetime:
//!
//! - `instance` is the reference to the JSON instance to be validated and it is borrowed from the caller.
//!   It changes for every validation subschema
//! - `state` is a mutable reference to the state of the validation process.
//!   It contains attributes that dont't change during the validation process,
//!   such as the absolute schemas already encountered and the reference resolver
//! - `relative_schemas` is a immutable reference to the relative schemas.
//!   It is fundamental for the `$ref` keyword.
//!   It is immutable because it is evaluated at the beginning the validation process for every new subschema
//! - `parent_id` is a reference to the parent schema id
//!   It is fundamental for the `$ref` keyword.
//!   It is immutable because it changes at every validation subschema

/// The module containing the error that may occurs while validating a JSON instance against a JSON schema
pub mod error;
/// The module containing the report of the validation
mod report;
/// The module containing the schema definitions
pub mod schema;

use either::Either;
use serde_json::Value;
use std::{collections, fmt};

pub use report::ValidationReport;

/// The custom reference resolver
type RefResolver<'a> = &'a dyn Fn(&str) -> Option<Value>;

/// The schemas involved in the validation
type Schemas = collections::HashMap<schema::common::id::Id, Value>;

/// Validate the given JSON instance against the schema
pub fn validate(instance: &Value, schema: Value) -> ValidationReport {
    validate_with_resolver(instance, schema, None, None)
}

/// Validate the given JSON instance against the schema using a custom reference resolver
pub fn validate_with_resolver(
    instance: &Value,
    schema: Value,
    pointer: Option<&str>,
    ref_resolver: Option<RefResolver>,
) -> ValidationReport {
    let validable_schema = schema::common::ref_::get_schema(&schema, None, pointer);
    match validable_schema {
        Either::Left((validable_schema, parent_id)) => {
            let parent_id = parent_id.as_ref();
            let absolute_schemas = validable_schema.get_schemas(parent_id, true);
            let mut relative_schemas = validable_schema.get_schemas(parent_id, false);
            relative_schemas.insert(schema::common::id::Id::Relative(String::new()), schema);
            let ref_resolver = ref_resolver.unwrap_or(&schema::common::ref_::resolve_ref);
            let mut state = schema::common::state::State {
                absolute_schemas,
                ref_resolver,
            };
            validable_schema.validate(instance, &mut state, &relative_schemas, parent_id)
        }
        Either::Right(err) => ValidationReport {
            errors: Some(vec![err]),
            ..Default::default()
        },
    }
}

trait Validable: fmt::Debug {
    fn get_schemas(&self, parent_id: Option<&schema::common::id::Id>, is_absolute: bool)
    -> Schemas;

    fn validate(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport;
}

#[cfg(test)]
mod tests {

    use super::*;

    pub(crate) fn assert_validate(
        instance: Value,
        schema: Value,
        errors: Option<error::ValidationErrors>,
    ) {
        let actual = validate(&instance, schema);
        let expected = ValidationReport {
            errors,
            ids: actual.ids.clone(),
            evaluated: actual.evaluated.clone(),
        };
        assert_eq!(actual, expected);
    }
}
