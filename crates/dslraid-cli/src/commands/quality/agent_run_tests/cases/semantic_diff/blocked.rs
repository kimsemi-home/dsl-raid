use super::fixture::diff;
use serde_json::json;

#[test]
fn blocked_semantic_diff_cannot_approve() {
    let mut value = super::manifest();
    let mut item = diff("sha256:core", json!(["evidence:quality"]));
    item["status"] = json!("blocked");
    value["semantic_diffs"] = json!([item]);

    assert_eq!(
        super::issues(&value),
        vec!["approved run cannot use blocked semantic diff semantic-diff:quality"]
    );
}
