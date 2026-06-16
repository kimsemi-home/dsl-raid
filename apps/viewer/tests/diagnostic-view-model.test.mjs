import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("FSM projection marks diagnostic state and transition subjects", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const view = projectIr(ir);
  const running = view.nodes.find((node) => node.subject === "state:runtime.running");
  const finish = view.edges.find((edge) => edge.subject === "transition:runtime.finish");
  const panel = view.inspector_panels.find((item) => item.subject === "state:runtime.running");
  const rows = panel.sections.find((section) => section.title === "Diagnostics").rows;

  assert.ok(running.badges.includes("diag:error"));
  assert.equal(running.style.tone, "danger");
  assert.equal(finish.style.tone, "warning");
  assert.equal(rows.find((row) => row.label === "Suggestion").value, "fix state");
});

const ir = {
  ir_version: "0.1.0",
  project: { id: "fixture", name: "fixture" },
  fsms: [
    {
      id: "fsm:runtime",
      name: "Runtime",
      states: [
        { id: "idle", kind: "atomic", initial: true },
        { id: "running", kind: "atomic" }
      ],
      events: [{ id: "start" }],
      transitions: [
        { id: "start", from: "idle", to: "running", on: "start" },
        { id: "finish", from: "running", to: "idle" }
      ]
    }
  ],
  projections: [{ id: "view:runtime", kind: "projection", source: "fsm:runtime" }],
  diagnostics: [
    {
      id: "diag:state",
      code: "FSM999",
      severity: "error",
      message: "state fixture",
      subjects: ["state:runtime.running"],
      suggestion: "fix state"
    },
    {
      id: "diag:transition",
      code: "FSM998",
      severity: "warning",
      message: "transition fixture",
      subjects: ["transition:runtime.finish"]
    }
  ]
};
