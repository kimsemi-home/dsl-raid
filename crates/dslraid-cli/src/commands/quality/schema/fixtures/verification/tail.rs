use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 26] {
    [
        (MANIFEST, "docs/generated/verification-versioned-ssot.json"),
        (MANIFEST, "docs/generated/verification-context-map.json"),
        (
            MANIFEST,
            "docs/generated/verification-translation-verifier.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-historical-interpreter.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-ontology-transition.json",
        ),
        (MANIFEST, "docs/generated/verification-ssot-defect.json"),
        (MANIFEST, "docs/generated/verification-root-cause.json"),
        (
            MANIFEST,
            "docs/generated/verification-semantic-debugger.json",
        ),
        (MANIFEST, "docs/generated/verification-pruning.json"),
        (MANIFEST, "docs/generated/verification-security-audit.json"),
        (
            MANIFEST,
            "docs/generated/verification-failure-conditions.json",
        ),
        (MANIFEST, "docs/generated/verification-debt.json"),
        (MANIFEST, "docs/generated/verification-codegen.json"),
        (
            MANIFEST,
            "docs/generated/verification-incident-learning.json",
        ),
        (MANIFEST, "docs/generated/verification-learning-loop.json"),
        (MANIFEST, "docs/generated/verification-quality-closure.json"),
        (
            MANIFEST,
            "docs/generated/verification-precommit-closure.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-evidence-before-change.json",
        ),
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
    ]
}
