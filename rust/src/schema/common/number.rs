use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{cmp, fmt, ops, str::FromStr};

/// A JSON number
#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(deny_unknown_fields, transparent)]
pub struct Number(pub serde_json::Number);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self(value.into())
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value.0)
    }
}

impl From<serde_json::Number> for Number {
    fn from(value: serde_json::Number) -> Self {
        Self(value)
    }
}

impl ops::Rem for &Number {
    type Output = BigDecimal;

    fn rem(self, rhs: Self) -> Self::Output {
        let left = Self::Output::from_str(&rhs.0.to_string()).unwrap();
        let right = Self::Output::from_str(&self.0.to_string()).unwrap();
        left % right
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_f64() == other.0.as_f64()
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.as_f64().partial_cmp(&other.0.as_f64())
    }
}
