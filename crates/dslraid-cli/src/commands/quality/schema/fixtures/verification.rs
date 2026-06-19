use super::Fixture;

const MANIFEST: &str = "schemas/dslraid-verification-manifest.schema.json";

pub(super) fn schemas() -> [Fixture; 28] {
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
        (MANIFEST, "docs/generated/verification-sidecar.json"),
        (MANIFEST, "docs/generated/verification-orchestration.json"),
        (
            MANIFEST,
            "docs/generated/verification-evidence-before-change.json",
        ),
        (MANIFEST, "docs/generated/verification-versioned-ssot.json"),
        (MANIFEST, "docs/generated/verification-context-map.json"),
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
    ]
}
