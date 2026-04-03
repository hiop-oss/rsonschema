use crate::{Schemas, Validable, schema};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub(crate) type DependentRequired = IndexMap<String, Vec<String>>;
pub(crate) type DependentSchemas = IndexMap<String, schema::inner::InnerSchema>;

/// The available JSON Schema types
///
/// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/type/)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase", untagged)]
pub(crate) enum Dependencies {
    /// The `dependentSchemas` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/dependentschemas/)
    Schema(DependentSchemas),

    /// The `dependentRequired` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/dependentrequired/)
    Required(DependentRequired),
}

impl Dependencies {
    pub(crate) fn get_schemas(
        &self,
        parent_id: Option<&schema::common::id::Id>,
        is_absolute: bool,
    ) -> Schemas {
        match self {
            Self::Schema(schema) => schema
                .values()
                .flat_map(|schema| schema.get_schemas(parent_id, is_absolute))
                .collect(),
            Self::Required(_) => Default::default(),
        }
    }
}
