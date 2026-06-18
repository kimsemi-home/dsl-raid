use serde_json::Value;

pub(crate) struct CollectionCheck<'a> {
    pub(crate) proposition: &'static str,
    pub(crate) assertion: &'static str,
    pub(crate) code: &'static str,
    pub(crate) layer: &'static str,
    pub(crate) predicate: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) failures: &'a [Value],
    pub(crate) pass_message: &'static str,
    pub(crate) fail_message: &'static str,
    pub(crate) suggestion: &'static str,
}
