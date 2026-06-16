import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

test("RunScope fixture remains an FSM-centered Core IR", async () => {
  const fixtureUrl = new URL("../../../examples/runscope/runscope.raid.json", import.meta.url);
  const ir = JSON.parse(await readFile(fixtureUrl, "utf8"));
  const fsms = new Map(ir.fsms.map((fsm) => [fsm.id, fsm]));
  const projections = new Map(ir.projections.map((view) => [view.id, view]));

  assert.equal(ir.ir_version, "0.1.0");
  assert.deepEqual([...fsms.keys()], ["fsm:runtime", "fsm:agent", "fsm:workspace"]);
  assert.equal(fsms.get("fsm:runtime").states.filter((state) => state.initial).length, 1);
  assert.equal(fsms.get("fsm:agent").states.filter((state) => state.initial).length, 1);
  assert.equal(fsms.get("fsm:workspace").states.filter((state) => state.initial).length, 1);
  assert.ok(fsms.get("fsm:runtime").transitions.some((t) => t.id === "running_to_completed"));
  assert.deepEqual(ir.compositions[0].inputs, ["fsm:runtime", "fsm:agent", "fsm:workspace"]);
  assert.deepEqual([...projections.keys()], ["view:runtime", "view:agent", "view:workspace"]);
});
