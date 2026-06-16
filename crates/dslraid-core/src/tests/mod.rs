use crate::state_subject;

#[test]
fn derives_state_subjects() {
    assert_eq!(
        state_subject("fsm:runtime", "running"),
        "state:runtime.running"
    );
}
