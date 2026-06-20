(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-compiler-assessments*
  '(("evidence-quality:governed-compiler"
     "docs/generated/verification-governed-compiler.json"
     "high" "governed-compiler-farm" "gate:evidence-quality"
     ("stages" "closure_rules")
     "Governed compiler evidence is suitable when candidate output is gated.")))
