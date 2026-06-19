import assert from "node:assert/strict";
import { test } from "node:test";
import { fixtureView } from "./helpers/hit-fixture.mjs";
import { loadTs } from "./helpers/load-ts.mjs";

test("hitTest finds indexed edges near the pointer", async () => {
  const { hitTest } = await loadTs("src/canvas/hit-test.ts");
  const hit = hitTest(fixtureView(), { x: 220, y: 84 });

  assert.deepEqual(hit, {
    kind: "edge",
    subject: "transition:runtime.running_to_completed",
    id: "edge:running_to_completed"
  });
});
