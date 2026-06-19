(in-package #:dslraid.agent)

(defparameter *verification-semantic-diffs*
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
    ("semantic-diff:evidence-before-change" "semantic:evidence-before-change"
     "Evidence-before-change semantic receipt.")
    ("semantic-diff:versioned-ssot" "semantic:versioned-ssot"
     "Versioned SSOT semantic receipt.")
    ("semantic-diff:context-map" "semantic:context-map"
     "Context map semantic receipt.")
    ("semantic-diff:historical-interpreter" "semantic:historical-interpreter"
     "Historical interpreter semantic receipt.")))
