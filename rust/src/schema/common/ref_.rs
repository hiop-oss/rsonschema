use crate::{Schemas, Validable, ValidationReport, error, schema};

use either::Either;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use serde_json::{Value, from_reader};
use std::fs;

fn get_remote_schema(ref_: &str) -> Option<Value> {
    if let Ok(mut response) = ureq::get(ref_).call()
        && let Ok(schema) = response.body_mut().read_json()
    {
        Some(schema)
    } else {
        None
    }
}

fn get_local_schema(ref_: &str) -> Option<Value> {
    if let Ok(file) = fs::File::open(ref_)
        && let Ok(schema) = from_reader(file)
    {
        Some(schema)
    } else {
        None
    }
}

fn get_schema_from_pointer<'a>(
    mut schema: &'a Value,
    mut parent_id: Option<schema::common::id::Id>,
    pointer: Option<&str>,
) -> Either<(&'a Value, Option<schema::common::id::Id>), error::ValidationError> {
    if let Some(pointer) = pointer {
        for key in pointer.split("/") {
            let key = percent_decode_str(key)
                .decode_utf8_lossy()
                .replace("~0", "~")
                .replace("~1", "/");
            let result = schema::common::id::extract_from_schema(schema, parent_id);
            match result {
                Either::Left(id) => {
                    parent_id = id;
                }
                Either::Right(error) => {
                    return Either::Right(error);
                }
            }
            let result = get_schema_from_key(schema, &key, pointer);
            match result {
                Either::Left(inner) => {
                    schema = inner;
                }
                Either::Right(error) => {
                    return Either::Right(error);
                }
            }
        }
    }
    Either::Left((schema, parent_id))
}

fn get_schema_from_key<'a>(
    schema: &'a Value,
    key: &str,
    ref_: &str,
) -> Either<&'a Value, error::ValidationError> {
    match schema {
        Value::Object(map) => {
            if let Some(inner) = map.get(key) {
                return Either::Left(inner);
            }
        }
        Value::Array(arr) => {
            if let Ok(index) = key.parse::<usize>()
                && let Some(inner) = arr.get(index)
            {
                return Either::Left(inner);
            }
        }
        _ => {}
    }
    let error = get_ref_error(ref_);
    Either::Right(error)
}

fn parse_schema(schema: &Value) -> Either<Box<dyn Validable>, error::ValidationError> {
    let is_root = ["$schema", "$vocabulary"]
        .iter()
        .any(|key| schema.get(key).is_some());
    let result: Result<Box<dyn Validable>, _> = if is_root {
        schema::root::RootSchema::deserialize(schema)
            .map(|root_schema| Box::new(root_schema) as Box<dyn Validable>)
    } else {
        schema::inner::InnerSchema::deserialize(schema)
            .map(|inner_schema| Box::new(inner_schema) as Box<dyn Validable>)
    };
    let result = result.map_err(|err| error::ValidationError {
        type_: error::type_::ValidationErrorType::from_de_error(err, schema.clone()),
        ..Default::default()
    });
    match result {
        Ok(schema) => Either::Left(schema),
        Err(error) => Either::Right(error),
    }
}

pub(crate) fn resolve_ref(ref_: &str) -> Option<Value> {
    match get_remote_schema(ref_) {
        Some(schema) => Some(schema),
        None => get_local_schema(ref_),
    }
}

pub(crate) fn get_schema(
    schema: &Value,
    parent_id: Option<schema::common::id::Id>,
    pointer: Option<&str>,
) -> Either<(Box<dyn Validable>, Option<schema::common::id::Id>), error::ValidationError> {
    get_schema_from_pointer(schema, parent_id, pointer).left_and_then(|(schema, parent_id)| {
        parse_schema(schema).map_left(|schema| (schema, parent_id))
    })
}

pub(crate) fn validate_ref(
    ref_: &str,
    instance: &Value,
    state: &mut schema::common::state::State,
    relative_schemas: &Schemas,
    parent_id: Option<&schema::common::id::Id>,
) -> ValidationReport {
    let (id, pointer) = get_id_and_pointer(ref_);
    let id = clean_id(id);
    let is_relative = id.is_empty();
    let (schema, id) = if is_relative {
        let id = schema::common::id::Id::Relative(String::new());
        let schema = relative_schemas.get(&id);
        (schema.cloned(), id)
    } else {
        let id = schema::common::id::Id::new(parent_id, id);
        let schema = state.get_schema_from_id(&id, relative_schemas);
        (schema, id)
    };
    if let Some(schema) = schema
        && let Either::Left((validable_schema, parent_id)) = get_schema(&schema, Some(id), pointer)
    {
        let parent_id = parent_id.as_ref();
        if is_relative {
            validable_schema.validate(instance, state, relative_schemas, parent_id)
        } else {
            let mut relative_schemas = validable_schema.get_schemas(None, false);
            relative_schemas.insert(schema::common::id::Id::Relative(String::new()), schema);
            validable_schema.validate(instance, state, &relative_schemas, parent_id)
        }
    } else {
        let error = get_ref_error(ref_);
        ValidationReport {
            errors: Some(vec![error]),
            ..Default::default()
        }
    }
}

fn get_ref_error(ref_: &str) -> error::ValidationError {
    error::ValidationError {
        type_: error::type_::ValidationErrorType::Ref {
            ref_: ref_.to_string(),
        },
        ..Default::default()
    }
}

fn get_id_and_pointer(ref_: &str) -> (&str, Option<&str>) {
    match ref_.split_once("#/") {
        Some((id, pointer)) => (id, Some(pointer)),
        None => (ref_, None),
    }
}

fn clean_id(id: &str) -> &str {
    if ["#", ""].contains(&id) { "" } else { id }
}
