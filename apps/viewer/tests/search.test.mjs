import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("subjectsForSearch includes inspector-only architecture subjects", async () => {
  const { subjectsForSearch } = await loadTs("src/graph/search.ts");
  const subjects = subjectsForSearch(fixtureView());

  assert.deepEqual(subjects.map((item) => item.subject), [
    "artifact:runtime_fsm.rs",
    "policy:no_secret_leak",
    "project:runscope",
    "state:runtime.running",
    "transition:runtime.running_to_completed"
  ]);
  assert.equal(subjects.find((item) => item.subject === "artifact:runtime_fsm.rs").kind, "artifact");
});

function fixtureView() {
  return {
    nodes: [{ subject: "state:runtime.running", label: "running" }],
    edges: [{ subject: "transition:runtime.running_to_completed", label: "completed" }],
    inspector_panels: [
      { subject: "state:runtime.running", title: "running", sections: [] },
      { subject: "project:runscope", title: "RunScope", sections: [] },
      { subject: "policy:no_secret_leak", title: "No secret leak", sections: [] },
      { subject: "artifact:runtime_fsm.rs", title: "generated/runtime_fsm.rs", sections: [] }
    ]
  };
}
