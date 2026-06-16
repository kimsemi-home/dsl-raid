use serde_json::Value;

#[derive(Debug)]
pub(crate) struct AssertionSpec {
    pub(crate) proposition: &'static str,
    pub(crate) assertion: &'static str,
    pub(crate) code: &'static str,
    pub(crate) layer: &'static str,
    pub(crate) predicate: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) status: &'static str,
    pub(crate) subjects: Vec<String>,
    pub(crate) evidence: Value,
    pub(crate) message: Option<String>,
    pub(crate) suggestion: Option<String>,
}
