(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-public-assessments*
  '(("evidence-quality:public-projection"
     "docs/generated/verification-public-projection.json"
     "high" "public-projection" "gate:evidence-quality"
     ("decisions" "closure_rules")
     "Public projection evidence is suitable when private sources are excluded.")))
