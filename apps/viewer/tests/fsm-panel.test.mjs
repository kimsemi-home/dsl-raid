import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("fsmPanel exposes execution summary rows", async () => {
  const { fsmPanel } = await loadTs("src/graph/inspector/fsm-panel.ts");
  const panel = fsmPanel(fixtureIr(), fixtureFsm());
  const rows = Object.fromEntries(panel.sections[0].rows.map((row) => [row.label, row.value]));
  const traceRows = panel.sections[1].rows;

  assert.equal(panel.subject, "fsm:runtime");
  assert.equal(rows.Project, "RunScope");
  assert.equal(rows.States, "3");
  assert.equal(rows.Transitions, "2");
  assert.equal(rows.Events, "1");
  assert.equal(rows.Initial, "idle");
  assert.equal(rows.Terminal, "completed");
  assert.equal(traceRows[0].value, "generated/runtime_fsm.rs");
  assert.equal(traceRows[0].subject, "artifact:runtime_fsm.rs");
});

function fixtureIr() {
  return {
    project: { id: "runscope", name: "RunScope" },
    derivations: [
      {
        id: "derivation:runtime.codegen",
        source: "fsm:runtime",
        targets: [{ artifact: "artifact:runtime_fsm.rs", role: "generated" }]
      }
    ],
    artifacts: [
      {
        id: "artifact:runtime_fsm.rs",
        kind: "generated",
        path: "generated/runtime_fsm.rs",
        generated_by: "derivation:runtime.codegen"
      }
    ]
  };
}

function fixtureFsm() {
  return {
    id: "fsm:runtime",
    name: "RuntimeFSM",
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: "running", kind: "atomic" },
      { id: "completed", kind: "atomic", terminal: true }
    ],
    events: [{ id: "start_requested" }],
    transitions: [
      { id: "start", from: "idle", to: "running" },
      { id: "finish", from: "running", to: "completed" }
    ]
  };
}
