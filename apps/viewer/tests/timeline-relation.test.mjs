import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("timeline relation treats transition endpoints as related evidence", async () => {
  const { eventClass, eventSubject, relatedEvents } = await loadTs("src/panels/timeline/relation.ts");
  const events = [
    event("evt-1", "transition:runtime.starting_to_running", "state:runtime.starting", "state:runtime.running"),
    event("evt-2", "transition:runtime.running_to_completed", "state:runtime.running", "state:runtime.completed")
  ];

  assert.equal(relatedEvents(events, "state:runtime.running").length, 2);
  assert.equal(eventSubject(events[0]), "transition:runtime.starting_to_running");
  assert.ok(eventClass(events[1], "state:runtime.completed").includes("related"));
});

function event(id, subject, from, to) {
  return {
    id,
    timestamp: "2026-06-15T00:00:00Z",
    kind: "transition_completed",
    subject,
    from,
    to,
    status: "ok"
  };
}
