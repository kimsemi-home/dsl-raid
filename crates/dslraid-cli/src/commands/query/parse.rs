use super::model::{QueryClause, QueryExpression, QueryOperator};
use super::syntax::{normalize_query_value, parse_list_value, split_logical, split_word_operator};
use anyhow::{bail, Result};

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

fn parse_query_clause(clause: &str) -> Result<QueryClause> {
    let clause = clause.trim();
    let lower = clause.to_ascii_lowercase();
    if let Some(key) = lower.strip_suffix(" exists") {
        return query_clause(key, QueryOperator::Exists);
    }
    if let Some(key) = lower.strip_suffix(" missing") {
        return query_clause(key, QueryOperator::Missing);
    }
    if let Some((key, value)) = split_word_operator(clause, "in") {
        let values = parse_list_value(value);
        if values.is_empty() {
            bail!("query in-list cannot be empty: {clause}");
        }
        return query_clause(key, QueryOperator::In(values));
    }
    for (operator, factory) in [
        ("!=", QueryOperator::NotEq as fn(String) -> QueryOperator),
        (">=", QueryOperator::GreaterOrEqual),
        ("<=", QueryOperator::LessOrEqual),
        ("~=", QueryOperator::Contains),
        ("^=", QueryOperator::Prefix),
        ("$=", QueryOperator::Suffix),
        (">", QueryOperator::GreaterThan),
        ("<", QueryOperator::LessThan),
        ("=", QueryOperator::Eq),
    ] {
        if let Some((key, value)) = clause.split_once(operator) {
            return query_clause(key, factory(normalize_query_value(value)));
        }
    }
    bail!("query clause must use an operator: {clause}")
}

fn query_clause(key: &str, operator: QueryOperator) -> Result<QueryClause> {
    let key = key.trim();
    if key.is_empty() {
        bail!("query key cannot be empty");
    }
    Ok(QueryClause {
        key: key.to_ascii_lowercase(),
        operator,
    })
}
