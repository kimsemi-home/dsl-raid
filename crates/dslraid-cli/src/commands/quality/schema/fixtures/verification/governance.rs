use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 17] {
    [
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
        (MANIFEST, "docs/generated/verification-experiment-loop.json"),
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
