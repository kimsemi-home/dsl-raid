import type { DefinedAt, Expression } from "./source";

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

export type State = {
  id: string;
  kind: string;
  initial?: boolean;
  terminal?: boolean;
  terminal_semantics?: string;
  defined_at?: DefinedAt;
  visibility?: string;
  tags?: string[];
};

export type EventDef = {
  id: string;
  name?: string;
  kind?: string;
};

export type Guard = {
  id: string;
  name?: string;
  kind?: string;
  capability?: string;
  expression?: Expression;
  input?: string;
  defined_at?: DefinedAt;
  visibility?: string;
  tags?: string[];
};

export type Action = {
  id: string;
  name?: string;
  kind?: string;
  capability?: string;
  command?: string;
  emits?: string[];
  expression?: Expression;
  depends_on?: string[];
  defined_at?: DefinedAt;
  visibility?: string;
  tags?: string[];
};

export type Transition = {
  id: string;
  from: string;
  to: string;
  on?: string;
  guards?: string[];
  actions?: string[];
  requires?: string[];
  defined_at?: DefinedAt;
  visibility?: string;
  tags?: string[];
};
