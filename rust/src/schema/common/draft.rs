use serde::{Deserialize, Serialize};
use url::Url;

/// The JSON Schema draft version
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum Draft {
    /// The draft-04 version
    #[serde(rename = "https://json-schema.org/draft-04/schema")]
    _04,

    /// The draft-06 version
    #[serde(rename = "https://json-schema.org/draft-06/schema")]
    _06,

    /// The draft-07 version
    #[serde(rename = "https://json-schema.org/draft-07/schema")]
    _07,

    /// The draft-2019-09 version
    #[serde(rename = "https://json-schema.org/draft/2019-09/schema")]
    _2019_09,

    /// The draft-2020-12 version
    #[serde(rename = "https://json-schema.org/draft/2020-12/schema")]
    _2020_12,

    /// Custom draft version
    #[serde(untagged)]
    Custom(Url),
}
