import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

test("RunScope fixture remains an FSM-centered Core IR", async () => {
  const fixtureUrl = new URL("../../../examples/runscope/runscope.raid.json", import.meta.url);
  const ir = JSON.parse(await readFile(fixtureUrl, "utf8"));
  assert.equal(ir.ir_version, "0.1.0");
  assert.equal(ir.fsms.length, 1);
  assert.equal(ir.fsms[0].id, "fsm:runtime");
  assert.equal(ir.fsms[0].states.filter((state) => state.initial).length, 1);
  assert.ok(ir.fsms[0].transitions.some((transition) => transition.id === "running_to_completed"));
});
