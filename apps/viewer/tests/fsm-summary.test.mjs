import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("fsmSummary describes execution structure for tree rows", async () => {
  const { fsmSummary, fsmSummaryLabel } = await loadTs("src/graph/fsm-summary.ts");
  const summary = fsmSummary({
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: "running", kind: "atomic" },
      { id: "completed", kind: "atomic", terminal: true }
    ],
    transitions: [
      { id: "start", from: "idle", to: "running" },
      { id: "finish", from: "running", to: "completed" }
    ]
  });

  assert.deepEqual(summary, { states: 3, transitions: 2, terminals: 1, initials: 1 });
  assert.equal(fsmSummaryLabel(summary), "3 states / 2 transitions / 1 terminal / 1 initial");
});
