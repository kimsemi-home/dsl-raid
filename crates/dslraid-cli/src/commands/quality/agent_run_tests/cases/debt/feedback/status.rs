#[test]
fn feedback_update_must_be_applied() {
    let value = super::manifest("proposed");

    assert_eq!(
        super::issues(&value),
        vec!["debt debt:review has unapplied feedback update update:review-policy"]
    );
}
