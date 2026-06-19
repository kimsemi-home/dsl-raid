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
     "Review capacity semantic receipt.")))
