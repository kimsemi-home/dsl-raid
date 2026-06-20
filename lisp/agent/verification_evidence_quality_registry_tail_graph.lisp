(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-graph-assessments*
  '(("evidence-quality:evidence-graph"
     "docs/generated/verification-evidence-graph.json"
     "high" "evidence-graph" "gate:evidence-quality"
     ("nodes" "edges" "closure_rules")
     "Evidence graph is suitable when evidence, claims, authority, and feedback link.")))
