import type { Fsm } from "../types";

export const sampleRuntimeFsm: Fsm = {
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
};
