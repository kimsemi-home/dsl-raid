import type { Fsm } from "../types";

export function allFlag(fsms: Fsm[], tuple: string[], flag: "initial" | "terminal"): boolean {
  return fsms.every((fsm, index) => {
    return (fsm.states ?? []).some((state) => state.id === tuple[index] && state[flag]);
  });
}
