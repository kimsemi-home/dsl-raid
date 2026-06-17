import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("FSM projection exposes guard and action inspector panels", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const panels = new Map(projectIr(fixtureIr(), "view:runtime").inspector_panels.map((panel) => [panel.subject, panel]));

  assert.equal(row(panels.get("guard:runtime.can_start"), "Kind"), "capability");
  assert.equal(row(panels.get("guard:runtime.can_start"), "Expression"), "runtime-ready");
  assert.equal(row(panels.get("action:runtime.emit_started"), "Command"), "command:runtime_start");
  assert.equal(row(panels.get("transition:runtime.start"), "Guard"), "can_start");
  assert.equal(row(panels.get("transition:runtime.start"), "Action"), "emit_started");
});

function row(panel, label) {
  return panel.sections.flatMap((section) => section.rows).find((item) => item.label === label)?.value;
}

function fixtureIr() {
  return {
    ir_version: "0.1.0",
    project: { id: "runscope", name: "RunScope" },
    commands: [{ id: "command:runtime_start", name: "Start runtime" }],
    fsms: [
      {
        id: "fsm:runtime",
        name: "RuntimeFSM",
        states: [
          { id: "idle", kind: "atomic", initial: true },
          { id: "running", kind: "atomic", terminal: true }
        ],
        guards: [{ id: "can_start", kind: "capability", expression: { language: "lisp", source: "runtime-ready" } }],
        actions: [{ id: "emit_started", kind: "emit", command: "command:runtime_start", emits: ["started"] }],
        transitions: [{ id: "start", from: "idle", to: "running", guards: ["can_start"], actions: ["emit_started"] }]
      }
    ],
    projections: [{ id: "view:runtime", kind: "projection", source: "fsm:runtime" }]
  };
}
