use serde_json::json;

#[test]
fn approved_manifest_requires_semantic_diff() {
    let mut value = super::manifest();
    value["semantic_diffs"] = json!([]);

    assert_eq!(
        super::issues(&value),
        vec!["approved run requires semantic diff receipt"]
    );
}
