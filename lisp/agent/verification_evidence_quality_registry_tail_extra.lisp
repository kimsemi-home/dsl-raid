(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-tail-extra-assessments*
  '(("evidence-quality:merge-readiness"
     "docs/generated/verification-merge-readiness.json"
     "high" "merge-readiness" "gate:evidence-quality"
     ("gates" "closure_rules")
     "Merge evidence is suitable when privacy and generated workflow gates are required.")
    ("evidence-quality:incompleteness-ledger"
     "docs/generated/verification-incompleteness-ledger.json"
     "high" "incompleteness-ledger" "gate:evidence-quality"
     ("unknowns" "closure_rules")
     "Incomplete knowledge evidence is suitable when unknowns are owned and classified.")))

(defparameter *verification-evidence-quality-assessments*
  (append *verification-evidence-quality-core-assessments*
          *verification-evidence-quality-tail-assessments*
          *verification-evidence-quality-tail-extra-assessments*))
