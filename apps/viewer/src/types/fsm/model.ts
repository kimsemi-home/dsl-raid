import type { Action, Guard } from "./effect";
import type { EventDef } from "./event";
import type { State } from "./state";
import type { Transition } from "./transition";
import type { DefinedAt } from "../source";

export type Fsm = {
  id: string;
  name: string;
  context?: string;
  states?: State[];
  events?: EventDef[];
  guards?: Guard[];
  actions?: Action[];
  transitions?: Transition[];
  defined_at?: DefinedAt;
  visibility?: string;
  tags?: string[];
};
