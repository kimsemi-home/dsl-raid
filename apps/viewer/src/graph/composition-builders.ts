import type { Fsm, Transition } from "../types";
import { allFlag } from "./composition-flags";
import type { TupleEdge, TupleNode } from "./composition-types";
import { tupleEdgeSubject, tupleMembers, tupleSubject } from "./composition-tuples";

export function tupleNode(compositionId: string, fsms: Fsm[], tuple: string[]): TupleNode {
  const members = tupleMembers(fsms, tuple);
  return {
    subject: tupleSubject(compositionId, members),
    members,
    states: tuple,
    initial: allFlag(fsms, tuple, "initial"),
    terminal: allFlag(fsms, tuple, "terminal")
  };
}

export function tupleEdge(compositionId: string, fsms: Fsm[], from: string[], to: string[], fsm: Fsm, transition: Transition): TupleEdge {
  return {
    subject: tupleEdgeSubject(compositionId, from, to, fsm, transition),
    from: tupleSubject(compositionId, tupleMembers(fsms, from)),
    to: tupleSubject(compositionId, tupleMembers(fsms, to)),
    members: [`transition:${fsm.id.replace("fsm:", "")}.${transition.id}`],
    event: transition.on ? `event:${fsm.id.replace("fsm:", "")}.${transition.on}` : undefined
  };
}
