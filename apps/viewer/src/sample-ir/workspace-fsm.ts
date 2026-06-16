import type { Fsm } from "../types";

export const sampleWorkspaceFsm: Fsm = {
  id: "fsm:workspace",
  name: "WorkspaceFSM",
  context: "context:runtime",
  defined_at: {
    uri: "lisp/runtime/runscope.lisp",
    range: { start_line: 78, end_line: 108 }
  },
  states: [
    { id: "clean", kind: "atomic", initial: true },
    { id: "dirty", kind: "atomic" },
    { id: "syncing", kind: "atomic" },
    { id: "synced", kind: "atomic", terminal: true, terminal_semantics: "success" },
    { id: "conflict", kind: "atomic", terminal: true, terminal_semantics: "failed" }
  ],
  events: [
    { id: "file_changed", kind: "external" },
    { id: "sync_requested", kind: "internal" },
    { id: "sync_completed", kind: "internal" },
    { id: "sync_conflict", kind: "error" }
  ],
  transitions: [
    { id: "clean_to_dirty", from: "clean", to: "dirty", on: "file_changed" },
    { id: "dirty_to_syncing", from: "dirty", to: "syncing", on: "sync_requested" },
    { id: "syncing_to_synced", from: "syncing", to: "synced", on: "sync_completed" },
    { id: "syncing_to_conflict", from: "syncing", to: "conflict", on: "sync_conflict" }
  ]
};
