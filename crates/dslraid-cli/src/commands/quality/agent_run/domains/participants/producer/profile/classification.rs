pub(super) fn is_cold_start(value: Option<&str>) -> bool {
    matches!(value, Some("T0" | "T1"))
}

pub(super) fn is_high_reasoning(value: Option<&str>) -> bool {
    matches!(value, Some("R3" | "R4"))
}

pub(super) fn is_high_risk_scope(value: Option<&str>) -> bool {
    matches!(
        value,
        Some("security" | "audit" | "ontology" | "incident" | "authority")
    )
}

pub(super) fn is_sensitive_scope(value: Option<&str>) -> bool {
    matches!(value, Some("security" | "audit" | "authority"))
}

pub(super) fn is_trusted(value: Option<&str>) -> bool {
    matches!(value, Some("T3" | "T4"))
}
