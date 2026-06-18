use super::super::fields::text;
use serde_json::Value;

pub(super) struct ProducerProfile<'a> {
    value: &'a Value,
}

impl<'a> ProducerProfile<'a> {
    pub(super) fn new(value: &'a Value) -> Self {
        Self { value }
    }

    pub(super) fn id(&self) -> &'a str {
        self.id_value().unwrap_or("<unknown>")
    }

    pub(super) fn id_value(&self) -> Option<&'a str> {
        text(self.value, &["producer", "id"])
    }

    pub(super) fn reasoning(&self) -> Option<&'a str> {
        text(self.value, &["producer", "reasoning_level"])
    }

    pub(super) fn scope(&self) -> Option<&'a str> {
        text(self.value, &["authority_gate", "scope"])
    }

    pub(super) fn trust(&self) -> Option<&'a str> {
        text(self.value, &["producer", "trust_tier"])
    }

    pub(super) fn is_automatic_gate(&self) -> bool {
        text(self.value, &["authority_gate", "profile"]) == Some("automatic")
    }

    pub(super) fn is_cold_start(&self) -> bool {
        matches!(self.trust(), Some("T0" | "T1"))
    }

    pub(super) fn is_high_reasoning(&self) -> bool {
        matches!(self.reasoning(), Some("R3" | "R4"))
    }

    pub(super) fn is_high_risk_scope(&self) -> bool {
        matches!(
            self.scope(),
            Some("security" | "audit" | "ontology" | "incident" | "authority")
        )
    }

    pub(super) fn is_sensitive_scope(&self) -> bool {
        matches!(self.scope(), Some("security" | "audit" | "authority"))
    }

    pub(super) fn is_trusted(&self) -> bool {
        matches!(self.trust(), Some("T3" | "T4"))
    }
}
