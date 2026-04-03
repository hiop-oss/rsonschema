use serde::{Deserialize, Serialize};
use std::fmt;

/// The validation error pointer
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ValidationErrorPointer {
    /// A key of a JSON object
    Key(String),

    /// An index of a JSON array
    Index(usize),
}

impl From<String> for ValidationErrorPointer {
    fn from(key: String) -> Self {
        Self::Key(key)
    }
}

impl From<usize> for ValidationErrorPointer {
    fn from(index: usize) -> Self {
        Self::Index(index)
    }
}

impl fmt::Display for ValidationErrorPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(key) => write!(f, "{key}"),
            Self::Index(index) => write!(f, "{index}"),
        }
    }
}

impl ValidationErrorPointer {
    pub(crate) fn prepend<P: Into<Self>>(pointer: P, mut pointers: Vec<Self>) -> Vec<Self> {
        pointers.insert(0, pointer.into());
        pointers
    }
}
