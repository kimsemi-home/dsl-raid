(in-package #:dslraid.agent)

(defparameter *verification-semantic-runtime-diffs*
  '(("semantic-diff:orchestration" "semantic:orchestration"
     "Orchestration routing semantic receipt.")
    ("semantic-diff:control-plane" "semantic:control-plane"
     "Control-plane verifier semantic receipt.")
    ("semantic-diff:shadow-orchestrator" "semantic:shadow-orchestrator"
     "Shadow orchestrator semantic receipt.")
    ("semantic-diff:provider-compat" "semantic:provider-compat"
     "Provider compatibility semantic receipt.")
    ("semantic-diff:runtime-trace" "semantic:runtime-trace"
     "Runtime trace mapping semantic receipt.")
    ("semantic-diff:domain-fsm-dsl" "semantic:domain-fsm-dsl"
     "Domain FSM DSL semantic receipt.")
    ("semantic-diff:semantic-os" "semantic:semantic-os"
     "Semantic operating system layer semantic receipt.")
    ("semantic-diff:operating-loop" "semantic:operating-loop"
     "Normal operating loop semantic receipt.")
    ("semantic-diff:query-surface" "semantic:query-surface"
     "Query and lazy composition surface semantic receipt.")
    ("semantic-diff:run-manifest" "semantic:run-manifest"
     "Agent run manifest semantic receipt.")
    ("semantic-diff:adr-governance" "semantic:adr-governance"
     "ADR governance semantic receipt.")
    ("semantic-diff:backend-parity" "semantic:backend-parity"
     "Backend parity semantic receipt.")
    ("semantic-diff:github-actions-suite" "semantic:github-actions-suite"
     "GitHub Actions suite semantic receipt.")
    ("semantic-diff:workflow-lineage" "semantic:workflow-lineage"
     "Workflow lineage semantic receipt.")
    ("semantic-diff:release-provenance" "semantic:release-provenance"
     "Release provenance semantic receipt.")
    ("semantic-diff:governed-compiler" "semantic:governed-compiler"
     "Governed compiler farm semantic receipt.")
    ("semantic-diff:executable-knowledge" "semantic:executable-knowledge"
     "Executable knowledge semantic receipt.")
    ("semantic-diff:knowledge-conversion" "semantic:knowledge-conversion"
     "Error to knowledge conversion semantic receipt.")))
