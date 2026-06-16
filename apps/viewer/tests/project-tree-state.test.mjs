import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("activeFsmId follows the active projection source", async () => {
  const { activeFsmId } = await loadTs("src/panels/project-tree-state.ts");
  const store = storeFor("view:agent", "state:runtime.running");

  assert.equal(activeFsmId(store), "fsm:agent");
});

test("activeFsmId falls back to selected FSM without projection", async () => {
  const { activeFsmId } = await loadTs("src/panels/project-tree-state.ts");
  const store = storeFor(undefined, "fsm:workspace");

  assert.equal(activeFsmId(store), "fsm:workspace");
});

function storeFor(activeProjectionId, selected) {
  return {
    activeProjectionId,
    selection: { selected },
    ir: {
      fsms: [{ id: "fsm:agent" }, { id: "fsm:workspace" }],
      projections: [{ id: "view:agent", source: "fsm:agent" }]
    }
  };
}
