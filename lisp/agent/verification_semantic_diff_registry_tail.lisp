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
    ("semantic-diff:cold-start-gate" "semantic:cold-start-gate"
     "Cold-start gate semantic receipt.")
    ("semantic-diff:reasoning-access" "semantic:reasoning-access"
     "Reasoning access semantic receipt.")
    ("semantic-diff:evidence-separation" "semantic:evidence-separation"
     "Evidence separation semantic receipt.")))

(defparameter *verification-semantic-diffs*
  (append *verification-semantic-core-diffs*
          *verification-semantic-tail-diffs*))
