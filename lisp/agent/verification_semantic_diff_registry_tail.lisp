(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-diffs*
  '(("semantic-diff:incident-learning" "semantic:incident-learning"
     "Incident learning semantic receipt.")
    ("semantic-diff:genesis-charter" "semantic:genesis-charter"
     "Genesis charter semantic receipt.")
    ("semantic-diff:meta-model" "semantic:meta-model"
     "Meta-model semantic receipt.")
    ("semantic-diff:backup-steward" "semantic:backup-steward"
     "Backup steward semantic receipt.")
    ("semantic-diff:revalidation-gate" "semantic:revalidation-gate"
     "Revalidation gate semantic receipt.")
    ("semantic-diff:evidence-ops" "semantic:evidence-ops"
     "Evidence operations semantic receipt.")
    ("semantic-diff:cold-start-gate" "semantic:cold-start-gate"
     "Cold-start gate semantic receipt.")
    ("semantic-diff:reasoning-access" "semantic:reasoning-access"
     "Reasoning access semantic receipt.")
    ("semantic-diff:adversarial-review" "semantic:adversarial-review"
     "Adversarial review semantic receipt.")
    ("semantic-diff:evidence-separation" "semantic:evidence-separation"
     "Evidence separation semantic receipt.")
    ("semantic-diff:bootstrap-sequence" "semantic:bootstrap-sequence"
     "Bootstrap sequence semantic receipt.")
    ("semantic-diff:experiment-loop" "semantic:experiment-loop"
     "Experiment loop semantic receipt.")
    ("semantic-diff:merge-readiness" "semantic:merge-readiness"
     "Merge readiness semantic receipt.")
    ("semantic-diff:merge-automation" "semantic:merge-automation"
     "Merge automation semantic receipt.")
    ("semantic-diff:branch-protection" "semantic:branch-protection"
     "Branch protection semantic receipt.")
    ("semantic-diff:actions-receipt" "semantic:actions-receipt"
     "Remote Actions receipt semantic receipt.")
    ("semantic-diff:incompleteness-ledger" "semantic:incompleteness-ledger"
     "Incompleteness ledger semantic receipt.")))

(defparameter *verification-semantic-diffs*
  (append *verification-semantic-core-diffs*
          *verification-semantic-tail-diffs*))
