use serde_json::Value;
use std::fmt::Write;

/// Compacts a JSON string into a more readable but compact form.
/// If the compacted output exceeds 80 characters, returns the original.
pub fn compact_json(input: &str) -> String {
    let value: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return input.to_string(),
    };

    let compacted = format_value(&value, 0);
    if compacted.len() > 80 {
        input.to_string()
    } else {
        compacted
    }
}

fn format_value(value: &Value, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let inner_indent = "  ".repeat(indent + 1);

    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else if arr.len() == 1 {
                format!("[{}]", format_value(&arr[0], indent))
            } else {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}{}", inner_indent, format_value(v, indent + 1)))
                    .collect();
                format!("[\n{}\n{}]", items.join(",\n"), indent_str)
            }
        }
        Value::Object(map) => {
            if map.is_empty() {
                "{}".to_string()
            } else {
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}\"{}\": {}",
                            inner_indent,
                            k.replace('\\', "\\\\").replace('"', "\\\""),
                            format_value(v, indent + 1)
                        )
                    })
                    .collect();
                format!("{{\n{}\n{}}}", items.join(",\n"), indent_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_simple_object() {
        let input = r#"{"name": "Alice", "age": 30}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"name\": \"Alice\",\n  \"age\": 30\n}");
    }

    #[test]
    fn test_compact_empty_object() {
        let input = r#"{}"#;
        let result = compact_json(input);
        assert_eq!(result, "{}");
    }

    #[test]
    fn test_compact_empty_array() {
        let input = r#"[]"#;
        let result = compact_json(input);
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_compact_single_element_array() {
        let input = r#"[42]"#;
        let result = compact_json(input);
        assert_eq!(result, "[42]");
    }

    #[test]
    fn test_compact_array_with_multiple_elements() {
        let input = r#"[1, 2, 3]"#;
        let result = compact_json(input);
        assert_eq!(result, "[\n  1,\n  2,\n  3\n]");
    }

    #[test]
    fn test_compact_nested_structure() {
        let input = r#"{"outer": {"inner": "value", "num": 42}}"#;
        let result = compact_json(input);
        assert_eq!(
            result,
            "{\n  \"outer\": {\n    \"inner\": \"value\",\n    \"num\": 42\n  }\n}"
        );
    }

    #[test]
    fn test_compact_array_of_objects() {
        let input = r#"[{"a": 1}, {"b": 2}]"#;
        let result = compact_json(input);
        assert_eq!(result, "[\n  {\n    \"a\": 1\n  },\n  {\n    \"b\": 2\n  }\n]");
    }

    #[test]
    fn test_compact_with_null() {
        let input = r#"{"key": null}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"key\": null\n}");
    }

    #[test]
    fn test_compact_with_boolean() {
        let input = r#"{"flag": true, "active": false}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"flag\": true,\n  \"active\": false\n}");
    }

    #[test]
    fn test_compact_with_numbers() {
        let input = r#"{"int": 42, "float": 3.14, "neg": -10}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"int\": 42,\n  \"float\": 3.14,\n  \"neg\": -10\n}");
    }

    #[test]
    fn test_compact_with_special_characters_in_string() {
        let input = r#"{"text": "hello \"world\" and \\backslash"}"#;
        let result = compact_json(input);
        assert_eq!(
            result,
            "{\n  \"text\": \"hello \\\"world\\\" and \\\\backslash\"\n}"
        );
    }

    #[test]
    fn test_compact_large_json_falls_back() {
        // Create a JSON string that will exceed 80 chars when compacted
        let long_string = "a".repeat(100);
        let input = format!(r#"{{"key": "{}"}}"#, long_string);
        let result = compact_json(&input);
        // Should return original since compacted version exceeds 80 chars
        assert_eq!(result, input);
    }

    #[test]
    fn test_compact_small_json_passes() {
        let input = r#"{"key": "short"}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"key\": \"short\"\n}");
        assert!(result.len() <= 80);
    }

    #[test]
    fn test_compact_invalid_json_returns_original() {
        let input = "not valid json";
        let result = compact_json(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_compact_mixed_types_in_array() {
        let input = r#"[1, "two", null, true, {"nested": "obj"}]"#;
        let result = compact_json(input);
        assert_eq!(
            result,
            "[\n  1,\n  \"two\",\n  null,\n  true,\n  {\n    \"nested\": \"obj\"\n  }\n]"
        );
    }

    #[test]
    fn test_format_value_null() {
        assert_eq!(format_value(&Value::Null, 0), "null");
    }

    #[test]
    fn test_format_value_bool() {
        assert_eq!(format_value(&Value::Bool(true), 0), "true");
        assert_eq!(format_value(&Value::Bool(false), 0), "false");
    }

    #[test]
    fn test_format_value_number() {
        assert_eq!(format_value(&Value::Number(serde_json::Number::from(42)), 0), "42");
        assert_eq!(
            format_value(&Value::Number(serde_json::Number::from_f64(3.14).unwrap()), 0),
            "3.14"
        );
    }

    #[test]
    fn test_format_value_string() {
        assert_eq!(
            format_value(&Value::String("hello".to_string()), 0),
            "\"hello\""
        );
    }

    #[test]
    fn test_format_value_string_with_escapes() {
        assert_eq!(
            format_value(&Value::String("hello \"world\"".to_string()), 0),
            "\"hello \\\"world\\\"\""
        );
    }

    #[test]
    fn test_format_value_empty_array() {
        assert_eq!(format_value(&Value::Array(vec![]), 0), "[]");
    }

    #[test]
    fn test_format_value_empty_object() {
        assert_eq!(format_value(&Value::Object(serde_json::Map::new()), 0), "{}");
    }

    #[test]
    fn test_compact_json_with_unicode() {
        let input = r#"{"greeting": "héllo wörld"}"#;
        let result = compact_json(input);
        assert_eq!(result, "{\n  \"greeting\": \"héllo wörld\"\n}");
    }

    #[test]
    fn test_compact_json_preserves_original_on_threshold() {
        // Create a JSON that when compacted is exactly at the threshold
        let input = r#"{"a": "b", "c": "d", "e": "f", "g": "h", "i": "j", "k": "l", "m": "n"}"#;
        let result = compact_json(input);
        // This should be compacted since it's under 80 chars
        assert!(result.len() <= 80);
        assert!(result.contains('\n'));
    }
}