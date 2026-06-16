import type { Fsm } from "../types";

export type FsmSummary = {
  states: number;
  transitions: number;
  terminals: number;
  initials: number;
};

export function fsmSummary(fsm: Fsm): FsmSummary {
  const states = fsm.states ?? [];
  return {
    states: states.length,
    transitions: (fsm.transitions ?? []).length,
    terminals: states.filter((state) => state.terminal).length,
    initials: states.filter((state) => state.initial).length
  };
}

export function fsmSummaryLabel(summary: FsmSummary): string {
  return [
    `${summary.states} states`,
    `${summary.transitions} transitions`,
    `${summary.terminals} terminal`,
    `${summary.initials} initial`
  ].join(" / ");
}
