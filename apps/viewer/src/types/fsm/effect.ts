import type { DefinedAt, Expression } from "../source";

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
