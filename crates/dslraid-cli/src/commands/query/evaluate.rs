mod operator;
mod path;
mod tag;

use super::model::QueryExpression;
use serde_json::Value;

pub(super) fn matches_query(item: &Value, expression: &QueryExpression) -> bool {
    expression.groups.iter().any(|group| {
        group.iter().all(|clause| {
            if clause.key == "tag" {
                return tag::matches_tag(item, &clause.operator);
            }
            operator::matches_operator(path::query_value(item, &clause.key), &clause.operator)
        })
    })
}
