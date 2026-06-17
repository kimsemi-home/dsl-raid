import type { AppStore } from "../store/app-store";
import { actionSubject, eventSubject, guardSubject, stateSubject, transitionSubject } from "./ids";
import { subjectsForSearch, type SearchSubject } from "./search";

export function subjectsForWorkspaceSearch(store: AppStore): SearchSubject[] {
  return unique([...subjectsForSearch(store.view), ...fsmSubjects(store)]);
}

function fsmSubjects(store: AppStore): SearchSubject[] {
  return (store.ir.fsms ?? []).flatMap((fsm) => [
    { subject: fsm.id, label: fsm.name, kind: "fsm" },
    ...(fsm.states ?? []).map((state) => ({
      subject: stateSubject(fsm.id, state.id),
      label: state.id,
      kind: "state"
    })),
    ...(fsm.transitions ?? []).map((transition) => ({
      subject: transitionSubject(fsm.id, transition.id),
      label: transition.id,
      kind: "transition"
    })),
    ...(fsm.events ?? []).map((event) => ({
      subject: eventSubject(fsm.id, event.id),
      label: event.name ?? event.id,
      kind: "event"
    })),
    ...(fsm.guards ?? []).map((guard) => ({
      subject: guardSubject(fsm.id, guard.id),
      label: guard.name ?? guard.id,
      kind: "guard"
    })),
    ...(fsm.actions ?? []).map((action) => ({
      subject: actionSubject(fsm.id, action.id),
      label: action.name ?? action.id,
      kind: "action"
    }))
  ]);
}

function unique(subjects: SearchSubject[]): SearchSubject[] {
  const seen = new Set<string>();
  return subjects.filter((item) => {
    if (seen.has(item.subject)) {
      return false;
    }
    seen.add(item.subject);
    return true;
  });
}
