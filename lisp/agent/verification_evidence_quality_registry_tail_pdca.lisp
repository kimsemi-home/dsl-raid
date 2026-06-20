(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-pdca-assessments*
  '(("evidence-quality:experiment-decision"
     "docs/generated/verification-experiment-decision.json"
     "high" "experiment-decision" "gate:evidence-quality"
     ("decisions" "closure_rules")
     "Experiment decision evidence is suitable when checked experiments close to acts.")))
