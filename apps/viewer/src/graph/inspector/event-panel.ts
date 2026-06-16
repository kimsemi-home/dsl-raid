import type { EventDef, Fsm, InspectorPanel } from "../../types";
import { eventSubject, transitionSubject } from "../ids";

export function eventPanel(fsm: Fsm, event: EventDef): InspectorPanel {
  const transitions = (fsm.transitions ?? []).filter((transition) => transition.on === event.id);
  return {
    subject: eventSubject(fsm.id, event.id),
    title: event.name ?? event.id,
    sections: [
      {
        title: "Event",
        rows: [
          { label: "Parent FSM", value: fsm.id, subject: fsm.id },
          { label: "Kind", value: event.kind ?? "unspecified" },
          { label: "Name", value: event.name ?? event.id },
          { label: "Transitions", value: String(transitions.length) }
        ]
      },
      {
        title: "Handled By",
        rows: transitionRows(fsm, transitions)
      }
    ]
  };
}

function transitionRows(fsm: Fsm, transitions: NonNullable<Fsm["transitions"]>) {
  if (transitions.length === 0) {
    return [{ label: "Transition", value: "none" }];
  }
  return transitions.map((transition) => ({
    label: "Transition",
    value: transition.id,
    subject: transitionSubject(fsm.id, transition.id)
  }));
}
