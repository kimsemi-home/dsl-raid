import type { CoreIr } from "./types";

export const sampleIr: CoreIr = {
  ir_version: "0.1.0",
  project: {
    id: "runscope",
    name: "RunScope",
    visibility: "public",
    tags: ["example", "fsm"]
  },
  contexts: [
    {
      id: "context:runtime",
      name: "Runtime Context",
      kind: "bounded_context",
      owns: ["fsm:runtime"]
    }
  ],
  policies: [
    {
      id: "policy:no_secret_leak",
      name: "No secret leak",
      kind: "security",
      applies_to: ["transition:runtime.running_to_completed"],
      visibility: "public",
      tags: ["security"]
    }
  ],
  capabilities: [
    {
      id: "capability:runtime_execution",
      name: "Runtime execution",
      kind: "runtime",
      owner: "context:runtime"
    }
  ],
  commands: [
    {
      id: "command:runtime_start",
      name: "Start runtime",
      capability: "capability:runtime_execution"
    }
  ],
  fsms: [
    {
      id: "fsm:runtime",
      name: "RuntimeFSM",
      context: "context:runtime",
      defined_at: {
        uri: "lisp/runtime/runscope.lisp",
        range: { start_line: 12, end_line: 44 }
      },
      states: [
        { id: "idle", kind: "atomic", initial: true },
        { id: "starting", kind: "atomic" },
        {
          id: "running",
          kind: "atomic",
          tags: ["tested", "generated"],
          defined_at: {
            uri: "lisp/runtime/runscope.lisp",
            range: { start_line: 24, end_line: 26 }
          }
        },
        { id: "completed", kind: "atomic", terminal: true, terminal_semantics: "success" },
        { id: "failed", kind: "atomic", terminal: true, terminal_semantics: "failed" }
      ],
      events: [
        { id: "start_requested", name: "start requested", kind: "external" },
        { id: "start_failed", name: "start failed", kind: "error" }
      ],
      transitions: [
        { id: "idle_to_starting", from: "idle", to: "starting", on: "start_requested" },
        { id: "starting_to_running", from: "starting", to: "running" },
        { id: "starting_to_failed", from: "starting", to: "failed", on: "start_failed" },
        {
          id: "running_to_completed",
          from: "running",
          to: "completed",
          requires: ["policy:no_secret_leak"]
        }
      ]
    }
  ],
  projections: [
    {
      id: "view:runtime",
      kind: "projection",
      source: "fsm:runtime",
      show: ["states", "transitions", "events", "policies", "artifacts"]
    }
  ],
  derivations: [],
  artifacts: [],
  diagnostics: []
};
