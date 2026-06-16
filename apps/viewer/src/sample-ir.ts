import type { CoreIr } from "./types";
import { sampleRuntimeFsm } from "./sample-ir/runtime-fsm";
import { sampleProjections } from "./sample-ir/projections";
import {
  sampleCapabilities,
  sampleCommands,
  sampleContexts,
  samplePolicies,
  sampleProject
} from "./sample-ir/project";

export const sampleIr: CoreIr = {
  ir_version: "0.1.0",
  project: sampleProject,
  contexts: sampleContexts,
  policies: samplePolicies,
  capabilities: sampleCapabilities,
  commands: sampleCommands,
  fsms: [sampleRuntimeFsm],
  projections: sampleProjections,
  derivations: [],
  artifacts: [],
  diagnostics: []
};
