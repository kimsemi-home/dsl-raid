import type { Fsm } from "./fsm";
import type { Composition, Projection } from "./core-graph";
import type { Artifact, Derivation, Diagnostic } from "./traceability";

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
