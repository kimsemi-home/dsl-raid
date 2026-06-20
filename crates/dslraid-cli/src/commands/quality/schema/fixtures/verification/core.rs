use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 15] {
    [
        (
            "schemas/dslraid-verification-evidence.schema.json",
            "docs/generated/verification-evidence.json",
        ),
        (MANIFEST, "docs/generated/verification-privacy.json"),
        (MANIFEST, "docs/generated/verification-pdca.json"),
        (MANIFEST, "docs/generated/verification-evidence-ops.json"),
        (MANIFEST, "docs/generated/verification-loss-ledger.json"),
        (MANIFEST, "docs/generated/verification-semantic-hash.json"),
        (MANIFEST, "docs/generated/verification-semantic-diff.json"),
        (MANIFEST, "docs/generated/verification-authority.json"),
        (MANIFEST, "docs/generated/verification-access-policy.json"),
        (
            MANIFEST,
            "docs/generated/verification-reasoning-access.json",
        ),
        (MANIFEST, "docs/generated/verification-reliability.json"),
        (MANIFEST, "docs/generated/verification-agreement.json"),
        (
            MANIFEST,
            "docs/generated/verification-adversarial-review.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-evidence-quality.json",
        ),
        (MANIFEST, "docs/generated/verification-lease.json"),
    ]
}
