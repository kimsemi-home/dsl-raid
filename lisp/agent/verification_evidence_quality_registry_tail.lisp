(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-tail-assessments*
  '(("evidence-quality:incident-learning"
     "docs/generated/verification-incident-learning.json"
     "high" "incident-learning" "gate:evidence-quality"
     ("cycles" "closure_rules")
     "Incident learning evidence is suitable when owner, update, and recheck are explicit.")
    ("evidence-quality:genesis-charter"
     "docs/generated/verification-genesis-charter.json"
     "high" "genesis-charter" "gate:evidence-quality"
     ("charter" "closure_rules")
     "Genesis charter evidence is suitable when owner and revalidation are explicit.")
    ("evidence-quality:meta-model"
     "docs/generated/verification-meta-model.json"
     "high" "meta-model" "gate:evidence-quality"
     ("terms" "closure_rules")
     "Meta-model evidence is suitable when terms are owned and gated.")
    ("evidence-quality:backup-steward"
     "docs/generated/verification-backup-steward.json"
     "high" "backup-steward" "gate:evidence-quality"
     ("assignments" "closure_rules")
     "Backup steward evidence is suitable when authority is temporary.")))

(defparameter *verification-evidence-quality-assessments*
  (append *verification-evidence-quality-core-assessments*
          *verification-evidence-quality-tail-assessments*))
