use super::{Fixture, MANIFEST};

pub(super) fn schemas() -> [Fixture; 1] {
    [(MANIFEST, "docs/generated/verification-source-shape.json")]
}
