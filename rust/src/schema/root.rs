use crate::{Schemas, Validable, ValidationReport, schema};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

/// The root object of a JSON Schema document.
/// It differs from the `ObjectSchema` since it contains the metadata about the schema
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RootSchema {
    /// The `$schema` keyword.
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/schema/)
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    schema: Option<schema::common::draft::Draft>,

    /// The actual object schema
    #[serde(flatten)]
    object: schema::object::ObjectSchema,

    /// The `$vocabulary` keyword.
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/vocabulary/)
    #[serde(rename = "$vocabulary", skip_serializing_if = "Option::is_none")]
    vocabulary: Option<IndexMap<Url, bool>>,
}

impl Validable for RootSchema {
    /// Validates the given JSON instance against the schema
    fn validate(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        self.object
            .validate(instance, state, relative_schemas, parent_id)
    }

    fn get_schemas(
        &self,
        parent_id: Option<&schema::common::id::Id>,
        is_absolute: bool,
    ) -> Schemas {
        self.object.get_schemas(parent_id, is_absolute)
    }
}
