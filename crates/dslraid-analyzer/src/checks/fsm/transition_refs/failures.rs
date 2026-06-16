use serde_json::Value;

pub(super) struct RefFailures {
    pub(super) unknown_from: Vec<Value>,
    pub(super) unknown_to: Vec<Value>,
    pub(super) unknown_events: Vec<Value>,
    pub(super) unknown_guard_action: Vec<Value>,
    pub(super) unknown_requires: Vec<Value>,
}

impl RefFailures {
    pub(super) fn new() -> Self {
        Self {
            unknown_from: Vec::new(),
            unknown_to: Vec::new(),
            unknown_events: Vec::new(),
            unknown_guard_action: Vec::new(),
            unknown_requires: Vec::new(),
        }
    }
}
