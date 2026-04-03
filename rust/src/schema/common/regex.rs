//! this module has been written by AI

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{error, fmt, hash};

/// A regular expression (ECMAScript-compatible)
/// Uses regress for standard ECMAScript regex, fancy-regex for Unicode property escapes
#[derive(Clone, Debug)]
pub struct Regex {
    inner: RegexImpl,
    pattern: String,
}

#[derive(Clone, Debug)]
enum RegexImpl {
    Regress(regress::Regex),
    Fancy(fancy_regex::Regex),
}

impl Regex {
    /// Check if the regex matches the string
    pub(crate) fn is_match<S: AsRef<str>>(&self, string: S) -> bool {
        match &self.inner {
            RegexImpl::Regress(r) => r.find(string.as_ref()).is_some(),
            RegexImpl::Fancy(r) => r.is_match(string.as_ref()).unwrap_or(false),
        }
    }

    /// Check if pattern contains Unicode property escapes
    fn has_unicode_property_escapes(pattern: &str) -> bool {
        pattern.contains("\\p{") || pattern.contains("\\P{")
    }
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pattern)
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pattern = String::deserialize(deserializer)?;

        // Use fancy-regex for patterns with Unicode property escapes
        let inner = if Self::has_unicode_property_escapes(&pattern) {
            let regex = fancy_regex::Regex::new(&pattern).map_err(de::Error::custom)?;
            RegexImpl::Fancy(regex)
        } else {
            // Use regress for standard ECMAScript patterns (better compatibility)
            let regex = regress::Regex::new(&pattern).map_err(de::Error::custom)?;
            RegexImpl::Regress(regex)
        };

        Ok(Self { inner, pattern })
    }
}

impl Serialize for Regex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.pattern.serialize(serializer)
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Eq for Regex {}

impl hash::Hash for Regex {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
    }
}

impl TryFrom<&str> for Regex {
    type Error = Box<dyn error::Error>;

    fn try_from(pattern: &str) -> Result<Self, Self::Error> {
        let inner = if Self::has_unicode_property_escapes(pattern) {
            let regex = fancy_regex::Regex::new(pattern)?;
            RegexImpl::Fancy(regex)
        } else {
            let regex = regress::Regex::new(pattern)?;
            RegexImpl::Regress(regex)
        };
        let pattern = Self {
            inner,
            pattern: pattern.to_string(),
        };
        Ok(pattern)
    }
}
