import assert from "node:assert/strict";
import { test } from "node:test";
import { fixtureView } from "./helpers/hit-fixture.mjs";
import { loadTs } from "./helpers/load-ts.mjs";

test("visibleNodes queries the spatial index in stable order", async () => {
  const { visibleNodes } = await loadTs("src/canvas/hit-test.ts");
  const nodes = visibleNodes(fixtureView(), { x: 0, y: 0, width: 130, height: 80 });

  assert.deepEqual(nodes.map((node) => node.id), ["node:starting", "node:running"]);
});
