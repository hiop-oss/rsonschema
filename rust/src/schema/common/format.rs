//! this module has been written by AI

use chrono::{DateTime, NaiveDate, Timelike};
use idna_adapter::{Adapter, LEFT_OR_DUAL_JOINING_MASK, RIGHT_OR_DUAL_JOINING_MASK};
use iri_string::types::{IriReferenceStr, IriStr, UriReferenceStr, UriStr};
use iso8601_duration::Duration;
use jsonptr::Pointer;
use regress::Regex as EcmaRegex;
use serde::{Deserialize, Serialize};
use std::{fmt, net, str::FromStr};
use unicode_normalization::is_nfc;
use unicode_script::{Script, UnicodeScript};
use uri_template_ex::UriTemplate;
use uuid::Uuid;

/// The possible formats of values in JSON Schema
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    /// The `date-time` format
    DateTime,

    /// The `date` format
    Date,

    /// The `time` format
    Time,

    /// The `duration` format
    Duration,

    /// The `email` format
    Email,

    /// The `idn-email` format
    IdnEmail,

    /// The `hostname` format
    Hostname,

    /// The `idn-hostname` format
    IdnHostname,

    /// The `ipv4` format
    Ipv4,

    /// The `ipv6` format
    Ipv6,

    /// The `uri` format
    Uri,

    /// The `uri-reference` format
    UriReference,

    /// The `iri` format
    Iri,

    /// The `iri-reference` format
    IriReference,

    /// The `uuid` format
    Uuid,

    /// The `uri-template` format
    UriTemplate,

    /// The `json-pointer` format
    JsonPointer,

    /// The `relative-json-pointer` format
    RelativeJsonPointer,

    /// The `regex` format
    Regex,

    /// Unknown format (always valid)
    #[serde(other)]
    Unknown,
}

impl Format {
    pub(crate) fn is_valid(&self, string: &str) -> bool {
        match self {
            // RFC 3339 date-time with leap second support
            Self::DateTime => validate_datetime(string),

            // RFC 3339 date (YYYY-MM-DD)
            Self::Date => validate_date(string),

            // RFC 3339 time with timezone
            Self::Time => is_valid_time(string),

            // ISO 8601 duration
            Self::Duration => is_valid_duration(string),

            // RFC 5321 email (ASCII only)
            Self::Email => validate_email(string, false),

            // RFC 6531 internationalized email
            Self::IdnEmail => validate_email(string, true),

            // RFC 1123 hostname
            Self::Hostname => hostname_validator::is_valid(string),

            // RFC 5890 internationalized hostname
            Self::IdnHostname => is_valid_idn_hostname(string),

            // RFC 2673 IPv4
            Self::Ipv4 => net::Ipv4Addr::from_str(string).is_ok(),

            // RFC 4291 IPv6
            Self::Ipv6 => net::Ipv6Addr::from_str(string).is_ok(),

            // RFC 3986 URI
            Self::Uri => UriStr::new(string).is_ok(),

            // RFC 3986 URI reference
            Self::UriReference => UriReferenceStr::new(string).is_ok(),

            // RFC 3987 IRI
            Self::Iri => IriStr::new(string).is_ok(),

            // RFC 3987 IRI reference
            Self::IriReference => IriReferenceStr::new(string).is_ok(),

            // RFC 4122 UUID (requires dashes)
            Self::Uuid => string.contains('-') && Uuid::parse_str(string).is_ok(),

            // RFC 6570 URI template
            Self::UriTemplate => UriTemplate::new(string).is_ok(),

            // RFC 6901 JSON Pointer
            Self::JsonPointer => Pointer::parse(string).is_ok(),

            // Relative JSON Pointer
            Self::RelativeJsonPointer => validate_relative_json_pointer(string),

            // ECMAScript regex
            Self::Regex => validate_regex(string),

            // Unknown formats are always valid
            Self::Unknown => true,
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::DateTime => "date-time",
            Self::Date => "date",
            Self::Time => "time",
            Self::Duration => "duration",
            Self::Email => "email",
            Self::IdnEmail => "idn-email",
            Self::Hostname => "hostname",
            Self::IdnHostname => "idn-hostname",
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Uri => "uri",
            Self::UriReference => "uri-reference",
            Self::Iri => "iri",
            Self::IriReference => "iri-reference",
            Self::Uuid => "uuid",
            Self::UriTemplate => "uri template",
            Self::JsonPointer => "json-pointer",
            Self::RelativeJsonPointer => "relative-json-pointer",
            Self::Regex => "regex",
            Self::Unknown => "unknown",
        };
        write!(f, "{string}")
    }
}

// ============================================================================
// Date/Time Validation
// ============================================================================

fn validate_datetime(string: &str) -> bool {
    let Ok(dt) = DateTime::parse_from_rfc3339(string) else {
        return false;
    };
    // Special handling for leap seconds (60th second)
    if string.contains(":60") {
        let utc_time = dt.naive_utc().time();
        utc_time.hour() == 23 && utc_time.minute() == 59
    } else {
        true
    }
}

fn validate_date(string: &str) -> bool {
    // RFC 3339 requires exactly 10 characters: YYYY-MM-DD
    string.len() == 10 && NaiveDate::parse_from_str(string, "%Y-%m-%d").is_ok()
}

fn is_valid_time(input: &str) -> bool {
    // RFC 3339 full-time requires time-offset (timezone)
    let Ok(dt) = DateTime::parse_from_rfc3339(&format!("1970-01-01T{input}")) else {
        return false;
    };
    // Special handling for leap seconds
    if input.contains(":60") {
        let utc_time = dt.naive_utc().time();
        utc_time.hour() == 23 && utc_time.minute() == 59
    } else {
        true
    }
}

fn is_valid_duration(input: &str) -> bool {
    if !input.starts_with('P') || Duration::parse(input).is_err() {
        return false;
    }
    // Ensure 'T' is not trailing and has valid time components
    if let Some(t_index) = input.find('T') {
        if t_index == input.len() - 1 {
            return false;
        }
        let time_part = &input[t_index + 1..];
        if !time_part.chars().any(|c| matches!(c, 'H' | 'M' | 'S')) {
            return false;
        }
    }
    // At least one component is required
    input[1..]
        .chars()
        .any(|c| matches!(c, 'Y' | 'M' | 'W' | 'D' | 'H' | 'S'))
}

// ============================================================================
// Email Validation
// ============================================================================

fn validate_email(string: &str, allow_idn: bool) -> bool {
    // ASCII-only email doesn't allow non-ASCII characters
    if !allow_idn && !string.is_ascii() {
        return false;
    }

    match addr::parse_email_address(string) {
        Ok(address) => is_valid_email_host(&address, allow_idn),
        Err(_) => is_email_domain_literal(string),
    }
}

fn is_valid_email_host(address: &addr::email::Address<'_>, allow_idn: bool) -> bool {
    match address.host() {
        addr::email::Host::Domain(name) => {
            if !name.as_str().contains('.') {
                return false;
            }
            if allow_idn {
                is_valid_idn_hostname(name.as_str())
            } else {
                hostname_validator::is_valid(name.as_str())
            }
        }
        addr::email::Host::IpAddr(_) => true,
    }
}

fn is_email_domain_literal(email: &str) -> bool {
    let Some(at) = email.rfind('@') else {
        return false;
    };
    let local = &email[..at];
    let domain = &email[at + 1..];
    if local.is_empty() || !domain.starts_with('[') || !domain.ends_with(']') {
        return false;
    }
    let inner = &domain[1..domain.len() - 1];
    if let Some(rest) = inner.strip_prefix("IPv6:") {
        net::Ipv6Addr::from_str(rest).is_ok()
    } else {
        net::Ipv4Addr::from_str(inner).is_ok()
    }
}

// ============================================================================
// JSON Pointer Validation
// ============================================================================

fn validate_relative_json_pointer(string: &str) -> bool {
    if string.is_empty() {
        return false;
    }

    let digits_len = string.chars().take_while(|ch| ch.is_ascii_digit()).count();
    if digits_len == 0 {
        return false;
    }

    let prefix = &string[..digits_len];
    // No leading zeros (except "0" itself)
    if prefix.len() > 1 && prefix.starts_with('0') {
        return false;
    }

    let suffix = &string[digits_len..];
    matches!(suffix, "" | "#") || (suffix.starts_with('/') && Pointer::parse(suffix).is_ok())
}

// ============================================================================
// Regex Validation
// ============================================================================

fn validate_regex(string: &str) -> bool {
    // Check for invalid escape sequences that the regress crate might accept
    if has_invalid_escape_sequences(string) {
        return false;
    }
    EcmaRegex::new(string).is_ok()
}

fn has_invalid_escape_sequences(pattern: &str) -> bool {
    let mut chars = pattern.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\'
            && let Some(&next) = chars.peek()
        {
            match next {
                // Valid single-character escapes
                'b' | 't' | 'n' | 'v' | 'f' | 'r' | '0' => {
                    chars.next();
                }
                // Character class escapes
                'd' | 'D' | 's' | 'S' | 'w' | 'W' => {
                    chars.next();
                }
                // Word boundary
                'B' => {
                    chars.next();
                }
                // Control characters \cX
                'c' => {
                    chars.next();
                    if let Some(&control_char) = chars.peek()
                        && control_char.is_ascii_alphabetic()
                    {
                        chars.next();
                    }
                }
                // Hex escapes \xHH
                'x' => {
                    chars.next();
                    for _ in 0..2 {
                        if let Some(&hex_ch) = chars.peek()
                            && hex_ch.is_ascii_hexdigit()
                        {
                            chars.next();
                        }
                    }
                }
                // Unicode escapes \uHHHH or \u{HHHHHH}
                'u' => {
                    chars.next();
                    if let Some(&brace) = chars.peek() {
                        if brace == '{' {
                            chars.next();
                            while let Some(&hex_ch) = chars.peek() {
                                if hex_ch == '}' {
                                    chars.next();
                                    break;
                                } else if hex_ch.is_ascii_hexdigit() {
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            for _ in 0..4 {
                                if let Some(&hex_ch) = chars.peek()
                                    && hex_ch.is_ascii_hexdigit()
                                {
                                    chars.next();
                                }
                            }
                        }
                    }
                }
                // Syntax characters and identity escapes for non-word chars
                '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}'
                | '|' | '/' | '-' | ',' | ';' | ':' | '!' | '#' | '%' | '&' | '<' | '>' | '='
                | '@' | '`' | '~' | '"' | '\'' | ' ' | '\n' | '\r' | '\t' => {
                    chars.next();
                }
                // Check for invalid escape of ASCII letters (except valid ones above)
                ch if ch.is_ascii_alphabetic() => {
                    // Invalid escape sequence like \a, \e, etc.
                    return true;
                }
                _ => {
                    chars.next();
                }
            }
        }
    }
    false
}

// ============================================================================
// IDN Hostname Validation (IDNA2008)
// ============================================================================

fn map_idn_separators(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            '.' | '\u{3002}' | '\u{FF0E}' | '\u{FF61}' => '.',
            _ => ch,
        })
        .collect()
}

fn split_idn_labels(input: &str) -> Vec<&str> {
    let mut labels = Vec::new();
    let mut start = 0;
    for (idx, ch) in input.char_indices() {
        if matches!(ch, '.' | '\u{3002}' | '\u{FF0E}' | '\u{FF61}') {
            labels.push(&input[start..idx]);
            start = idx + ch.len_utf8();
        }
    }
    labels.push(&input[start..]);
    labels
}

fn prev_joining_char(chars: &[char], index: usize, adapter: &Adapter) -> Option<char> {
    let mut idx = index;
    while idx > 0 {
        idx -= 1;
        let c = chars[idx];
        if !adapter.joining_type(c).is_transparent() {
            return Some(c);
        }
    }
    None
}

fn next_joining_char(chars: &[char], index: usize, adapter: &Adapter) -> Option<char> {
    let mut idx = index + 1;
    while idx < chars.len() {
        let c = chars[idx];
        if !adapter.joining_type(c).is_transparent() {
            return Some(c);
        }
        idx += 1;
    }
    None
}

fn has_virama_before(chars: &[char], index: usize, adapter: &Adapter) -> bool {
    let mut idx = index;
    while idx > 0 {
        idx -= 1;
        let c = chars[idx];
        if adapter.is_virama(c) {
            return true;
        }
        if !adapter.joining_type(c).is_transparent() {
            return false;
        }
    }
    false
}

fn is_valid_idn_hostname(input: &str) -> bool {
    if input.is_empty() || has_disallowed_idna_chars(input) {
        return false;
    }

    // Validate ASCII conversion and basic structure
    let ascii_input = map_idn_separators(input);
    let ascii = match idna::domain_to_ascii_strict(&ascii_input) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if !hostname_validator::is_valid(&ascii) {
        return false;
    }

    // Validate each label according to IDNA2008 rules
    let adapter = Adapter::new();
    split_idn_labels(input)
        .iter()
        .all(|label| validate_single_idna_label(label, &adapter))
}

fn has_disallowed_idna_chars(input: &str) -> bool {
    input.chars().any(|ch| {
        matches!(
            ch,
            '\u{0640}'            // ARABIC TATWEEL
            | '\u{07FA}'          // NKO LAJANYALAN
            | '\u{3031}'..='\u{3035}' // Various Japanese marks
            | '\u{302E}'..='\u{302F}' // Hangul tone marks
            | '\u{303B}'          // Vertical ideographic iteration mark
        )
    })
}

fn validate_single_idna_label(label: &str, adapter: &Adapter) -> bool {
    if label.is_empty() || !is_nfc(label) {
        return false;
    }

    let chars: Vec<char> = label.chars().collect();
    if chars.is_empty() {
        return false;
    }

    // Cannot start with combining mark
    if adapter.is_mark(chars[0]) {
        return false;
    }

    // Cannot start or end with hyphen
    if matches!(chars.first(), Some('-')) || matches!(chars.last(), Some('-')) {
        return false;
    }

    validate_label_characters(&chars, adapter)
}

fn validate_label_characters(chars: &[char], adapter: &Adapter) -> bool {
    let mut has_kana_middle_dot = false;
    let mut has_required_script_for_kana_dot = false;
    let mut has_arabic_indic_digits = false;
    let mut has_ext_arabic_indic_digits = false;

    for (i, &ch) in chars.iter().enumerate() {
        // Validate context-dependent characters
        if !validate_contextual_character(ch, i, chars, adapter) {
            return false;
        }

        // Track script requirements
        if matches!(
            UnicodeScript::script(&ch),
            Script::Hiragana | Script::Katakana | Script::Han
        ) {
            has_required_script_for_kana_dot = true;
        }

        // Track digit types
        match ch {
            '\u{0660}'..='\u{0669}' => has_arabic_indic_digits = true,
            '\u{06F0}'..='\u{06F9}' => has_ext_arabic_indic_digits = true,
            '\u{30FB}' | '\u{FF65}' => has_kana_middle_dot = true,
            _ => {}
        }
    }

    // Validate script and digit mixing rules
    if has_kana_middle_dot && !has_required_script_for_kana_dot {
        return false;
    }
    if has_arabic_indic_digits && has_ext_arabic_indic_digits {
        return false;
    }

    true
}

fn validate_contextual_character(ch: char, i: usize, chars: &[char], adapter: &Adapter) -> bool {
    match ch {
        // MIDDLE DOT: must be between 'l' or 'L'
        '\u{00B7}' => {
            i > 0
                && i + 1 < chars.len()
                && matches!(chars[i - 1], 'l' | 'L')
                && matches!(chars[i + 1], 'l' | 'L')
        }
        // GREEK LOWER NUMERAL SIGN: must be followed by Greek
        '\u{0375}' => i + 1 < chars.len() && UnicodeScript::script(&chars[i + 1]) == Script::Greek,
        // HEBREW PUNCTUATION: must be preceded by Hebrew
        '\u{05F3}' | '\u{05F4}' => i > 0 && UnicodeScript::script(&chars[i - 1]) == Script::Hebrew,
        // DISALLOWED characters
        '\u{302E}' | '\u{302F}' | '\u{3031}'..='\u{3035}' | '\u{303B}' => false,
        // ZERO WIDTH NON-JOINER
        '\u{200C}' => {
            has_virama_before(chars, i, adapter) || validate_zwnj_context(chars, i, adapter)
        }
        // ZERO WIDTH JOINER
        '\u{200D}' => has_virama_before(chars, i, adapter),
        _ => true,
    }
}

fn validate_zwnj_context(chars: &[char], i: usize, adapter: &Adapter) -> bool {
    let Some(prev_char) = prev_joining_char(chars, i, adapter) else {
        return false;
    };
    let Some(next_char) = next_joining_char(chars, i, adapter) else {
        return false;
    };
    let prev_mask = adapter.joining_type(prev_char).to_mask();
    let next_mask = adapter.joining_type(next_char).to_mask();
    prev_mask.intersects(LEFT_OR_DUAL_JOINING_MASK)
        && next_mask.intersects(RIGHT_OR_DUAL_JOINING_MASK)
}
