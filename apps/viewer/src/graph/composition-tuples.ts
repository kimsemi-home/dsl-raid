import type { Fsm, Transition } from "../types";
import { stateSubject, transitionSubject } from "./ids";

export function initialTuple(fsms: Fsm[]): string[] {
  return fsms.map((fsm) => {
    return (fsm.states ?? []).find((state) => state.initial)?.id ?? fsm.states?.[0]?.id ?? "";
  });
}

export function tupleKey(tuple: string[]): string {
  return tuple.join("\u{1f}");
}

export function tupleMembers(fsms: Fsm[], tuple: string[]): string[] {
  return fsms.map((fsm, index) => stateSubject(fsm.id, tuple[index]));
}

export function tupleSubject(compositionId: string, members: string[]): string {
  return `state_tuple:${local(compositionId)}.${members.map(sanitize).join("__")}`;
}

export function tupleEdgeSubject(
  compositionId: string,
  from: string[],
  to: string[],
  fsm: Fsm,
  transition: Transition
): string {
  return [
    `tuple_transition:${local(compositionId)}`,
    sanitize(tupleKey(from)),
    sanitize(tupleKey(to)),
    sanitize(transitionSubject(fsm.id, transition.id))
  ].join(".");
}

export function local(subject: string): string {
  return subject.includes(":") ? subject.split(":").at(-1) ?? subject : subject;
}

function sanitize(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]+/g, "_").replace(/^_+|_+$/g, "");
}
