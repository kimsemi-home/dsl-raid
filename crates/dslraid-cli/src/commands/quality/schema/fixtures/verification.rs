use super::Fixture;

mod tail;

pub(super) const MANIFEST: &str = "schemas/dslraid-verification-manifest.schema.json";

pub(super) fn schemas() -> impl Iterator<Item = Fixture> {
    core_schemas().into_iter().chain(tail::schemas())
}

fn core_schemas() -> [Fixture; 31] {
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
        (MANIFEST, "docs/generated/verification-review-capacity.json"),
        (MANIFEST, "docs/generated/verification-feedback.json"),
        (MANIFEST, "docs/generated/verification-quarantine.json"),
        (MANIFEST, "docs/generated/verification-confidence.json"),
        (MANIFEST, "docs/generated/verification-sidecar.json"),
        (MANIFEST, "docs/generated/verification-orchestration.json"),
        (MANIFEST, "docs/generated/verification-control-plane.json"),
        (MANIFEST, "docs/generated/verification-provider-compat.json"),
        (MANIFEST, "docs/generated/verification-runtime-trace.json"),
        (MANIFEST, "docs/generated/verification-run-manifest.json"),
        (
            MANIFEST,
            "docs/generated/verification-bootstrap-sequence.json",
        ),
        (MANIFEST, "docs/generated/verification-adr-governance.json"),
        (MANIFEST, "docs/generated/verification-backend-parity.json"),
        (MANIFEST, "docs/generated/verification-github-actions.json"),
        (
            MANIFEST,
            "docs/generated/verification-release-provenance.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-incompleteness-ledger.json",
        ),
    ]
}
