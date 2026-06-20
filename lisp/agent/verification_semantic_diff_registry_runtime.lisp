(in-package #:dslraid.agent)

(defparameter *verification-semantic-runtime-diffs*
  '(("semantic-diff:orchestration" "semantic:orchestration"
     "Orchestration routing semantic receipt.")
    ("semantic-diff:control-plane" "semantic:control-plane"
     "Control-plane verifier semantic receipt.")
    ("semantic-diff:provider-compat" "semantic:provider-compat"
     "Provider compatibility semantic receipt.")
    ("semantic-diff:runtime-trace" "semantic:runtime-trace"
     "Runtime trace mapping semantic receipt.")
    ("semantic-diff:run-manifest" "semantic:run-manifest"
     "Agent run manifest semantic receipt.")
    ("semantic-diff:adr-governance" "semantic:adr-governance"
     "ADR governance semantic receipt.")
    ("semantic-diff:backend-parity" "semantic:backend-parity"
     "Backend parity semantic receipt.")
    ("semantic-diff:github-actions-suite" "semantic:github-actions-suite"
     "GitHub Actions suite semantic receipt.")
    ("semantic-diff:release-provenance" "semantic:release-provenance"
     "Release provenance semantic receipt.")))
