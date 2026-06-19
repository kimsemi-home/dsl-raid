use super::fixture::diff;
use serde_json::json;

#[test]
fn semantic_diff_head_must_match_ssot_hash() {
    let mut value = super::manifest();
    value["semantic_diffs"] = json!([diff(
        "sha256:other",
        json!(["evidence:quality", "evidence:trace"])
    )]);

    assert_eq!(
        super::issues(&value),
        vec!["semantic diff semantic-diff:quality head_hash differs from ssot core hash"]
    );
}
