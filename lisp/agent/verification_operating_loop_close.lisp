(in-package #:dslraid.agent)

(defparameter *verification-operating-loop-close-stages*
  '((16 "operating-loop:retrospective" "retrospective"
     "lisp/agent/verification_incident_learning.lisp"
     "docs/generated/verification-incident-learning.json"
     "bash scripts/verificationincidentgen.sh check"
     "stdout:verification incident learning check ok"
     "gate:incident-learning" ("docs/generated/verification-incident-learning.json")
     "Retrospectives turn incidents into learning cycles.")
    (17 "operating-loop:knowledge-update" "knowledge-update"
     "lisp/agent/verification_learning_loop.lisp"
     "docs/generated/verification-learning-loop.json"
     "bash scripts/verificationlearninggen.sh check"
     "stdout:verification learning loop check ok"
     "gate:learning-loop" ("docs/generated/verification-learning-loop.json")
     "Knowledge update closes the operating loop.")))

(setf *verification-operating-loop-stages*
      (append *verification-operating-loop-stages*
              *verification-operating-loop-close-stages*))
