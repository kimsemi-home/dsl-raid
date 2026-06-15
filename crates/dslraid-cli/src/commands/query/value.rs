use serde_json::Value;

pub(super) fn value_exists(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(values) => !values.is_empty(),
        Value::String(value) => !value.is_empty(),
        _ => true,
    }
}

pub(super) fn compare_numbers(
    actual: Option<&Value>,
    expected: &str,
    predicate: impl Fn(f64, f64) -> bool,
) -> bool {
    let Some(actual) = actual.and_then(value_as_f64) else {
        return false;
    };
    let Ok(expected) = expected.parse::<f64>() else {
        return false;
    };
    predicate(actual, expected)
}

fn value_as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(value) => value.parse().ok(),
        _ => None,
    }
}

pub(super) fn value_contains(actual: &Value, expected: &str) -> bool {
    match actual {
        Value::String(value) => value
            .to_ascii_lowercase()
            .contains(&expected.to_ascii_lowercase()),
        Value::Array(values) => values.iter().any(|value| value_contains(value, expected)),
        Value::Bool(_) | Value::Number(_) | Value::Null => value_matches(actual, expected),
        Value::Object(object) => object.values().any(|value| value_contains(value, expected)),
    }
}

pub(super) fn value_matches(actual: &Value, expected: &str) -> bool {
    match actual {
        Value::Bool(value) => match expected.to_ascii_lowercase().as_str() {
            "true" | "yes" | "1" => *value,
            "false" | "no" | "0" => !*value,
            _ => false,
        },
        Value::String(value) => value.eq_ignore_ascii_case(expected),
        Value::Number(value) => value.to_string() == expected,
        Value::Array(values) => values.iter().any(|value| value_matches(value, expected)),
        Value::Null => expected.eq_ignore_ascii_case("null"),
        Value::Object(_) => false,
    }
}
