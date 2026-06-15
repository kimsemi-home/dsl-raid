use super::model::{QueryExpression, QueryOperator};
use super::value::{compare_numbers, value_contains, value_exists, value_matches};
use serde_json::Value;

pub(super) fn matches_query(item: &Value, expression: &QueryExpression) -> bool {
    expression.groups.iter().any(|group| {
        group.iter().all(|clause| {
            if clause.key == "tag" {
                return item
                    .get("tags")
                    .and_then(Value::as_array)
                    .is_some_and(|tags| {
                        tags.iter()
                            .any(|tag| value_matches_operator(Some(tag), &clause.operator))
                    });
            }
            value_matches_operator(query_value(item, &clause.key), &clause.operator)
        })
    })
}

fn query_value<'a>(item: &'a Value, key: &str) -> Option<&'a Value> {
    let mut current = item;
    for part in key.split('.') {
        current = current.get(part)?;
    }
    Some(current)
}

fn value_matches_operator(actual: Option<&Value>, operator: &QueryOperator) -> bool {
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
        QueryOperator::Prefix(expected) => actual.is_some_and(|value| {
            value.as_str().is_some_and(|actual| {
                actual
                    .to_ascii_lowercase()
                    .starts_with(&expected.to_ascii_lowercase())
            })
        }),
        QueryOperator::Suffix(expected) => actual.is_some_and(|value| {
            value.as_str().is_some_and(|actual| {
                actual
                    .to_ascii_lowercase()
                    .ends_with(&expected.to_ascii_lowercase())
            })
        }),
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
        QueryOperator::In(expected) => actual.is_some_and(|value| {
            expected
                .iter()
                .any(|expected| value_matches(value, expected))
        }),
    }
}
