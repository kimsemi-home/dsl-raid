import type { DefinedAt } from "../source";

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
