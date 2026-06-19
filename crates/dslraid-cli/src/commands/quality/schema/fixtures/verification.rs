use super::Fixture;

const MANIFEST: &str = "schemas/dslraid-verification-manifest.schema.json";

pub(super) fn schemas() -> [Fixture; 14] {
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
        (MANIFEST, "docs/generated/verification-authority.json"),
        (
            MANIFEST,
            "docs/generated/verification-evidence-quality.json",
        ),
        (MANIFEST, "docs/generated/verification-lease.json"),
        (MANIFEST, "docs/generated/verification-review-capacity.json"),
        (MANIFEST, "docs/generated/verification-feedback.json"),
        (MANIFEST, "docs/generated/verification-quarantine.json"),
        (MANIFEST, "docs/generated/verification-confidence.json"),
        (MANIFEST, "docs/generated/verification-codegen.json"),
    ]
}
