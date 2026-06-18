mod classification;

use super::super::super::fields::text;
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
        classification::is_cold_start(self.trust())
    }

    pub(super) fn is_high_reasoning(&self) -> bool {
        classification::is_high_reasoning(self.reasoning())
    }

    pub(super) fn is_high_risk_scope(&self) -> bool {
        classification::is_high_risk_scope(self.scope())
    }

    pub(super) fn is_sensitive_scope(&self) -> bool {
        classification::is_sensitive_scope(self.scope())
    }

    pub(super) fn is_trusted(&self) -> bool {
        classification::is_trusted(self.trust())
    }
}
