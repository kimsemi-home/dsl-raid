import type { Composition, Fsm } from "../types";
import type { CompositionMaterialization, TupleEdge, TupleNode } from "./composition-types";
import { initialTuple, tupleEdgeSubject, tupleKey, tupleMembers, tupleSubject } from "./composition-tuples";

export function materializeComposition(composition: Composition, fsms: Fsm[]): CompositionMaterialization {
  const limit = composition.state_space?.max_materialized_states ?? 48;
  const stateSpace = fsms.reduce((product, fsm) => product * Math.max(1, fsm.states?.length ?? 0), 1);
  const queue: string[][] = [initialTuple(fsms)];
  const seen = new Set<string>();
  const nodes: TupleNode[] = [];
  const edges: TupleEdge[] = [];
  let truncated = false;
  while (queue.length > 0) {
    const tuple = queue.shift() ?? [];
    if (!tuple.length || seen.has(tupleKey(tuple))) {
      continue;
    }
    seen.add(tupleKey(tuple));
    nodes.push(tupleNode(composition.id, fsms, tuple));
    if (seen.size >= limit) {
      truncated = true;
      continue;
    }
    pushOutgoing(composition, fsms, tuple, queue, seen, edges);
  }
  const visible = new Set(nodes.map((node) => node.subject));
  return { stateSpace, truncated, nodes, edges: edges.filter((edge) => visible.has(edge.from) && visible.has(edge.to)) };
}

function pushOutgoing(
  composition: Composition,
  fsms: Fsm[],
  tuple: string[],
  queue: string[][],
  seen: Set<string>,
  edges: TupleEdge[]
): void {
  fsms.forEach((fsm, index) => {
    for (const transition of (fsm.transitions ?? []).filter((candidate) => candidate.from === tuple[index])) {
      const next = [...tuple];
      next[index] = transition.to;
      edges.push(tupleEdge(composition.id, fsms, tuple, next, fsm, transition));
      if (!seen.has(tupleKey(next))) {
        queue.push(next);
      }
    }
  });
}

function tupleNode(compositionId: string, fsms: Fsm[], tuple: string[]): TupleNode {
  const members = tupleMembers(fsms, tuple);
  return {
    subject: tupleSubject(compositionId, members),
    members,
    states: tuple,
    initial: allFlag(fsms, tuple, "initial"),
    terminal: allFlag(fsms, tuple, "terminal")
  };
}

function tupleEdge(compositionId: string, fsms: Fsm[], from: string[], to: string[], fsm: Fsm, transition: NonNullable<Fsm["transitions"]>[number]): TupleEdge {
  return {
    subject: tupleEdgeSubject(compositionId, from, to, fsm, transition),
    from: tupleSubject(compositionId, tupleMembers(fsms, from)),
    to: tupleSubject(compositionId, tupleMembers(fsms, to)),
    members: [`transition:${fsm.id.replace("fsm:", "")}.${transition.id}`],
    event: transition.on ? `event:${fsm.id.replace("fsm:", "")}.${transition.on}` : undefined
  };
}

function allFlag(fsms: Fsm[], tuple: string[], flag: "initial" | "terminal"): boolean {
  return fsms.every((fsm, index) => (fsm.states ?? []).some((state) => state.id === tuple[index] && state[flag]));
}
