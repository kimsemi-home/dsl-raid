import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("diagnostic selection carries every related subject", async () => {
  const { diagnosticSelection, relatedAttribute } = await loadTs("src/panels/diagnostics/subjects.ts");
  const diagnostic = {
    id: "diag:runtime-conflict",
    code: "FSM015",
    severity: "error",
    message: "Two transitions handle the same event.",
    subjects: ["state:runtime.running", "transition:runtime.t1", "transition:runtime.t2"]
  };
  const selection = diagnosticSelection(diagnostic);

  assert.equal(selection.subject, "diag:runtime-conflict");
  assert.deepEqual(selection.related, diagnostic.subjects);
  assert.equal(relatedAttribute(selection.related), diagnostic.subjects.join(" "));
});

test("canvas selection marks related diagnostic subjects as active", async () => {
  const { activeLineWidth, activeStroke, subjectVisualState } = await loadTs("src/canvas/selection.ts");
  const selection = {
    selected: "diag:runtime-conflict",
    related: ["state:runtime.running", "transition:runtime.t1"]
  };
  const state = subjectVisualState(selection, "transition:runtime.t1");

  assert.equal(state.related, true);
  assert.equal(activeStroke("default", state), "#b91c1c");
  assert.equal(activeLineWidth(state), 3);
});

test("subject buttons decode optional related subject lists", async () => {
  const { relatedSubjects } = await loadTs("src/panels/subject-buttons.ts");
  const button = { dataset: { relatedSubjects: "state:a transition:b" } };

  assert.deepEqual(relatedSubjects(button), ["state:a", "transition:b"]);
});

test("diagnostic filter narrows by exact severity", async () => {
  const { diagnosticFilterValue, filterDiagnostics } = await loadTs("src/panels/diagnostics/filter.ts");
  const diagnostics = [diagnostic("diag:1", "error"), diagnostic("diag:2", "warning")];

  assert.deepEqual(filterDiagnostics(diagnostics, "warning").map((item) => item.id), ["diag:2"]);
  assert.equal(filterDiagnostics(diagnostics, "all").length, 2);
  assert.equal(diagnosticFilterValue("unexpected"), "all");
});

function diagnostic(id, severity) {
  return {
    id,
    code: "FSM000",
    severity,
    message: "fixture"
  };
}
