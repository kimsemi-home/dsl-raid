use crate::{Artifact, CoreIr, Derivation, Fsm, Projection};

impl CoreIr {
    pub fn find_fsm(&self, id: &str) -> Option<&Fsm> {
        self.fsms.iter().find(|fsm| fsm.id == id)
    }

    pub fn find_projection(&self, id: Option<&str>) -> Option<&Projection> {
        match id {
            Some(id) => self
                .projections
                .iter()
                .find(|projection| projection.id == id),
            None => self.projections.first(),
        }
    }

    pub fn artifact_by_id(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.iter().find(|artifact| artifact.id == id)
    }

    pub fn derivation_by_id(&self, id: &str) -> Option<&Derivation> {
        self.derivations
            .iter()
            .find(|derivation| derivation.id == id)
    }
}
