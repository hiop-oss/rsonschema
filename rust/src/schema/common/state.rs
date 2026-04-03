use crate::{RefResolver, Schemas, schema};

use serde_json::Value;

pub(crate) struct State<'a> {
    pub(crate) absolute_schemas: Schemas,
    pub(crate) ref_resolver: RefResolver<'a>,
}

impl State<'_> {
    pub(crate) fn get_schema_from_id(
        &mut self,
        id: &schema::common::id::Id,
        relative_schemas: &Schemas,
    ) -> Option<Value> {
        match &id {
            schema::common::id::Id::Absolute(url) => match self.absolute_schemas.get(id) {
                Some(schema) => Some(schema.clone()),
                None => {
                    let url = url.as_str();
                    let schema = (self.ref_resolver)(url);
                    if let Some(schema) = &schema {
                        self.absolute_schemas.insert(id.clone(), schema.clone());
                    }
                    schema
                }
            },
            schema::common::id::Id::Relative(relative) => match relative_schemas.get(id) {
                Some(schema) => Some(schema.clone()),
                None => (self.ref_resolver)(relative),
            },
        }
    }
}
