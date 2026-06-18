use super::fixtures::runscope_fixture;
use super::subject;
use dslraid_core::load_core_ir;

#[test]
fn values_support_policy_or_terminal_query() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let results = super::super::values(
        &ir,
        "kind=transition and requires~=policy:no_secret_leak or terminal=true",
    )
    .unwrap();

    assert!(results
        .iter()
        .any(|item| subject(item) == Some("transition:runtime.running_to_completed")));
    assert!(results
        .iter()
        .any(|item| subject(item) == Some("state:runtime.completed")));
}

#[test]
fn values_support_numeric_query() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let results = super::super::values(&ir, "kind=fsm and states>=1").unwrap();

    assert!(results
        .iter()
        .any(|item| subject(item) == Some("fsm:runtime")));
}
