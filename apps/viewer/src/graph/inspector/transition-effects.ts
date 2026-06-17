import type { Fsm, InspectorSection, Transition } from "../../types";
import { actionSubject, guardSubject } from "../ids";

export function transitionEffectSection(fsm: Fsm, transition: Transition): InspectorSection {
  return {
    title: "Effects",
    rows: [
      ...effectRows("Guard", transition.guards ?? [], (id) => guardSubject(fsm.id, id)),
      ...effectRows("Action", transition.actions ?? [], (id) => actionSubject(fsm.id, id))
    ]
  };
}

function effectRows(label: string, ids: string[], subject: (id: string) => string) {
  if (ids.length === 0) {
    return [{ label, value: "none" }];
  }
  return ids.map((id) => ({ label, value: id, subject: subject(id) }));
}
