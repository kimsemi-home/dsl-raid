(in-package #:dslraid.agent)

(defparameter *verification-semantic-governance-diffs*
  '(("semantic-diff:authority" "semantic:authority"
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
    ("semantic-diff:abort-evidence" "semantic:abort-evidence"
     "Abort evidence bundle semantic receipt.")
    ("semantic-diff:review-capacity" "semantic:review-capacity"
     "Review capacity semantic receipt.")
    ("semantic-diff:feedback-closure" "semantic:feedback-closure"
     "Feedback closure semantic receipt.")
    ("semantic-diff:quarantine" "semantic:quarantine"
     "Quarantine semantic receipt.")
    ("semantic-diff:quarantine-release" "semantic:quarantine-release"
     "Quarantine release semantic receipt.")
    ("semantic-diff:confidence" "semantic:confidence"
     "External confidence semantic receipt.")
    ("semantic-diff:sidecar" "semantic:sidecar"
     "Verification sidecar semantic receipt.")
    ("semantic-diff:execution-projection" "semantic:execution-projection"
     "Execution projection semantic receipt.")))
