use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 11] {
    [
        (
            MANIFEST,
            "docs/generated/verification-migration-surface.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-language-expansion.json",
        ),
        (MANIFEST, "docs/generated/verification-genesis-charter.json"),
        (MANIFEST, "docs/generated/verification-meta-model.json"),
        (MANIFEST, "docs/generated/verification-backup-steward.json"),
        (
            MANIFEST,
            "docs/generated/verification-revalidation-gate.json",
        ),
        (MANIFEST, "docs/generated/verification-cold-start-gate.json"),
        (
            MANIFEST,
            "docs/generated/verification-evidence-separation.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-workflow-lineage.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-experiment-decision.json",
        ),
        (MANIFEST, "docs/generated/verification-merge-receipt.json"),
    ]
}
