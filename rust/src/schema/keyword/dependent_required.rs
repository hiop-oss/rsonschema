use crate::{error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_dependent_required(
        &self,
        instance: &Value,
    ) -> Option<error::ValidationErrors> {
        self.dependent_required
            .as_ref()
            .and_then(|dependent_required| {
                _validate_dependent_required(dependent_required, instance)
            })
    }
}

pub(crate) fn _validate_dependent_required(
    dependent_required: &schema::common::dependencies::DependentRequired,
    instance: &Value,
) -> Option<error::ValidationErrors> {
    let mut errors = Vec::new();
    for (property_key, dependent_properties) in dependent_required {
        if instance.get(property_key).is_some() {
            let mut property_names = dependent_properties
                .iter()
                .filter(|dependent_property| instance.get(dependent_property.as_str()).is_none())
                .peekable();
            if property_names.peek().is_some() {
                let error = error::ValidationError {
                    instance: instance.clone(),
                    type_: error::type_::ValidationErrorType::DependentRequired {
                        dependent_property: property_key.clone(),
                        property_names: property_names.cloned().collect(),
                    },
                    ..Default::default()
                };
                errors.push(error)
            }
        }
    }
    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::json;

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/validation/dependentrequired/)
    #[rstest]
    #[case::valid_simple(
        json!({
            "name": "John",
            "age": 25,
            "license": "XYZ123"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "license": {"type": "string"}
            },
            "dependentRequired": {
                "license": [
                    "age"
                ]
            }
        }),
        None
    )]
    #[case::invalid_simple(
        json!({
            "name": "John",
            "license": "XYZ123"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "license": {"type": "string"}
            },
            "dependentRequired": {
                "license": [
                    "age"
                ]
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "name": "John",
                "license": "XYZ123"
            }),
            type_: error::type_::ValidationErrorType::DependentRequired {
                dependent_property: "license".to_string(),
                property_names: Vec::from(["age".to_string()])
            },
            ..Default::default()
        }])
    )]
    #[case::valid_complex(
        json!({
            "productName": "Iphone",
            "productPriceUSD": 399.99,
            "units": 5,
            "totalCost": 1599.99,
            "trackingId" : 1414326241,
            "outForDelivery": "yes"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "productName": {"type": "string"},
                "productPriceUSD": {"type": "number"},
                "units": {"type": "number"}
            },
            "dependentRequired": {
                "productPriceUSD": [
                    "productName"
                ],
                "totalCost": [
                    "productPriceUSD",
                    "units"
                ],
                "trackingId": [
                    "outForDelivery"
                ]
            }
        }),
        None
    )]
    #[case::invalid_complex(
        json!({
            "productName": "Iphone",
            "units": 5,
            "totalCost": 1599.99,
            "trackingId" : 1414326241,
            "outForDelivery": "yes"
        }),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "productName": {"type": "string"},
                "productPriceUSD": {"type": "number"},
                "units": {"type": "number"}
            },
            "dependentRequired": {
                "productPriceUSD": [
                    "productName"
                ],
                "totalCost": [
                    "productPriceUSD",
                    "units"
                ],
                "trackingId": [
                    "outForDelivery"
                ]
            }
        }),
        Some(vec![error::ValidationError {
            instance: json!({
                "productName": "Iphone",
                "units": 5,
                "totalCost": 1599.99,
                "trackingId": 1414326241,
                "outForDelivery": "yes"
            }),
            type_: error::type_::ValidationErrorType::DependentRequired {
                dependent_property: "totalCost".to_string(),
                property_names: Vec::from(["productPriceUSD".to_string()])
            },
            ..Default::default()
        }])
    )]
    fn test_dependent_required_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
