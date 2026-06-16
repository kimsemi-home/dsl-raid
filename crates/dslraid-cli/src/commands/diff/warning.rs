use super::model::DiffWarning;

pub(super) fn diff_warning(code: &'static str, subject: &str, message: &str) -> DiffWarning {
    DiffWarning {
        code,
        severity: "warning",
        subject: subject.to_string(),
        message: message.to_string(),
    }
}
