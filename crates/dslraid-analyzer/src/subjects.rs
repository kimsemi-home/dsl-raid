use dslraid_core::{action_subject, event_subject, guard_subject, state_subject};
use dslraid_core::{transition_subject, CoreIr, Fsm};

pub(crate) fn all_declared(ir: &CoreIr) -> Vec<String> {
    let mut subjects = Vec::new();
    subjects.push(format!("project:{}", ir.project.id));
    subjects.extend(ir.contexts.iter().map(|item| item.id.clone()));
    subjects.extend(ir.requirements.iter().map(|item| item.id.clone()));
    subjects.extend(ir.capabilities.iter().map(|item| item.id.clone()));
    subjects.extend(ir.policies.iter().map(|item| item.id.clone()));
    subjects.extend(ir.commands.iter().map(|item| item.id.clone()));
    for fsm in &ir.fsms {
        add_fsm_subjects(&mut subjects, fsm);
    }
    subjects.extend(ir.compositions.iter().map(|item| item.id.clone()));
    subjects.extend(ir.projections.iter().map(|item| item.id.clone()));
    subjects.extend(ir.derivations.iter().map(|item| item.id.clone()));
    subjects.extend(ir.artifacts.iter().map(|item| item.id.clone()));
    subjects
}

fn add_fsm_subjects(subjects: &mut Vec<String>, fsm: &Fsm) {
    subjects.push(fsm.id.clone());
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
