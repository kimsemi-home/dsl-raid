import type { Composition, Projection } from "../core-graph";
import type { Fsm } from "../fsm";
import type { Artifact, Derivation, Diagnostic } from "../traceability";
import type { Capability } from "./capability";
import type { Command } from "./command";
import type { ContextObject } from "./context";
import type { Policy } from "./policy";
import type { Project } from "./project";
import type { Requirement } from "./requirement";

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
