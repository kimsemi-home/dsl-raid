use dslraid_core::Artifact;

pub(super) fn requires_lock_record(artifact: &Artifact) -> bool {
    artifact.kind == "generated" || artifact.kind == "test" || artifact.kind == "doc"
}
