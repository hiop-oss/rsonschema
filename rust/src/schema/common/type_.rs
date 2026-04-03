use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// A wrapper for a single item or multiple items of the same type
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, untagged)]
pub enum SingleOrMultiple<T> {
    /// Variant for a single item
    Single(T),

    /// Variant for multiple items
    Multiple(Vec<T>),
}

impl<T> SingleOrMultiple<T> {
    /// Get the items. If there is only one item, it will be returned as a single-element vector
    pub(crate) fn get_items(&self) -> Vec<&T> {
        match self {
            Self::Single(item) => vec![item],
            Self::Multiple(items) => items.iter().collect(),
        }
    }
}

impl<T> From<T> for SingleOrMultiple<T> {
    fn from(single: T) -> Self {
        SingleOrMultiple::Single(single)
    }
}

impl<T> From<Vec<T>> for SingleOrMultiple<T> {
    fn from(multiple: Vec<T>) -> Self {
        SingleOrMultiple::Multiple(multiple)
    }
}

/// The available JSON Schema types
///
/// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/type/)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum Type {
    /// The `array` type
    Array,

    /// The `boolean` type
    Boolean,

    /// The `integer` type
    Integer,

    /// The `null` type
    Null,

    /// The `number` type
    Number,

    /// The `object` type
    Object,

    /// The `string` type
    String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label: &str = match self {
            Self::Array => "array",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Null => "null",
            Self::Number => "number",
            Self::Object => "object",
            Self::String => "string",
        };
        write!(f, "`{label}`")
    }
}

impl SingleOrMultiple<Type> {
    /// Check if the instance type is valid
    pub(crate) fn has_type_of(&self, instance: &Value) -> bool {
        let types = self.get_items();
        match &instance {
            Value::Array(_) => types.contains(&&Type::Array),
            Value::Bool(_) => types.contains(&&Type::Boolean),
            Value::Null => types.contains(&&Type::Null),
            Value::Number(number) => {
                if types.contains(&&Type::Number) {
                    true
                } else if number.is_i64() {
                    types.contains(&&Type::Integer)
                } else if let Some(float) = number.as_f64() {
                    if float.fract() == 0.0 {
                        types.contains(&&Type::Integer)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Value::Object(_) => types.contains(&&Type::Object),
            Value::String(_) => types.contains(&&Type::String),
        }
    }
}
