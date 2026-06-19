(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-diffs*
  '(("semantic-diff:incident-learning" "semantic:incident-learning"
     "Incident learning semantic receipt.")
    ("semantic-diff:genesis-charter" "semantic:genesis-charter"
     "Genesis charter semantic receipt.")))

(defparameter *verification-semantic-diffs*
  (append *verification-semantic-core-diffs*
          *verification-semantic-tail-diffs*))
