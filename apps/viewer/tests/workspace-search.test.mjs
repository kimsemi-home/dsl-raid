import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";
import { workspaceStore } from "./helpers/workspace-store.mjs";

test("workspace search includes inactive FSM states and transitions", async () => {
  const { subjectsForWorkspaceSearch } = await loadTs("src/graph/workspace-search.ts");
  const subjects = subjectsForWorkspaceSearch(workspaceStore());
  const subjectIds = subjects.map((item) => item.subject);

  assert.ok(subjectIds.includes("fsm:agent"));
  assert.ok(subjectIds.includes("state:agent.planning"));
  assert.ok(subjectIds.includes("transition:agent.plan"));
  assert.equal(subjects.find((item) => item.subject === "state:agent.planning").detail, "AgentFSM");
});

test("selectSubject opens the owning FSM projection", async () => {
  const { selectSubject } = await loadTs("src/app/select-subject.ts");
  const session = { store: workspaceStore() };

  const changed = selectSubject(session, "state:agent.planning");

  assert.equal(changed, true);
  assert.equal(session.store.activeProjectionId, "view:agent");
  assert.equal(session.store.selection.selected, "state:agent.planning");
  assert.equal(session.store.view.source.projection, "view:agent");
});

test("selectSubject keeps projection stable for active FSM subjects", async () => {
  const { selectSubject } = await loadTs("src/app/select-subject.ts");
  const session = { store: workspaceStore() };

  const changed = selectSubject(session, "state:runtime.running");

  assert.equal(changed, false);
  assert.equal(session.store.activeProjectionId, "view:runtime");
  assert.equal(session.store.selection.selected, "state:runtime.running");
});
