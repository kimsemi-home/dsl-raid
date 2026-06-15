import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

test("RunScope coverage fixture carries state and transition evidence", async () => {
  const coverageUrl = new URL("../../../examples/runscope/run-001.coverage.json", import.meta.url);
  const coverage = JSON.parse(await readFile(coverageUrl, "utf8"));
  const subjects = new Map(coverage.subjects.map((subject) => [subject.subject, subject]));

  assert.equal(coverage.coverage_version, "0.1.0");
  assert.equal(subjects.get("state:runtime.running").status, "covered");
  assert.equal(subjects.get("state:runtime.failed").status, "uncovered");
  assert.equal(subjects.get("transition:runtime.running_to_completed").status, "covered");
  assert.equal(subjects.get("transition:runtime.starting_to_failed").status, "uncovered");
  assert.equal(subjects.get("artifact:runtime_fsm.rs").status, "not_deployed");
});
