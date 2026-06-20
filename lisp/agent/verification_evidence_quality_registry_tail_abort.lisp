(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-abort-assessments*
  '(("evidence-quality:abort-evidence"
     "docs/generated/verification-abort-evidence.json"
     "high" "abort-evidence" "gate:evidence-quality"
     ("bundles" "closure_rules")
     "Abort evidence is suitable when stopped work keeps evidence and rechecks claims.")))
