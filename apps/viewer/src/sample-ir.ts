import type { CoreIr } from "./types";
import { sampleAgentFsm } from "./sample-ir/agent-fsm";
import { sampleRuntimeFsm } from "./sample-ir/runtime-fsm";
import { sampleWorkspaceFsm } from "./sample-ir/workspace-fsm";
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
  compositions: [{
    id: "composition:runscope",
    name: "RunScopeFSM",
    kind: "product",
    inputs: ["fsm:runtime", "fsm:agent", "fsm:workspace"],
    state_space: { kind: "lazy", max_materialized_states: 48 }
  }],
  fsms: [sampleRuntimeFsm, sampleAgentFsm, sampleWorkspaceFsm],
  projections: sampleProjections,
  derivations: [],
  artifacts: [],
  diagnostics: []
};
