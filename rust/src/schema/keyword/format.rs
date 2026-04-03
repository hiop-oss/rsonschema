use crate::{error, schema};

use serde_json::Value;

impl schema::object::ObjectSchema {
    pub(crate) fn validate_format(&self, string: &str) -> Option<error::ValidationError> {
        self.format
            .as_ref()
            .filter(|format| !format.is_valid(string))
            .map(|format| error::ValidationError {
                instance: Value::String(string.to_string()),
                type_: error::type_::ValidationErrorType::Format { format: *format },
                ..Default::default()
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests;

    use rstest::rstest;
    use serde_json::{Value, json};

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_date_time(
        json!("2020-12-31T23:59:59Z"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "date-time"
        }),
        None
    )]
    #[case::invalid_date_time(
        json!("2020-12-31 23:59:59"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "date-time"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::DateTime
            },
            ..Default::default()
        }])
    )]
    fn test_date_time_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_date(
        json!("2020-12-31"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "date"
        }),
        None
    )]
    #[case::invalid_date(
        json!("2020-02-30"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "date"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Date
            },
            ..Default::default()
        }])
    )]
    fn test_date_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_time(
        json!("23:59:59Z"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "time"
        }),
        None
    )]
    #[case::invalid_time(
        json!("24:00:00"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "time"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Time
            },
            ..Default::default()
        }])
    )]
    fn test_time_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_duration(
        json!("P1DT1H"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "duration"
        }),
        None
    )]
    #[case::invalid_duration(
        json!("1 day"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "duration"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Duration
            },
            ..Default::default()
        }])
    )]
    fn test_duration_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_email(
        json!("john.doe@example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "email"
        }),
        None
    )]
    #[case::invalid_email(
        json!("foo_bar"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "email"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Email
            },
            ..Default::default()
        }])
    )]
    fn test_email_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_idn_email(
        json!("bücher@example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "idn-email"
        }),
        None
    )]
    #[case::invalid_idn_email(
        json!("bücher@com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "idn-email"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::IdnEmail
            },
            ..Default::default()
        }])
    )]
    fn test_idn_email_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_hostname(
        json!("example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "hostname"
        }),
        None
    )]
    #[case::invalid_hostname(
        json!("-example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "hostname"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Hostname
            },
            ..Default::default()
        }])
    )]
    fn test_hostname_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_idn_hostname(
        json!("xn--bcher-kva.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "idn-hostname"
        }),
        None
    )]
    #[case::invalid_idn_hostname(
        json!("-xn--bcher-kva.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "idn-hostname"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::IdnHostname
            },
            ..Default::default()
        }])
    )]
    fn test_idn_hostname_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_ipv4(
        json!("192.168.0.1"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "ipv4"
        }),
        None
    )]
    #[case::invalid_ipv4(
        json!("256.256.256.256"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "ipv4"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Ipv4
            },
            ..Default::default()
        }])
    )]
    fn test_ipv4_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_ipv6(
        json!("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "ipv6"
        }),
        None
    )]
    #[case::invalid_ipv6(
        json!("2001:db8::85a3::8a2e:370:7334"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "ipv6"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Ipv6
            },
            ..Default::default()
        }])
    )]
    fn test_ipv6_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_uri(
        json!("https://example.com/path?query#fragment"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri"
        }),
        None
    )]
    #[case::invalid_uri(
        json!("example.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Uri
            },
            ..Default::default()
        }])
    )]
    fn test_uri_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_uri_reference(
        json!("https://example.com/path?query#fragment"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri-reference"
        }),
        None
    )]
    #[case::invalid_uri_reference(
        json!("://missing-scheme.com"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri-reference"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::UriReference
            },
            ..Default::default()
        }])
    )]
    fn test_uri_reference_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_uuid(
        json!("123e4567-e89b-12d3-a456-426614174000"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uuid"
        }),
        None
    )]
    #[case::invalid_uuid(
        json!("123e4567-e89b-12d3-a456-42661417400z"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uuid"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Uuid
            },
            ..Default::default()
        }])
    )]
    fn test_uuid_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_uri_template(
        json!("/path/{var}"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri-template"
        }),
        None
    )]
    #[case::invalid_uri_template(
        json!("/path/{var"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "uri-template"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::UriTemplate
            },
            ..Default::default()
        }])
    )]
    fn test_uri_template_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_json_pointer(
        json!("/foo/bar"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "json-pointer"
        }),
        None
    )]
    #[case::invalid_json_pointer(
        json!("foo/bar"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "json-pointer"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::JsonPointer
            },
            ..Default::default()
        }])
    )]
    fn test_json_pointer_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_relative_json_pointer(
        json!("0/foo/bar"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "relative-json-pointer"
        }),
        None
    )]
    #[case::invalid_relative_json_pointer(
        json!("foo/bar"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "relative-json-pointer"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::RelativeJsonPointer
            },
            ..Default::default()
        }])
    )]
    fn test_relative_json_pointer_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }

    /// Tests from [Learn JSONSchema](https://www.learnjsonschema.com/2020-12/format-annotation/format/)
    #[rstest]
    #[case::valid_regex(
        json!(r"^\d{3}-\d{2}-\d{4}$"),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "regex"
        }),
        None
    )]
    #[case::invalid_regex(
        json!(r"^\d{3}-\d{2}-\d{4}$("),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "regex"
        }),
        Some(vec![error::ValidationError {
            instance: instance.clone(),
            type_: error::type_::ValidationErrorType::Format {
                format: schema::common::format::Format::Regex
            },
            ..Default::default()
        }])
    )]
    fn test_regex_format_validate(
        #[case] instance: Value,
        #[case] schema: Value,
        #[case] expected: Option<error::ValidationErrors>,
    ) {
        tests::assert_validate(instance, schema, expected)
    }
}
