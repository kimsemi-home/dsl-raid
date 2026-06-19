(in-package #:dslraid.agent)

(defparameter *verification-semantic-core-diffs*
  '(("semantic-diff:verification-graph" "semantic:verification-graph"
     "Verification graph semantic receipt.")
    ("semantic-diff:codegen-map" "semantic:codegen-map"
     "Codegen axis semantic receipt.")
    ("semantic-diff:loss-ledger" "semantic:loss-ledger"
     "Translation loss semantic receipt.")
    ("semantic-diff:conformance" "semantic:conformance"
     "Conformance contract semantic receipt.")
    ("semantic-diff:authority" "semantic:authority"
     "Authority gate semantic receipt.")
    ("semantic-diff:access-policy" "semantic:access-policy"
     "Access policy semantic receipt.")
    ("semantic-diff:reliability" "semantic:reliability"
     "Reliability registry semantic receipt.")
    ("semantic-diff:agreement" "semantic:agreement"
     "Cross-agent agreement semantic receipt.")
    ("semantic-diff:evidence-quality" "semantic:evidence-quality"
     "Evidence quality semantic receipt.")
    ("semantic-diff:lease" "semantic:lease"
     "Lease and abort semantic receipt.")
    ("semantic-diff:review-capacity" "semantic:review-capacity"
     "Review capacity semantic receipt.")
    ("semantic-diff:feedback-closure" "semantic:feedback-closure"
     "Feedback closure semantic receipt.")
    ("semantic-diff:quarantine" "semantic:quarantine"
     "Quarantine semantic receipt.")
    ("semantic-diff:confidence" "semantic:confidence"
     "External confidence semantic receipt.")
    ("semantic-diff:sidecar" "semantic:sidecar"
     "Verification sidecar semantic receipt.")
    ("semantic-diff:orchestration" "semantic:orchestration"
     "Orchestration routing semantic receipt.")
    ("semantic-diff:control-plane" "semantic:control-plane"
     "Control-plane verifier semantic receipt.")
    ("semantic-diff:provider-compat" "semantic:provider-compat"
     "Provider compatibility semantic receipt.")
    ("semantic-diff:runtime-trace" "semantic:runtime-trace"
     "Runtime trace mapping semantic receipt.")
    ("semantic-diff:adr-governance" "semantic:adr-governance"
     "ADR governance semantic receipt.")
    ("semantic-diff:backend-parity" "semantic:backend-parity"
     "Backend parity semantic receipt.")
    ("semantic-diff:github-actions-suite" "semantic:github-actions-suite"
     "GitHub Actions suite semantic receipt.")
    ("semantic-diff:release-provenance" "semantic:release-provenance"
     "Release provenance semantic receipt.")
    ("semantic-diff:evidence-before-change" "semantic:evidence-before-change"
     "Evidence-before-change semantic receipt.")
    ("semantic-diff:versioned-ssot" "semantic:versioned-ssot"
     "Versioned SSOT semantic receipt.")
    ("semantic-diff:context-map" "semantic:context-map"
     "Context map semantic receipt.")
    ("semantic-diff:historical-interpreter" "semantic:historical-interpreter"
     "Historical interpreter semantic receipt.")
    ("semantic-diff:ontology-transition" "semantic:ontology-transition"
     "Ontology transition semantic receipt.")
    ("semantic-diff:ssot-defect" "semantic:ssot-defect"
     "SSOT defect semantic receipt.")
    ("semantic-diff:root-cause" "semantic:root-cause"
     "Root cause semantic receipt.")
    ("semantic-diff:semantic-debugger" "semantic:semantic-debugger"
     "Semantic debugger semantic receipt.")
    ("semantic-diff:evidence-pruning" "semantic:evidence-pruning"
     "Evidence pruning semantic receipt.")
    ("semantic-diff:security-audit" "semantic:security-audit"
     "Security audit semantic receipt.")
    ("semantic-diff:failure-conditions" "semantic:failure-conditions"
     "Failure conditions semantic receipt.")
    ("semantic-diff:debt-register" "semantic:debt-register"
     "Debt register semantic receipt.")))
