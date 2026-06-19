(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-tail-assessments*
  '(("evidence-quality:incident-learning"
     "docs/generated/verification-incident-learning.json"
     "high" "incident-learning" "gate:evidence-quality"
     ("cycles" "closure_rules")
     "Incident learning evidence is suitable when owner, update, and recheck are explicit.")))

(defparameter *verification-evidence-quality-assessments*
  (append *verification-evidence-quality-core-assessments*
          *verification-evidence-quality-tail-assessments*))
