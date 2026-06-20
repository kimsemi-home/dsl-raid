(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-abort-assessments*
  '(("evidence-quality:abort-evidence"
     "docs/generated/verification-abort-evidence.json"
     "high" "abort-evidence" "gate:evidence-quality"
     ("bundles" "closure_rules")
     "Abort evidence is suitable when stopped work keeps evidence and rechecks claims.")
    ("evidence-quality:quarantine-release"
     "docs/generated/verification-quarantine-release.json"
     "high" "quarantine-release" "gate:evidence-quality"
     ("release_gates" "closure_rules")
     "Quarantine release evidence is suitable when release conditions are explicit.")
    ("evidence-quality:failure-recovery"
     "docs/generated/verification-failure-recovery.json"
     "high" "failure-recovery" "gate:evidence-quality"
     ("recoveries" "closure_rules")
     "Failure recovery evidence is suitable when failure paths close through learning.")))
