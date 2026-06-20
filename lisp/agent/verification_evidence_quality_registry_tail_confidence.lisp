(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-confidence-assessments*
  '(("evidence-quality:confidence-decision"
     "docs/generated/verification-confidence-decision.json"
     "high" "confidence-decision" "gate:evidence-quality"
     ("decisions" "closure_rules")
     "Confidence decision evidence is suitable when gates close confidence changes.")))
