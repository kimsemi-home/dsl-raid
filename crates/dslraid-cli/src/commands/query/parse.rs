mod clause;

use self::clause::parse_query_clause;
use super::model::QueryExpression;
use super::syntax::split_logical;
use anyhow::Result;

pub(super) fn parse_query(expression: &str) -> Result<QueryExpression> {
    let expression = expression.trim();
    if expression.is_empty() || expression == "*" {
        return Ok(QueryExpression {
            groups: vec![Vec::new()],
        });
    }
    let groups = split_logical(expression, "or")
        .into_iter()
        .map(|group| {
            split_logical(&group, "and")
                .into_iter()
                .map(|clause| parse_query_clause(&clause))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(QueryExpression { groups })
}
