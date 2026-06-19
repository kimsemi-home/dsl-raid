import assert from "node:assert/strict";
import { test } from "node:test";
import { fixtureView } from "./helpers/hit-fixture.mjs";
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
