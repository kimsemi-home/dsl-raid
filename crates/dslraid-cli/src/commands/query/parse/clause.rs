use super::super::model::{QueryClause, QueryOperator};
use super::super::syntax::{normalize_query_value, parse_list_value, split_word_operator};
use anyhow::{bail, Result};

type OperatorFactory = fn(String) -> QueryOperator;
type SymbolOperator = (&'static str, OperatorFactory);

pub(super) fn parse_query_clause(clause: &str) -> Result<QueryClause> {
    let clause = clause.trim();
    let lower = clause.to_ascii_lowercase();
    if let Some(key) = lower.strip_suffix(" exists") {
        return query_clause(key, QueryOperator::Exists);
    }
    if let Some(key) = lower.strip_suffix(" missing") {
        return query_clause(key, QueryOperator::Missing);
    }
    if let Some((key, value)) = split_word_operator(clause, "in") {
        return in_clause(clause, key, value);
    }
    parse_symbol_clause(clause)
}

fn in_clause(clause: &str, key: &str, value: &str) -> Result<QueryClause> {
    let values = parse_list_value(value);
    if values.is_empty() {
        bail!("query in-list cannot be empty: {clause}");
    }
    query_clause(key, QueryOperator::In(values))
}

fn parse_symbol_clause(clause: &str) -> Result<QueryClause> {
    for (operator, factory) in symbol_operators() {
        if let Some((key, value)) = clause.split_once(operator) {
            return query_clause(key, factory(normalize_query_value(value)));
        }
    }
    bail!("query clause must use an operator: {clause}")
}

fn symbol_operators() -> [SymbolOperator; 9] {
    [
        ("!=", QueryOperator::NotEq),
        (">=", QueryOperator::GreaterOrEqual),
        ("<=", QueryOperator::LessOrEqual),
        ("~=", QueryOperator::Contains),
        ("^=", QueryOperator::Prefix),
        ("$=", QueryOperator::Suffix),
        (">", QueryOperator::GreaterThan),
        ("<", QueryOperator::LessThan),
        ("=", QueryOperator::Eq),
    ]
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
