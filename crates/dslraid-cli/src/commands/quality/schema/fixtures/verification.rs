use super::Fixture;

const MANIFEST: &str = "schemas/dslraid-verification-manifest.schema.json";

pub(super) fn schemas() -> [Fixture; 7] {
    [
        (
            "schemas/dslraid-verification-evidence.schema.json",
            "docs/generated/verification-evidence.json",
        ),
        (MANIFEST, "docs/generated/verification-privacy.json"),
        (MANIFEST, "docs/generated/verification-pdca.json"),
        (MANIFEST, "docs/generated/verification-loss-ledger.json"),
        (MANIFEST, "docs/generated/verification-semantic-hash.json"),
        (MANIFEST, "docs/generated/verification-semantic-diff.json"),
        (MANIFEST, "docs/generated/verification-codegen.json"),
    ]
}
