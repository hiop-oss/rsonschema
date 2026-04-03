use crate::{Schemas, Validable, ValidationReport, schema};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value, to_value};
use std::collections;

// Object schema contains all the keys because serde messes up when having nested `deny_unknows_fields` and `flatten`

/// A JSON Schema object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ObjectSchema {
    /// The `additionalProperties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/additionalproperties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) additional_properties: Option<Box<schema::inner::InnerSchema>>,

    /// The `allOf` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/allof/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) all_of: Option<Vec<schema::inner::InnerSchema>>,

    /// The `$anchor` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/anchor/)
    #[serde(rename = "$anchor", skip_serializing_if = "Option::is_none")]
    pub(crate) anchor: Option<String>,

    /// The `anyOf` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/anyof/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) any_of: Option<Vec<schema::inner::InnerSchema>>,

    /// The `$comment` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/comment/)
    #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    comment: Option<String>,

    /// The `const` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/const/)
    #[serde(
        default,
        deserialize_with = "schema::common::option::allow_null",
        rename = "const",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) const_: Option<Value>,

    /// The `contains` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/contains/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) contains: Option<Box<schema::inner::InnerSchema>>,

    /// The `contentEncoding` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/content/contentencoding/)
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    content_encoding: Option<String>,

    /// The `contentMediaType` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/content/contentmediatype/)
    #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    content_media_type: Option<String>,

    /// The `contentSchema` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/content/contentschema/)
    #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    content_schema: Option<Box<schema::inner::InnerSchema>>,

    /// The `default` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/default/)
    #[serde(
        default,
        deserialize_with = "schema::common::option::allow_null",
        skip_serializing_if = "Option::is_none"
    )]
    default: Option<Value>,

    /// The `$defs` keyword (In JSON Schema draft 2019-09 `definitions` was replaced by `$defs`).
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/defs/)
    #[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
    defs: Option<IndexMap<String, schema::inner::InnerSchema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dependencies: Option<schema::common::dependencies::Dependencies>,

    /// The `dependentRequired` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/dependentrequired/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dependent_required: Option<schema::common::dependencies::DependentRequired>,

    /// The `dependentSchemas` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/dependentschemas/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dependent_schemas: Option<schema::common::dependencies::DependentSchemas>,

    /// The `deprecated` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/deprecated/)
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<bool>,

    /// The `description` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/description/)
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// The `$dynamicAnchor` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/dynamicanchor/)
    #[serde(rename = "$dynamicAnchor", skip_serializing_if = "Option::is_none")]
    dynamic_anchor: Option<String>,

    /// The `$dynamicRef` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/dynamicref/)
    #[serde(rename = "$dynamicRef", skip_serializing_if = "Option::is_none")]
    dynamic_ref: Option<String>,

    /// The `else` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/else/)
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub(crate) else_: Option<Box<schema::inner::InnerSchema>>,

    /// The `enum` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/enum/)
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub(crate) enum_: Option<Vec<Value>>,

    /// The `examples` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/examples/)
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<Value>>,

    /// The `exclusiveMaximum` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusivemaximum/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exclusive_maximum: Option<schema::common::number::Number>,

    /// The `exclusiveMinimum` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusiveminimum/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exclusive_minimum: Option<schema::common::number::Number>,

    /// The `format` keyword
    ///
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-assertion/format/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<schema::common::format::Format>,

    /// The `$id` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/id/)
    #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// The `if` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/if/)
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub(crate) if_: Option<Box<schema::inner::InnerSchema>>,

    /// The `items` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/items/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Box<schema::inner::InnerSchema>>,

    /// The `maxContains` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxcontains/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_contains: Option<schema::common::number::Number>,

    /// The `maxItems` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxitems/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_items: Option<schema::common::number::Number>,

    /// The `maxLength` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxlength/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_length: Option<schema::common::number::Number>,

    /// The `maxProperties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxproperties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_properties: Option<schema::common::number::Number>,

    /// The `maximum` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maximum/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) maximum: Option<schema::common::number::Number>,

    /// The `minContains` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/mincontains/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_contains: Option<schema::common::number::Number>,

    /// The `minItems` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minitems/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_items: Option<schema::common::number::Number>,

    /// The `minLength` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minlength/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_length: Option<schema::common::number::Number>,

    /// The `minProperties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minproperties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_properties: Option<schema::common::number::Number>,

    /// The `minimum` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minimum/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) minimum: Option<schema::common::number::Number>,

    /// The `multipleOf` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/multipleof/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) multiple_of: Option<schema::common::number::Number>,

    /// The `not` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/not/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) not: Option<Box<schema::inner::InnerSchema>>,

    /// The `oneOf` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/oneof/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) one_of: Option<Vec<schema::inner::InnerSchema>>,

    /// The `pattern` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/pattern/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pattern: Option<schema::common::regex::Regex>,

    /// The `patternProperties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/properties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pattern_properties:
        Option<IndexMap<schema::common::regex::Regex, schema::inner::InnerSchema>>,

    /// The `prefixItems` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/prefixitems/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) prefix_items: Option<Vec<schema::inner::InnerSchema>>,

    /// The `properties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/properties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) properties: Option<IndexMap<String, schema::inner::InnerSchema>>,

    /// The `propertyNames` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/propertynames/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) property_names: Option<Box<schema::inner::InnerSchema>>,

    /// The `readOnly` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/readonly/)
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,

    /// The `$ref` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/ref/)
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub(crate) ref_: Option<String>,

    /// The `required` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/required/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<Vec<String>>,

    /// The `then` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/then/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) then: Option<Box<schema::inner::InnerSchema>>,

    /// The `title` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/title/)
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// The `type` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/type/)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub(crate) type_: Option<schema::common::type_::SingleOrMultiple<schema::common::type_::Type>>,

    /// The `unevaluatedItems` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluateditems/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unevaluated_items: Option<Box<schema::inner::InnerSchema>>,

    /// The `unevaluatedProperties` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluatedproperties/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unevaluated_properties: Option<Box<schema::inner::InnerSchema>>,

    /// The `uniqueItems` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/uniqueitems/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unique_items: Option<bool>,

    /// The `writeOnly` keyword
    ///
    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/meta-data/writeonly/)
    #[serde(skip_serializing_if = "Option::is_none")]
    write_only: Option<bool>,
}

impl Validable for ObjectSchema {
    fn validate(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
    ) -> ValidationReport {
        let new_id;
        let mut ids = vec![];
        let parent_id = match self.id.as_ref() {
            Some(id) => {
                new_id = schema::common::id::Id::new(parent_id, id);
                ids.push(new_id.to_string());
                Some(&new_id)
            }
            None => parent_id,
        };
        let mut report = ValidationReport {
            ids,
            ..Default::default()
        };
        [
            self.validate_type(instance),
            self.validate_enum(instance),
            self.validate_const(instance),
        ]
        .into_iter()
        .flatten()
        .for_each(|error| {
            report.push_error(error);
        });
        let dependent_required_errors = self.validate_dependent_required(instance);
        report.extend_errors(dependent_required_errors);
        [
            self.validate_ref(instance, state, relative_schemas, parent_id),
            self.validate_not(instance, state, relative_schemas, parent_id),
            self.validate_all_of(instance, state, relative_schemas, parent_id),
            self.validate_any_of(instance, state, relative_schemas, parent_id),
            self.validate_one_of(instance, state, relative_schemas, parent_id),
            self.validate_if(instance, state, relative_schemas, parent_id),
            self.validate_dependent_schemas(instance, state, relative_schemas, parent_id),
            self.validate_dependencies(instance, state, relative_schemas, parent_id),
        ]
        .into_iter()
        .for_each(|inner_report| {
            report.extend(inner_report, None);
        });
        self.validate_by_type(instance, state, relative_schemas, parent_id, report)
    }

    fn get_schemas(
        &self,
        parent_id: Option<&schema::common::id::Id>,
        is_absolute: bool,
    ) -> Schemas {
        match self.id.as_ref() {
            Some(id) => {
                let id = schema::common::id::Id::new(parent_id, id);
                let condition = if is_absolute {
                    id.is_absolute()
                } else {
                    id.is_relative()
                };
                let mut schemas = self.get_inner_schemas(Some(&id), is_absolute);
                if condition {
                    let schema = to_value(self).unwrap();
                    self.add_anchor(Some(&id), Some(&schema), &mut schemas);
                    schemas.insert(id, schema);
                }
                schemas
            }
            None => {
                let mut schemas = self.get_inner_schemas(parent_id, is_absolute);
                let condition = match parent_id {
                    Some(parent_id) => {
                        if is_absolute {
                            parent_id.is_absolute()
                        } else {
                            parent_id.is_relative()
                        }
                    }
                    None => !is_absolute,
                };
                if condition {
                    self.add_anchor(parent_id, None, &mut schemas);
                }
                schemas
            }
        }
    }
}

impl ObjectSchema {
    fn get_inner_schemas(
        &self,
        parent_id: Option<&schema::common::id::Id>,
        is_absolute: bool,
    ) -> Schemas {
        let mut schemas: Schemas = [
            self.items.as_ref(),
            self.unevaluated_items.as_ref(),
            self.contains.as_ref(),
            self.additional_properties.as_ref(),
            self.unevaluated_properties.as_ref(),
            self.property_names.as_ref(),
            self.not.as_ref(),
            self.if_.as_ref(),
            self.then.as_ref(),
            self.else_.as_ref(),
        ]
        .into_iter()
        .flatten()
        .flat_map(|schema| schema.get_schemas(parent_id, is_absolute))
        .collect();
        [
            self.prefix_items.as_ref(),
            self.all_of.as_ref(),
            self.any_of.as_ref(),
            self.one_of.as_ref(),
        ]
        .into_iter()
        .flatten()
        .flatten()
        .flat_map(|schema| schema.get_schemas(parent_id, is_absolute))
        .for_each(|(id, schema)| {
            schemas.insert(id, schema);
        });
        [
            self.defs.as_ref(),
            self.dependent_schemas.as_ref(),
            self.properties.as_ref(),
        ]
        .into_iter()
        .flatten()
        .flat_map(|schemas| {
            schemas
                .values()
                .flat_map(|schema| schema.get_schemas(parent_id, is_absolute))
        })
        .for_each(|(id, schema)| {
            schemas.insert(id, schema);
        });
        self.pattern_properties
            .as_ref()
            .into_iter()
            .flatten()
            .flat_map(|(_, schema)| schema.get_schemas(parent_id, is_absolute))
            .for_each(|(id, schema)| {
                schemas.insert(id, schema);
            });
        self.dependencies
            .as_ref()
            .into_iter()
            .flat_map(|dependencies| dependencies.get_schemas(parent_id, is_absolute))
            .for_each(|(id, schema)| {
                schemas.insert(id, schema);
            });
        schemas
    }

    fn add_anchor(
        &self,
        id: Option<&schema::common::id::Id>,
        schema: Option<&Value>,
        schemas: &mut Schemas,
    ) {
        if let Some(anchor) = self.anchor.as_ref() {
            let id = match id {
                Some(id) => format!("{id}#{anchor}"),
                None => format!("#{anchor}"),
            };
            let id = schema::common::id::Id::new(None, &id);
            let schema = match schema {
                Some(schema) => schema.clone(),
                None => to_value(self).unwrap(),
            };
            schemas.insert(id, schema);
        }
    }

    fn validate_by_type(
        &self,
        instance: &Value,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
        report: ValidationReport,
    ) -> ValidationReport {
        match instance {
            Value::Null | Value::Bool(_) => report,
            Value::Number(number) => self.validate_number(number, report),
            Value::String(string) => self.validate_string(string, report),
            Value::Array(array) => {
                self.validate_array(array, state, relative_schemas, parent_id, report)
            }
            Value::Object(object) => {
                self.validate_object(object, state, relative_schemas, parent_id, report)
            }
        }
    }

    fn validate_string(&self, string: &String, mut report: ValidationReport) -> ValidationReport {
        [
            self.validate_format(string),
            self.validate_min_length(string),
            self.validate_max_length(string),
            self.validate_pattern(string),
        ]
        .into_iter()
        .flatten()
        .for_each(|error| {
            report.push_error(error);
        });
        report
    }

    fn validate_number(&self, number: &Number, mut report: ValidationReport) -> ValidationReport {
        let number = schema::common::number::Number(number.clone());
        [
            self.validate_multiple_of(&number),
            self.validate_exclusive_maximum(&number),
            self.validate_maximum(&number),
            self.validate_exclusive_minimum(&number),
            self.validate_minimum(&number),
        ]
        .into_iter()
        .flatten()
        .for_each(|error| {
            report.push_error(error);
        });
        report
    }

    fn validate_array(
        &self,
        instance: &[Value],
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
        mut report: ValidationReport,
    ) -> ValidationReport {
        let n_items = instance.len().into();
        [
            self.validate_min_items(&n_items),
            self.validate_max_items(&n_items),
        ]
        .into_iter()
        .flatten()
        .for_each(|mut error| {
            let instance = Value::Array(instance.to_vec());
            error.instance = instance;
            report.push_error(error);
        });
        let (prefix_items_report, offset) =
            self.validate_prefix_items(instance, state, relative_schemas, parent_id);
        report.extend(prefix_items_report, None);
        let unique_items_errors = self.validate_unique_items(instance);
        report.extend_errors(unique_items_errors);
        [
            self.validate_contains(instance, state, relative_schemas, parent_id),
            self.validate_items(instance, state, relative_schemas, parent_id, offset),
        ]
        .into_iter()
        .for_each(|inner_report| {
            report.extend(inner_report, None);
        });
        let unevaluated_items_report = self.validate_unevaluated_items(
            instance,
            &report.evaluated.items,
            state,
            relative_schemas,
            parent_id,
        );
        report.extend(unevaluated_items_report, None);
        report
    }

    fn validate_object(
        &self,
        instance: &Map<String, Value>,
        state: &mut schema::common::state::State,
        relative_schemas: &Schemas,
        parent_id: Option<&schema::common::id::Id>,
        mut report: ValidationReport,
    ) -> ValidationReport {
        let validate_properties_result =
            self.validate_properties(instance, state, relative_schemas, parent_id);
        let validate_pattern_properties_result =
            self.validate_pattern_properties(instance, state, relative_schemas, parent_id);
        let n_keys = instance.len().into();
        [
            self.validate_min_properties(&n_keys),
            self.validate_max_properties(&n_keys),
        ]
        .into_iter()
        .flatten()
        .for_each(|mut error| {
            let instance = Value::Object(instance.clone());
            error.instance = instance;
            report.push_error(error);
        });
        let mut matched_keys = collections::HashSet::new();
        for (inner_report, keys) in [
            validate_properties_result,
            validate_pattern_properties_result,
        ] {
            report.extend(inner_report, None);
            matched_keys.extend(keys);
        }
        [
            self.validate_required(instance),
            self.validate_property_names(instance.keys(), state, relative_schemas, parent_id),
            self.validate_additional_properties(
                instance,
                state,
                relative_schemas,
                parent_id,
                matched_keys,
            ),
        ]
        .into_iter()
        .for_each(|inner_report| {
            report.extend(inner_report, None);
        });
        let unevaluated_properties_report = self.validate_unevaluated_properties(
            instance,
            &report.evaluated.properties,
            state,
            relative_schemas,
            parent_id,
        );
        report.extend(unevaluated_properties_report, None);
        report
    }
}
