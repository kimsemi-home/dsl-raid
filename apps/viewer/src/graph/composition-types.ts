export type TupleNode = {
  subject: string;
  members: string[];
  states: string[];
  initial: boolean;
  terminal: boolean;
};

export type TupleEdge = {
  subject: string;
  from: string;
  to: string;
  members: string[];
  event?: string;
};

export type CompositionMaterialization = {
  stateSpace: number;
  truncated: boolean;
  nodes: TupleNode[];
  edges: TupleEdge[];
};
