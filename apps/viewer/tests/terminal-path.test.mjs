import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("terminalPath follows the deterministic edge to a terminal state", async () => {
  const { terminalPath } = await loadTs("src/graph/terminal-path.ts");

  assert.deepEqual(
    terminalPath(fixtureView(), "state:runtime.running").map((step) => step.subject),
    ["state:runtime.running", "transition:runtime.a_done", "state:runtime.completed"]
  );
});

test("terminalPath can start from a selected transition", async () => {
  const { terminalPath } = await loadTs("src/graph/terminal-path.ts");

  assert.deepEqual(
    terminalPath(fixtureView(), "transition:runtime.b_fail").map((step) => step.subject),
    ["transition:runtime.b_fail", "state:runtime.failed"]
  );
});

test("terminalPath stops at cycles", async () => {
  const { terminalPath } = await loadTs("src/graph/terminal-path.ts");
  const view = { nodes: [node("running")], edges: [edge("loop", "running", "running")] };

  assert.deepEqual(
    terminalPath(view, "state:runtime.running").map((step) => step.subject),
    ["state:runtime.running", "transition:runtime.loop"]
  );
});

function fixtureView() {
  return {
    nodes: [node("running"), node("completed"), node("failed")],
    edges: [edge("b_fail", "running", "failed"), edge("a_done", "running", "completed")]
  };
}

function node(label) {
  return { id: `node:${label}`, subject: `state:runtime.${label}`, label };
}

function edge(id, from, to) {
  return {
    id: `edge:${id}`,
    subject: `transition:runtime.${id}`,
    from: `node:${from}`,
    to: `node:${to}`,
    label: id,
    route: []
  };
}
