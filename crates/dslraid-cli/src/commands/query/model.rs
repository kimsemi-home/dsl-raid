#[derive(Debug, Clone)]
pub(super) struct QueryExpression {
    pub(super) groups: Vec<Vec<QueryClause>>,
}

#[derive(Debug, Clone)]
pub(super) struct QueryClause {
    pub(super) key: String,
    pub(super) operator: QueryOperator,
}

#[derive(Debug, Clone)]
pub(super) enum QueryOperator {
    Eq(String),
    NotEq(String),
    Contains(String),
    Prefix(String),
    Suffix(String),
    GreaterThan(String),
    GreaterOrEqual(String),
    LessThan(String),
    LessOrEqual(String),
    In(Vec<String>),
    Exists,
    Missing,
}
