use crate::error;

use either::Either;
use serde_json::Value;
use std::fmt;
use url::Url;

const ID_KEY: &str = "$id";

/// The id of a schema
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum Id {
    /// An absolute id is a valid url
    Absolute(Url),

    /// A relative id can be any string
    Relative(String),
}

impl Id {
    pub(crate) fn new(base: Option<&Self>, id: &str) -> Self {
        match Url::parse(id) {
            Ok(url) => Self::Absolute(url),
            Err(_) => match base {
                Some(base) => match base {
                    Self::Absolute(url) => {
                        let absolute = url.join(id).unwrap();
                        Self::Absolute(absolute)
                    }
                    Self::Relative(string) => {
                        let relative = join_with_base(string, id);
                        Self::Relative(relative)
                    }
                },
                None => Self::Relative(id.to_string()),
            },
        }
    }

    pub(crate) fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute(_))
    }

    pub(crate) fn is_relative(&self) -> bool {
        matches!(self, Self::Relative(_))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absolute(id) => write!(f, "{id}"),
            Self::Relative(id) => write!(f, "{id}"),
        }
    }
}

fn join_with_base(base: &str, ref_: &str) -> String {
    let parts: Vec<&str> = base.split('/').collect();
    let base = &parts[..parts.len() - 1].join("/");
    format!("{base}/{ref_}")
}

pub(crate) fn extract_from_schema(
    schema: &Value,
    parent_id: Option<Id>,
) -> Either<Option<Id>, error::ValidationError> {
    match schema.get(ID_KEY) {
        Some(Value::String(id)) => {
            let id = Id::new(parent_id.as_ref(), id);
            Either::Left(Some(id))
        }
        Some(id) => {
            let err = error::ValidationError {
                instance: id.clone(),
                pointer: vec![error::pointer::ValidationErrorPointer::Key(
                    ID_KEY.to_string(),
                )],
                type_: error::type_::ValidationErrorType::UnparsableId,
            };
            Either::Right(err)
        }
        None => Either::Left(parent_id),
    }
}
