use serde::{Deserialize, Serialize};
use std::{collections, hash};

pub(crate) enum EvaluatedKey {
    Property(String),
    Item(usize),
}

/// Evaluated items to track validation
pub type EvaluatedItems = EvaluatedMap<usize>;

/// Evaluated properties to track validation
pub type EvaluatedProperties = EvaluatedMap<String>;

/// Evaluated values to track validation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Evaluated {
    /// The properties of an object that have been evaluated
    pub properties: EvaluatedProperties,

    /// The items of an array that have been evaluated
    pub items: EvaluatedItems,
}

impl Evaluated {
    pub(crate) fn extend(&mut self, other: Evaluated, evaluated_key: Option<EvaluatedKey>) {
        match evaluated_key {
            Some(key) => match key {
                EvaluatedKey::Property(key) => {
                    self.properties.insert(key, other.properties);
                }
                EvaluatedKey::Item(key) => {
                    self.items.insert(key, other.items);
                }
            },
            None => {
                self.properties.extend(other.properties);
                self.items.extend(other.items);
            }
        }
    }
}

/// Evaluated type to track validation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum EvaluatedMap<K: Eq + hash::Hash> {
    /// Just a placeholder to end the recursion
    #[default]
    Placeholder,

    /// A recursive map
    Map(collections::HashMap<K, EvaluatedMap<K>>),
}

impl<K: Eq + hash::Hash> EvaluatedMap<K> {
    pub(crate) fn insert(&mut self, key: K, value: Self) {
        match self {
            Self::Placeholder => {
                let map = collections::HashMap::from([(key, value)]);
                *self = Self::Map(map);
            }
            Self::Map(map) => {
                map.insert(key, value);
            }
        }
    }

    pub(crate) fn extend(&mut self, other: Self) {
        match self {
            Self::Placeholder => {
                *self = other;
            }
            Self::Map(map) => {
                if let Self::Map(other_map) = other {
                    map.extend(other_map);
                }
            }
        }
    }

    pub(crate) fn contains_key(&self, key: &K) -> bool {
        match self {
            Self::Placeholder => false,
            Self::Map(map) => map.contains_key(key),
        }
    }
}

impl<K: Eq + hash::Hash> From<collections::HashMap<K, EvaluatedMap<K>>> for EvaluatedMap<K> {
    fn from(map: collections::HashMap<K, EvaluatedMap<K>>) -> Self {
        EvaluatedMap::Map(map)
    }
}
