import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("hitTest uses topmost indexed node before edges", async () => {
  const { hitTest } = await loadTs("src/canvas/hit-test.ts");
  const hit = hitTest(fixtureView(), { x: 35, y: 30 });

  assert.deepEqual(hit, {
    kind: "node",
    subject: "state:runtime.running",
    id: "node:running"
  });
});

test("hitTest finds indexed edges near the pointer", async () => {
  const { hitTest } = await loadTs("src/canvas/hit-test.ts");
  const hit = hitTest(fixtureView(), { x: 220, y: 84 });

  assert.deepEqual(hit, {
    kind: "edge",
    subject: "transition:runtime.running_to_completed",
    id: "edge:running_to_completed"
  });
});

test("visibleNodes queries the spatial index in stable order", async () => {
  const { visibleNodes } = await loadTs("src/canvas/hit-test.ts");
  const nodes = visibleNodes(fixtureView(), { x: 0, y: 0, width: 130, height: 80 });

  assert.deepEqual(nodes.map((node) => node.id), ["node:starting", "node:running"]);
});

function fixtureView() {
  return {
    nodes: [
      node("starting", 0, 0),
      node("running", 24, 16),
      node("completed", 300, 90)
    ],
    edges: [edge("running_to_completed", "running", "completed")]
  };
}

function node(id, x, y) {
  return {
    id: `node:${id}`,
    subject: `state:runtime.${id}`,
    x,
    y,
    width: 100,
    height: 56,
    label: id,
    badges: []
  };
}

function edge(id, from, to) {
  return {
    id: `edge:${id}`,
    subject: `transition:runtime.${id}`,
    from: `node:${from}`,
    to: `node:${to}`,
    route: [{ x: 124, y: 44 }, { x: 270, y: 100 }],
    label: id
  };
}
