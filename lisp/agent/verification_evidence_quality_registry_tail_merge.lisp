(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-merge-assessments*
  '(("evidence-quality:merge-receipt"
     "docs/generated/verification-merge-receipt.json"
     "high" "merge-receipt" "gate:evidence-quality"
     ("receipts" "closure_rules")
     "Merge receipt evidence is suitable when post-push verdicts are explicit.")))
