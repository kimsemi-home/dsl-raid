use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffWarning {
    pub(crate) code: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) subject: String,
    pub(crate) message: String,
}
