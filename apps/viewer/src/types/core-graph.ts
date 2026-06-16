export type Projection = {
  id: string;
  kind: string;
  source: string;
  show?: string[];
  filters?: Record<string, unknown>;
};

export type Composition = {
  id: string;
  name: string;
  kind: string;
  inputs?: string[];
  state_space?: {
    kind?: string;
    max_materialized_states?: number;
  };
};
