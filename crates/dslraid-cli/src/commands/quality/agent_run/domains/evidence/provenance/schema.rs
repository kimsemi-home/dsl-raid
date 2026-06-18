pub(super) const REQUIRED_FIELDS: &[&str] =
    &["kind", "observed_by", "observed_at", "ontology_version"];

pub(super) fn supports_kind(value: &str) -> bool {
    matches!(
        value,
        "sidecar-assessment" | "runtime-trace" | "generated" | "external" | "human-annotation"
    )
}
