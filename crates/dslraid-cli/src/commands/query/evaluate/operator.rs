use crate::commands::query::model::QueryOperator;
use crate::commands::query::value::{compare_numbers, value_contains, value_exists, value_matches};
use serde_json::Value;

pub(super) fn matches_operator(actual: Option<&Value>, operator: &QueryOperator) -> bool {
    match operator {
        QueryOperator::Exists => actual.is_some_and(value_exists),
        QueryOperator::Missing => actual.is_none_or(|value| !value_exists(value)),
        QueryOperator::Eq(expected) => actual.is_some_and(|value| value_matches(value, expected)),
        QueryOperator::NotEq(expected) => {
            actual.is_some_and(|value| !value_matches(value, expected))
        }
        QueryOperator::Contains(expected) => {
            actual.is_some_and(|value| value_contains(value, expected))
        }
        QueryOperator::Prefix(expected) => matches_prefix(actual, expected),
        QueryOperator::Suffix(expected) => matches_suffix(actual, expected),
        QueryOperator::GreaterThan(expected) => {
            compare_numbers(actual, expected, |left, right| left > right)
        }
        QueryOperator::GreaterOrEqual(expected) => {
            compare_numbers(actual, expected, |left, right| left >= right)
        }
        QueryOperator::LessThan(expected) => {
            compare_numbers(actual, expected, |left, right| left < right)
        }
        QueryOperator::LessOrEqual(expected) => {
            compare_numbers(actual, expected, |left, right| left <= right)
        }
        QueryOperator::In(expected) => {
            actual.is_some_and(|value| expected.iter().any(|item| value_matches(value, item)))
        }
    }
}

fn matches_prefix(actual: Option<&Value>, expected: &str) -> bool {
    actual.is_some_and(|value| {
        value.as_str().is_some_and(|actual| {
            actual
                .to_ascii_lowercase()
                .starts_with(&expected.to_ascii_lowercase())
        })
    })
}

fn matches_suffix(actual: Option<&Value>, expected: &str) -> bool {
    actual.is_some_and(|value| {
        value.as_str().is_some_and(|actual| {
            actual
                .to_ascii_lowercase()
                .ends_with(&expected.to_ascii_lowercase())
        })
    })
}
