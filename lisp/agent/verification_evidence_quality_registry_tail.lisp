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
     "Backup steward evidence is suitable when authority is temporary.")
    ("evidence-quality:revalidation-gate"
     "docs/generated/verification-revalidation-gate.json"
     "high" "revalidation-gate" "gate:evidence-quality"
     ("gates" "closure_rules")
     "Revalidation evidence is suitable when freshness changes authority.")
    ("evidence-quality:evidence-ops"
     "docs/generated/verification-evidence-ops.json"
     "high" "evidence-ops" "gate:evidence-quality"
     ("records" "closure_rules")
     "EvidenceOps evidence is suitable when CI and deploy update evidence graph claims.")
    ("evidence-quality:cold-start-gate"
     "docs/generated/verification-cold-start-gate.json"
     "high" "cold-start-gate" "gate:evidence-quality"
     ("gates" "closure_rules")
     "Cold-start evidence is suitable when early authority is blocked.")
    ("evidence-quality:reasoning-access"
     "docs/generated/verification-reasoning-access.json"
     "high" "reasoning-access" "gate:evidence-quality"
     ("records" "closure_rules")
     "Reasoning access evidence is suitable when tier never implies approval.")
    ("evidence-quality:evidence-separation"
     "docs/generated/verification-evidence-separation.json"
     "high" "evidence-separation" "gate:evidence-quality"
     ("records" "closure_rules")
     "Separation evidence is suitable when raw evidence, interpretation, claim, and artifact stay distinct.")))

(defparameter *verification-evidence-quality-assessments*
  (append *verification-evidence-quality-core-assessments*
          *verification-evidence-quality-tail-assessments*))
