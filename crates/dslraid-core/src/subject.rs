pub fn fsm_local_name(fsm_id: &str) -> &str {
    fsm_id.strip_prefix("fsm:").unwrap_or(fsm_id)
}

pub fn state_subject(fsm_id: &str, state_id: &str) -> String {
    format!("state:{}.{}", fsm_local_name(fsm_id), state_id)
}

pub fn event_subject(fsm_id: &str, event_id: &str) -> String {
    format!("event:{}.{}", fsm_local_name(fsm_id), event_id)
}

pub fn guard_subject(fsm_id: &str, guard_id: &str) -> String {
    format!("guard:{}.{}", fsm_local_name(fsm_id), guard_id)
}

pub fn action_subject(fsm_id: &str, action_id: &str) -> String {
    format!("action:{}.{}", fsm_local_name(fsm_id), action_id)
}

pub fn transition_subject(fsm_id: &str, transition_id: &str) -> String {
    format!("transition:{}.{}", fsm_local_name(fsm_id), transition_id)
}
