use crate::{error, schema};

use serde::{Deserialize, Serialize, de};
use serde_json::Value;
use std::{cmp, fmt};

fn get_similarity_for_string(instance: &Value, value: &str) -> f64 {
    strsim::jaro(&instance.to_string(), value)
}

fn get_similarity_for_required(instance: &Value, property_names: &[String]) -> f64 {
    match instance {
        Value::Object(object) => {
            // Sum over each required: best similarity from instance keys to that required name.
            property_names
                .iter()
                .map(|required| {
                    object
                        .keys()
                        .map(|key| strsim::jaro(key, required))
                        .fold(Default::default(), f64::max)
                })
                .sum()
        }
        Value::String(string) => property_names
            .iter()
            .map(|required| strsim::jaro(string, required))
            .fold(Default::default(), f64::max),
        _ => Default::default(),
    }
}

/// The validation error types
#[derive(Clone, Debug, Default, Deserialize, Eq, thiserror::Error, PartialEq, Serialize)]
pub enum ValidationErrorType {
    /// Error raised when an id is unparsable
    UnparsableId,

    /// Error raised when a schema is unparsable
    UnparsableSchema {
        /// The schema involved
        schema: Value,

        /// The parsing error
        cause: String,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/additionalproperties/)
    AdditionalProperties {
        /// The additional property validation error
        additional_property: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/allof/)
    AllOf {
        /// The validation error occured in the subschema
        schema: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/anyof/)
    AnyOf {
        /// The validation error occured in the subschema
        schema: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/contains/)
    Contains,

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/const/)
    Const {
        /// The mismatched constant
        const_: Value,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/dependentschemas/)
    DependentSchema {
        /// The name of the dependent property
        dependent_property: String,

        /// The dependent schema validation error
        schema: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/dependentrequired/)
    DependentRequired {
        /// The name of the dependent property
        dependent_property: String,

        /// The names of the required properties
        property_names: Vec<String>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/enum/)
    Enum {
        /// The mismatched enum instances
        enum_: Vec<Value>,
    },

    /// See [Learn JSONSchema (if)](https://www.learnjsonschema.com/2020-12/applicator/if/)
    /// and [Learn JSONSchema (else)](https://www.learnjsonschema.com/2020-12/applicator/else/)
    Else {
        /// The validation error occured in the `if` statement
        if_: error::ValidationErrors,

        /// The validation error occured in the `else` branch
        else_: error::ValidationErrors,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusivemaximum/)
    ExclusiveMaximum {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/exclusiveminimum/)
    ExclusiveMinimum {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// Error raised when a schema is `false`
    #[default]
    FalseSchema,

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/format-assertion/format/)
    Format {
        /// The mismatched format
        format: schema::common::format::Format,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/items/)
    Items {
        /// The item validation error
        item: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maximum/)
    Maximum {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxcontains/)
    MaxContains {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxitems/)
    MaxItems {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxlength/)
    MaxLength {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/maxproperties/)
    MaxProperties {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minimum/)
    Minimum {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/mincontains/)
    MinContains {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minitems/)
    MinItems {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minlength/)
    MinLength {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/minproperties/)
    MinProperties {
        /// The unmet limit
        limit: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/multipleof/)
    MultipleOf {
        /// The ummet multiplier
        multiple_of: schema::common::number::Number,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/not/)
    Not,

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/oneof/)
    OneOf {
        /// The validation error occured in the subschema
        schema: Option<Box<Self>>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/patternproperties/)
    PatternProperties {
        /// The pattern property validation error
        pattern_property: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/pattern/)
    Pattern {
        /// The mismatched pattern
        pattern: schema::common::regex::Regex,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/prefixitems/)
    PrefixItems {
        /// The prefix item validation error
        prefix_item: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/properties/)
    Properties {
        /// The property validation error
        property: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/applicator/propertynames/)
    PropertyName {
        /// The property name validation error
        property_name: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/core/ref/)
    Ref {
        /// The broken reference
        ref_: String,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/required/)
    Required {
        /// The names of the required properties
        property_names: Vec<String>,
    },

    /// See [Learn JSONSchema (if)](https://www.learnjsonschema.com/2020-12/applicator/if/)
    /// and [Learn JSONSchema (then)](https://www.learnjsonschema.com/2020-12/applicator/then/)
    Then {
        /// The validation error occured in the `then` branch
        then: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/type/)
    Type {
        /// The expected type
        expected: schema::common::type_::SingleOrMultiple<schema::common::type_::Type>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluateditems/)
    UnevaluatedItems {
        /// The unevaluated item validation error
        unevaluated_item: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluatedproperties/)
    UnevaluatedProperties {
        /// The unevaluated property validation error
        unevaluated_property: Box<Self>,
    },

    /// See [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/uniqueitems/)
    UniqueItems,
}

impl fmt::Display for ValidationErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match &self {
            Self::UnparsableId => "as id of the schema is not valid".to_string(),
            Self::UnparsableSchema { cause, .. } => format!("is unparsable because: {cause}"),
            Self::AdditionalProperties {
                additional_property: inner,
            }
            | Self::AllOf { schema: inner }
            | Self::AnyOf { schema: inner }
            | Self::Items { item: inner }
            | Self::PatternProperties {
                pattern_property: inner,
            }
            | Self::PrefixItems { prefix_item: inner }
            | Self::Properties { property: inner }
            | Self::PropertyName {
                property_name: inner,
            }
            | Self::Then { then: inner }
            | Self::UnevaluatedItems {
                unevaluated_item: inner,
            }
            | Self::UnevaluatedProperties {
                unevaluated_property: inner,
            } => inner.to_string(),
            Self::Contains => "does not contain the required schema".to_string(),
            Self::Const { const_ } => format!("does not match the constant `{const_}`"),
            Self::DependentSchema {
                dependent_property,
                schema,
            } => format!("contains `{dependent_property}`, but {schema}"),
            Self::DependentRequired {
                dependent_property,
                property_names,
            } => {
                let property_names = property_names
                    .iter()
                    .map(|property_name| format!("`{property_name}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("contains `{dependent_property}`, then missing required: {property_names}")
            }
            Self::Enum { enum_ } => {
                let instances = enum_
                    .iter()
                    .map(|instance| instance.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("does not match any of the following:\n{instances}")
            }
            Self::Else { else_, .. } => else_
                .iter()
                .map(|err| err.type_.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            Self::ExclusiveMaximum { limit } => format!("is greater or equal than `{limit}`"),
            Self::ExclusiveMinimum { limit } => format!("is less or equal than `{limit}`"),
            Self::FalseSchema => "is not allowed".to_string(),
            Self::Format { format } => format!("does not match the `{format}` format"),
            Self::Maximum { limit } => format!("is greater than `{limit}`"),
            Self::MaxContains { limit } => {
                format!("has more than `{limit}` items that already match the schema")
            }
            Self::MaxItems { limit } => format!("has more than `{limit}` items"),
            Self::MaxLength { limit } => format!("is longer than `{limit}` characters"),
            Self::MaxProperties { limit } => format!("has more than `{limit}` properties"),
            Self::Minimum { limit } => format!("is less than `{limit}`"),
            Self::MinContains { limit } => {
                format!("has less than `{limit}` items that match the schema")
            }
            Self::MinItems { limit } => format!("must have more than `{limit}` items"),
            Self::MinLength { limit } => format!("must be longer than `{limit}` characters"),
            Self::MinProperties { limit } => format!("must have at least `{limit}` properties"),
            Self::MultipleOf { multiple_of } => format!("is not a multiple of `{multiple_of}`"),
            Self::Not => "is forbidden by the schema".to_string(),
            Self::OneOf { schema } => match schema {
                Some(inner) => inner.to_string(),
                None => "is valid for 2+ schemas".to_string(),
            },
            Self::Pattern { pattern } => format!("does not match the `{pattern}` pattern"),
            Self::Ref { ref_ } => format!("reference `{ref_}` is broken"),
            Self::Required { property_names } => {
                let required_properties = property_names
                    .iter()
                    .map(|property_name| format!("`{property_name}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("missing required: {required_properties}")
            }
            Self::Type { expected } => {
                let types = expected
                    .get_items()
                    .iter()
                    .map(|type_| type_.to_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                format!("is not a {types}")
            }
            Self::UniqueItems => "has duplicate items".to_string(),
        };
        write!(f, "{string}")
    }
}

impl PartialOrd for ValidationErrorType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValidationErrorType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

impl ValidationErrorType {
    /// The more descriptive is the error, the higher is the rank
    fn get_rank(&self) -> u8 {
        match self {
            Self::FalseSchema => 0,
            Self::UnparsableId
            | Self::UnparsableSchema { .. }
            | Self::Else { .. }
            | Self::Ref { .. } => 255,
            Self::AdditionalProperties {
                additional_property: inner,
            }
            | Self::AllOf { schema: inner }
            | Self::AnyOf { schema: inner }
            | Self::DependentSchema { schema: inner, .. }
            | Self::Items { item: inner }
            | Self::OneOf {
                schema: Some(inner),
            }
            | Self::PatternProperties {
                pattern_property: inner,
            }
            | Self::PrefixItems { prefix_item: inner }
            | Self::Properties { property: inner }
            | Self::PropertyName {
                property_name: inner,
            }
            | Self::Then { then: inner }
            | Self::UnevaluatedItems {
                unevaluated_item: inner,
            }
            | Self::UnevaluatedProperties {
                unevaluated_property: inner,
            } => inner.get_rank(),
            _ => 127,
        }
    }

    pub(crate) fn get_similarity(&self, instance: &Value) -> f64 {
        match self {
            Self::AdditionalProperties {
                additional_property: inner,
            }
            | Self::AllOf { schema: inner }
            | Self::AnyOf { schema: inner }
            | Self::DependentSchema { schema: inner, .. }
            | Self::Items { item: inner }
            | Self::OneOf {
                schema: Some(inner),
            }
            | Self::PatternProperties {
                pattern_property: inner,
            }
            | Self::PrefixItems { prefix_item: inner }
            | Self::Properties { property: inner }
            | Self::PropertyName {
                property_name: inner,
            }
            | Self::Then { then: inner }
            | Self::UnevaluatedItems {
                unevaluated_item: inner,
            }
            | Self::UnevaluatedProperties {
                unevaluated_property: inner,
            } => inner.get_similarity(instance),
            Self::Const { const_ } => get_similarity_for_string(instance, &const_.to_string()),
            Self::DependentRequired { property_names, .. } | Self::Required { property_names } => {
                get_similarity_for_required(instance, property_names)
            }
            Self::Enum { enum_ } => enum_
                .iter()
                .map(|value| get_similarity_for_string(instance, &value.to_string()))
                .fold(f64::NAN, f64::max),
            Self::Format { format } => get_similarity_for_string(instance, &format.to_string()),
            Self::Pattern { pattern } => get_similarity_for_string(instance, &pattern.to_string()),
            _ => Default::default(),
        }
    }

    pub(crate) fn from_de_error<E: de::Error>(err: E, schema: Value) -> Self {
        Self::UnparsableSchema {
            schema,
            cause: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        vec![ValidationErrorType::FalseSchema, ValidationErrorType::Contains],
        ValidationErrorType::Contains
    )]
    fn test_cmp(#[case] errors: Vec<ValidationErrorType>, #[case] expected: ValidationErrorType) {
        let actual = errors.into_iter().max();
        assert_eq!(actual, Some(expected));
    }
}
