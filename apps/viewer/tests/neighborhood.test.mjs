import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("focusedView narrows selected state by hop depth", async () => {
  const { focusedView } = await loadTs("src/graph/neighborhood.ts");
  const view = fixtureView();

  const oneHop = focusedView(view, "state:runtime.running", 1);
  const twoHop = focusedView(view, "state:runtime.running", 2);

  assert.deepEqual(subjects(oneHop.nodes), [
    "state:runtime.completed",
    "state:runtime.running",
    "state:runtime.starting"
  ]);
  assert.deepEqual(subjects(oneHop.edges), [
    "transition:runtime.running_to_completed",
    "transition:runtime.starting_to_running"
  ]);
  assert.equal(twoHop.nodes.length, 5);
  assert.equal(twoHop.edges.length, 4);
});

test("focusedView keeps full view when the subject is not renderable", async () => {
  const { focusedView } = await loadTs("src/graph/neighborhood.ts");
  const view = fixtureView();

  assert.equal(focusedView(view, "fsm:runtime", 1), view);
});

function fixtureView() {
  const ids = ["idle", "starting", "running", "completed", "failed"];
  return {
    nodes: ids.map((id) => ({ id: `node:${id}`, subject: `state:runtime.${id}` })),
    edges: [
      edge("idle_to_starting", "idle", "starting"),
      edge("starting_to_running", "starting", "running"),
      edge("running_to_completed", "running", "completed"),
      edge("starting_to_failed", "starting", "failed")
    ]
  };
}

function edge(id, from, to) {
  return {
    id: `edge:${id}`,
    subject: `transition:runtime.${id}`,
    from: `node:${from}`,
    to: `node:${to}`,
    route: []
  };
}

function subjects(items) {
  return items.map((item) => item.subject).sort();
}
