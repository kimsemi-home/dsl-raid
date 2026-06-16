import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("visibleSubjects lists renderable FSM subjects deterministically", async () => {
  const { visibleSubjects, visibleSubjectSummary } = await loadTs("src/graph/visible-subjects.ts");
  const view = fixtureView();

  assert.deepEqual(
    visibleSubjects(view).map((item) => item.subject),
    ["state:runtime.completed", "state:runtime.running", "transition:runtime.running_to_completed"]
  );
  assert.deepEqual(visibleSubjectSummary(view), { states: 2, transitions: 1 });
});

function fixtureView() {
  return {
    nodes: [
      node("running", "state:runtime.running"),
      node("completed", "state:runtime.completed")
    ],
    edges: [edge("running_to_completed")]
  };
}

function node(label, subject) {
  return { id: `node:${label}`, subject, label };
}

function edge(id) {
  return {
    id: `edge:${id}`,
    subject: `transition:runtime.${id}`,
    from: "node:running",
    to: "node:completed",
    label: "done",
    route: []
  };
}
