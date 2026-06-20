(in-package #:dslraid.agent)

(defparameter *verification-knowledge-conversion-tail*
  '((6 "knowledge-conversion:incident-learning" "incident-learned"
     "lisp/agent/verification_incident_learning.lisp"
     "docs/generated/verification-incident-learning.json"
     "bash scripts/verificationincidentgen.sh check"
     "stdout:verification incident learning check ok"
     "gate:incident-learning"
     ("docs/generated/verification-incident-learning.json")
     "Incidents close only after knowledge update and prevention.")
    (7 "knowledge-conversion:learning-loop" "knowledge-updated"
     "lisp/agent/verification_learning_loop.lisp"
     "docs/generated/verification-learning-loop.json"
     "bash scripts/verificationlearninggen.sh check"
     "stdout:verification learning loop check ok"
     "gate:learning-loop"
     ("docs/generated/verification-learning-loop.json")
     "Evidence returns through the learning loop.")
    (8 "knowledge-conversion:version-propagated" "version-propagated"
     "lisp/agent/verification_versioned_ssot.lisp"
     "docs/generated/verification-versioned-ssot.json"
     "bash scripts/verificationversionedssotgen.sh check"
     "stdout:verification versioned ssot check ok"
     "gate:versioned-ssot"
     ("docs/generated/verification-versioned-ssot.json")
     "Updated knowledge is versioned before runtime authority.")
    (9 "knowledge-conversion:revalidated" "runtime-revalidated"
     "lisp/agent/verification_revalidation_gate.lisp"
     "docs/generated/verification-revalidation-gate.json"
     "bash scripts/verificationrevalidationgen.sh check"
     "stdout:verification revalidation gate check ok"
     "gate:revalidation"
     ("docs/generated/verification-revalidation-gate.json")
     "Revalidation turns learned knowledge back into authority.")))

(defparameter *verification-knowledge-conversion-steps*
  (append *verification-knowledge-conversion-core*
          *verification-knowledge-conversion-tail*))
