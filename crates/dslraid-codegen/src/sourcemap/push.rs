use dslraid_core::Artifact;

use super::generated::{push, Index};

pub(super) fn generated(
    index: &mut Index,
    artifact: &Artifact,
    subject: String,
    start: usize,
    end: usize,
) {
    push(
        index,
        subject,
        super::location::generated(artifact, start, end),
    );
}
