export type CoreIr = {
  ir_version: string;
  project: Project;
  contexts?: ContextObject[];
  requirements?: Requirement[];
  capabilities?: Capability[];
  policies?: Policy[];
  commands?: Command[];
  fsms?: Fsm[];
  compositions?: Composition[];
  projections?: Projection[];
  derivations?: Derivation[];
  artifacts?: Artifact[];
  diagnostics?: Diagnostic[];
};

export type CoverageOverlay = {
  coverage_version: string;
  design_ir: {
    path: string;
    hash?: string;
  };
  traces?: Array<{
    path: string;
    hash?: string;
  }>;
  subjects: CoverageSubject[];
  metadata?: Record<string, unknown>;
};

export type CoverageSubject = {
  subject: string;
  kind: "state" | "transition" | "event" | "guard" | "action" | "artifact";
  status: CoverageStatus;
  count?: number;
  failure_rate?: number;
  last_seen?: string;
  diagnostics?: string[];
};

export type CoverageStatus = "covered" | "uncovered" | "failed" | "flaky" | "deployed" | "not_deployed";

export type Project = {
  id: string;
  name: string;
  visibility?: string;
  tags?: string[];
  metadata?: Record<string, unknown>;
};

export type ContextObject = {
  id: string;
  name: string;
  kind: string;
  owns?: string[];
};

export type Requirement = {
  id: string;
  name: string;
  description?: string;
  satisfied_by?: string[];
  visibility?: string;
  tags?: string[];
};

export type Capability = {
  id: string;
  name: string;
  kind: string;
  owner?: string;
  provides?: string[];
  requires?: string[];
  visibility?: string;
  tags?: string[];
};

export type Policy = {
  id: string;
  name: string;
  kind: string;
  applies_to?: string[];
  visibility?: string;
  tags?: string[];
};

export type Command = {
  id: string;
  name: string;
  capability?: string;
  visibility?: string;
  tags?: string[];
};

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
  capability?: string;
};

export type Action = {
  id: string;
  name?: string;
  capability?: string;
  depends_on?: string[];
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
};

export type Derivation = {
  id: string;
  source: string;
  rule: {
    id: string;
    kind: string;
    generator?: string;
    version?: string;
  };
  targets?: Array<{
    artifact: string;
    role: string;
  }>;
};

export type Artifact = {
  id: string;
  kind: string;
  path: string;
  generated_by?: string;
  visibility?: string;
  tags?: string[];
};

export type Diagnostic = {
  id: string;
  code: string;
  severity: "hint" | "info" | "warning" | "error";
  message: string;
  subjects?: string[];
  suggestion?: string;
};

export type DefinedAt = {
  uri: string;
  range?: {
    start_line?: number;
    start_column?: number;
    end_line?: number;
    end_column?: number;
  };
};

export type ViewModel = {
  view_version: string;
  source: {
    core_ir: string;
    projection: string;
    hash?: string;
  };
  layout: {
    engine: string;
    version: string;
  };
  nodes: SceneNode[];
  edges: SceneEdge[];
  inspector_panels: InspectorPanel[];
};

export type SceneNode = {
  id: string;
  subject: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label: string;
  badges: string[];
  style?: StyleToken;
};

export type SceneEdge = {
  id: string;
  subject: string;
  from: string;
  to: string;
  label?: string;
  route: Point[];
  style?: StyleToken;
};

export type Point = {
  x: number;
  y: number;
};

export type StyleToken = {
  tone?: "default" | "success" | "warning" | "danger" | "muted";
  emphasis?: "normal" | "strong" | "faint";
  coverage?: CoverageStatus;
};

export type InspectorPanel = {
  subject: string;
  title: string;
  sections: InspectorSection[];
};

export type InspectorSection = {
  title: string;
  rows: InspectorRow[];
};

export type InspectorRow = {
  label: string;
  value: string;
  subject?: string;
};

export type Camera = {
  zoom: number;
  panX: number;
  panY: number;
};

export type SelectionState = {
  selected?: string;
  hovered?: string;
};
