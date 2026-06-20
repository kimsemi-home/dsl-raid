use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 24] {
    [
        (MANIFEST, "docs/generated/verification-versioned-ssot.json"),
        (MANIFEST, "docs/generated/verification-domain-fsm.json"),
        (MANIFEST, "docs/generated/verification-semantic-os.json"),
        (MANIFEST, "docs/generated/verification-operating-loop.json"),
        (MANIFEST, "docs/generated/verification-bounded-context.json"),
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
        (
            MANIFEST,
            "docs/generated/verification-executable-knowledge.json",
        ),
        (MANIFEST, "docs/generated/verification-learning-loop.json"),
        (
            MANIFEST,
            "docs/generated/verification-knowledge-conversion.json",
        ),
        (MANIFEST, "docs/generated/verification-quality-closure.json"),
        (
            MANIFEST,
            "docs/generated/verification-precommit-closure.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-evidence-before-change.json",
        ),
    ]
}
