use std::collections::BTreeSet;

use crate::{action_subject, event_subject, guard_subject, state_subject, transition_subject};
use crate::{CoreIr, Fsm};

impl CoreIr {
    pub fn semantic_subjects(&self) -> BTreeSet<String> {
        let mut subjects = BTreeSet::new();
        collect_project_subjects(self, &mut subjects);
        for fsm in &self.fsms {
            collect_fsm_subjects(fsm, &mut subjects);
        }
        collect_derived_subjects(self, &mut subjects);
        subjects
    }
}

fn collect_project_subjects(ir: &CoreIr, subjects: &mut BTreeSet<String>) {
    subjects.insert(format!("project:{}", ir.project.id));
    subjects.extend(ir.contexts.iter().map(|item| item.id.clone()));
    subjects.extend(ir.requirements.iter().map(|item| item.id.clone()));
    subjects.extend(ir.capabilities.iter().map(|item| item.id.clone()));
    subjects.extend(ir.policies.iter().map(|item| item.id.clone()));
    subjects.extend(ir.commands.iter().map(|item| item.id.clone()));
}

fn collect_fsm_subjects(fsm: &Fsm, subjects: &mut BTreeSet<String>) {
    subjects.insert(fsm.id.clone());
    subjects.extend(
        fsm.states
            .iter()
            .map(|state| state_subject(&fsm.id, &state.id)),
    );
    subjects.extend(
        fsm.events
            .iter()
            .map(|event| event_subject(&fsm.id, &event.id)),
    );
    subjects.extend(
        fsm.guards
            .iter()
            .map(|guard| guard_subject(&fsm.id, &guard.id)),
    );
    subjects.extend(
        fsm.actions
            .iter()
            .map(|action| action_subject(&fsm.id, &action.id)),
    );
    subjects.extend(
        fsm.transitions
            .iter()
            .map(|transition| transition_subject(&fsm.id, &transition.id)),
    );
}

fn collect_derived_subjects(ir: &CoreIr, subjects: &mut BTreeSet<String>) {
    subjects.extend(ir.compositions.iter().map(|item| item.id.clone()));
    subjects.extend(ir.projections.iter().map(|item| item.id.clone()));
    subjects.extend(ir.derivations.iter().map(|item| item.id.clone()));
    subjects.extend(ir.artifacts.iter().map(|item| item.id.clone()));
    subjects.extend(ir.diagnostics.iter().map(|item| item.id.clone()));
}
