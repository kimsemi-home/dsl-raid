import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

test("RunScope trace fixture carries ordered runtime transition evidence", async () => {
  const traceUrl = new URL("../../../examples/runscope/run-001.trace.json", import.meta.url);
  const trace = JSON.parse(await readFile(traceUrl, "utf8"));
  const subjects = trace.events.map((event) => event.subject).filter(Boolean);

  assert.equal(trace.trace_version, "0.1.0");
  assert.equal(trace.run.id, "run-001");
  assert.deepEqual(subjects.slice(1), [
    "transition:runtime.idle_to_starting",
    "transition:runtime.starting_to_running",
    "transition:runtime.running_to_completed"
  ]);
  assert.ok(trace.events.every((event) => event.timestamp));
});
