use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffEndpoint {
    pub(crate) path: String,
    pub(crate) hash: String,
    pub(crate) ir_version: String,
}
