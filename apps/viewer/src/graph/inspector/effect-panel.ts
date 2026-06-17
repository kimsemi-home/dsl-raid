import type { Action, Fsm, Guard, InspectorPanel, InspectorRow } from "../../types";
import { actionSubject, guardSubject, transitionSubject } from "../ids";

export function guardPanel(fsm: Fsm, guard: Guard): InspectorPanel {
  return effectPanel("Guard", fsm, guard, guardSubject(fsm.id, guard.id), guardTransitions(fsm, guard.id));
}

export function actionPanel(fsm: Fsm, action: Action): InspectorPanel {
  return effectPanel("Action", fsm, action, actionSubject(fsm.id, action.id), actionTransitions(fsm, action.id));
}

function effectPanel(kind: "Guard" | "Action", fsm: Fsm, effect: Guard | Action, subject: string, transitions: string[]): InspectorPanel {
  return {
    subject,
    title: effect.name ?? effect.id,
    sections: [
      {
        title: kind,
        rows: effectRows(kind, fsm, effect)
      },
      {
        title: "Used By",
        rows: transitionRows(fsm, transitions)
      }
    ]
  };
}

function effectRows(kind: "Guard" | "Action", fsm: Fsm, effect: Guard | Action): InspectorRow[] {
  return [
    { label: "Parent FSM", value: fsm.id, subject: fsm.id },
    { label: "Kind", value: effect.kind ?? "unspecified" },
    ...specificRows(kind, effect),
    { label: "Expression", value: effect.expression?.source ?? "none" },
    { label: "Tags", value: (effect.tags ?? []).join(", ") || "none" }
  ];
}

function specificRows(kind: "Guard" | "Action", effect: Guard | Action): InspectorRow[] {
  if (kind === "Guard") {
    const guard = effect as Guard;
    return [{ label: "Input", value: guard.input ?? guard.capability ?? "none" }];
  }
  const action = effect as Action;
  return [
    { label: "Command", value: action.command ?? action.capability ?? "none", subject: action.command ?? action.capability },
    { label: "Emits", value: (action.emits ?? []).join(", ") || "none" }
  ];
}

function transitionRows(fsm: Fsm, transitions: string[]): InspectorRow[] {
  if (transitions.length === 0) {
    return [{ label: "Transition", value: "none" }];
  }
  return transitions.map((id) => ({ label: "Transition", value: id, subject: transitionSubject(fsm.id, id) }));
}

function guardTransitions(fsm: Fsm, guardId: string): string[] {
  return (fsm.transitions ?? []).filter((transition) => transition.guards?.includes(guardId)).map((transition) => transition.id);
}

function actionTransitions(fsm: Fsm, actionId: string): string[] {
  return (fsm.transitions ?? []).filter((transition) => transition.actions?.includes(actionId)).map((transition) => transition.id);
}
