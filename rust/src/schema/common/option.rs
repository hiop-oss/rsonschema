use serde::{Deserialize, Deserializer};
use serde_json::Value;

// Deserializing "null" to `Option<Value>` directly results in `None`,
// this function deserializes it to `Some(Value::Null)`
pub(crate) fn allow_null<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    Value::deserialize(deserializer).map(Some)
}
