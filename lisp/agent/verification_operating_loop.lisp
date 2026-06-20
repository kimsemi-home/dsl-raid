(in-package #:dslraid.agent)

(defparameter *verification-operating-loop-core-stages*
  '((1 "operating-loop:observation" "observation"
     "examples/runscope/run-001.trace.json" "docs/generated/verification-runtime-trace.json"
     "bash scripts/verificationruntimegen.sh check" "stdout:verification runtime trace check ok"
     "gate:runtime-trace" ("docs/generated/verification-runtime-trace.json")
     "Observation starts from runtime trace evidence.")
    (2 "operating-loop:evidence-collection" "evidence-collection"
     "lisp/agent/verification_evidence.lisp" "docs/generated/verification-evidence.json"
     "bash scripts/verificationevidencegen.sh check" "stdout:verification evidence generated output ok"
     "gate:evidence-graph" ("docs/generated/verification-evidence.json")
     "Collected evidence is file-backed.")
    (3 "operating-loop:evidence-quality-check" "evidence-quality-check"
     "lisp/agent/verification_evidence_quality.lisp"
     "docs/generated/verification-evidence-quality.json"
     "bash scripts/verificationevidencequalitygen.sh check"
     "stdout:verification evidence quality check ok"
     "gate:evidence-quality" ("docs/generated/verification-evidence-quality.json")
     "Evidence quality is checked before interpretation.")
    (4 "operating-loop:ontology-mapping" "ontology-mapping"
     "lisp/agent/verification_context_map.lisp"
     "docs/generated/verification-context-map.json"
     "bash scripts/verificationcontextmapgen.sh check"
     "stdout:verification context map check ok"
     "gate:context-map" ("docs/generated/verification-context-map.json")
     "Evidence gains meaning through context mapping.")
    (5 "operating-loop:version-attribution" "version-attribution"
     "lisp/agent/verification_versioned_ssot.lisp"
     "docs/generated/verification-versioned-ssot.json"
     "bash scripts/verificationversionedssotgen.sh check"
     "stdout:verification versioned ssot check ok"
     "gate:versioned-ssot" ("docs/generated/verification-versioned-ssot.json")
     "Version attribution binds claims to SSOT versions.")
    (6 "operating-loop:root-cause-discovery" "root-cause-discovery"
     "lisp/agent/verification_root_cause.lisp"
     "docs/generated/verification-root-cause.json"
     "bash scripts/verificationrootcausegen.sh check"
     "stdout:verification root cause check ok"
     "gate:root-cause" ("docs/generated/verification-root-cause.json")
     "Root cause is discovered through validated cases.")
    (7 "operating-loop:hypothesis" "hypothesis"
     "lisp/agent/verification_experiment_loop.lisp"
     "docs/generated/verification-experiment-loop.json"
     "bash scripts/verificationexperimentgen.sh check"
     "stdout:verification experiment loop check ok"
     "gate:experiment-loop" ("docs/generated/verification-experiment-loop.json")
     "Hypotheses enter the PDCA experiment loop.")
    (8 "operating-loop:target-verification" "target-verification"
     "lisp/agent/verification_conformance.lisp"
     "docs/generated/verification-conformance.json"
     "bash scripts/verificationconformancegen.sh check"
     "stdout:verification conformance generated output ok"
     "gate:conformance" ("docs/generated/verification-conformance.json")
     "Target verification produces conformance evidence.")))
