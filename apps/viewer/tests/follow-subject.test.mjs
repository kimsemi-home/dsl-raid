import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("followSubject walks state to transition to target state", async () => {
  const { followSubject } = await loadTs("src/graph/follow-subject.ts");
  const view = fixtureView();

  assert.equal(followSubject(view, "state:runtime.running"), "transition:runtime.running_to_completed");
  assert.equal(followSubject(view, "transition:runtime.running_to_completed"), "state:runtime.completed");
});

test("followSubject does nothing at terminal states", async () => {
  const { followSubject } = await loadTs("src/graph/follow-subject.ts");

  assert.equal(followSubject(fixtureView(), "state:runtime.completed"), undefined);
});

function fixtureView() {
  return {
    nodes: [node("running"), node("completed")],
    edges: [edge("running_to_completed", "running", "completed")]
  };
}

function node(id) {
  return { id: `node:${id}`, subject: `state:runtime.${id}` };
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
