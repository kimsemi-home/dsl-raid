use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 4] {
    [
        (MANIFEST, "docs/generated/verification-merge-readiness.json"),
        (
            MANIFEST,
            "docs/generated/verification-merge-automation.json",
        ),
        (
            MANIFEST,
            "docs/generated/verification-branch-protection.json",
        ),
        (MANIFEST, "docs/generated/verification-actions-receipt.json"),
    ]
}
