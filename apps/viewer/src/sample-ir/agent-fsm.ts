import type { Fsm } from "../types";

export const sampleAgentFsm: Fsm = {
  id: "fsm:agent",
  name: "AgentFSM",
  context: "context:runtime",
  defined_at: {
    uri: "lisp/runtime/runscope.lisp",
    range: { start_line: 46, end_line: 76 }
  },
  states: [
    { id: "idle", kind: "atomic", initial: true },
    { id: "planning", kind: "atomic" },
    { id: "acting", kind: "atomic" },
    { id: "waiting", kind: "atomic" },
    { id: "completed", kind: "atomic", terminal: true, terminal_semantics: "success" },
    { id: "failed", kind: "atomic", terminal: true, terminal_semantics: "failed" }
  ],
  events: [
    { id: "plan_requested", kind: "internal" },
    { id: "action_completed", kind: "internal" },
    { id: "action_failed", kind: "error" }
  ],
  transitions: [
    { id: "idle_to_planning", from: "idle", to: "planning", on: "plan_requested" },
    { id: "planning_to_acting", from: "planning", to: "acting" },
    { id: "acting_to_waiting", from: "acting", to: "waiting", on: "action_completed" },
    { id: "waiting_to_completed", from: "waiting", to: "completed" },
    { id: "acting_to_failed", from: "acting", to: "failed", on: "action_failed" }
  ]
};
