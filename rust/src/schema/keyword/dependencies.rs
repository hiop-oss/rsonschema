use crate::{Schemas, ValidationReport, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_dependencies(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        match &self.dependencies {
            Some(dependencies) => match dependencies {
                schema::common::dependencies::Dependencies::Schema(dependent_schemas) => {
                    schema::keyword::dependent_schemas::_validate_dependent_schemas(
                        dependent_schemas,
                        instance,
                        state,
                        relative_schemas,
                        parent_id,
                    )
                }
                schema::common::dependencies::Dependencies::Required(dependent_required) => {
                    let errors = schema::keyword::dependent_required::_validate_dependent_required(
                        dependent_required,
                        instance,
                    );
                    ValidationReport {
                        errors,
                        ..Default::default()
                    }
                }
            },
            None => Default::default(),
        }
    }
}
