import type { DefinedAt } from "../source";

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
