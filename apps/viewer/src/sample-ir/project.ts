import type { Capability, Command, ContextObject, Policy, Project } from "../types";

export const sampleProject: Project = {
  id: "runscope",
  name: "RunScope",
  visibility: "public",
  tags: ["example", "fsm"]
};

export const sampleContexts: ContextObject[] = [
  {
    id: "context:runtime",
    name: "Runtime Context",
    kind: "bounded_context",
    owns: ["fsm:runtime"]
  }
];

export const samplePolicies: Policy[] = [
  {
    id: "policy:no_secret_leak",
    name: "No secret leak",
    kind: "security",
    applies_to: ["transition:runtime.running_to_completed"],
    visibility: "public",
    tags: ["security"]
  }
];

export const sampleCapabilities: Capability[] = [
  {
    id: "capability:runtime_execution",
    name: "Runtime execution",
    kind: "runtime",
    owner: "context:runtime"
  }
];

export const sampleCommands: Command[] = [
  {
    id: "command:runtime_start",
    name: "Start runtime",
    capability: "capability:runtime_execution"
  }
];
