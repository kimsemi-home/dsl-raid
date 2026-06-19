(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-hashes*
  '(("semantic:incident-learning"
     "docs/generated/verification-incident-learning.json"
     ("incident_learning_profile" "cycles" "closure_rules")
     "Incident learning observe, owner, update, and prevention contract.")
    ("semantic:genesis-charter"
     "docs/generated/verification-genesis-charter.json"
     ("genesis_profile" "charter" "closure_rules")
     "Genesis charter purpose, assumption, owner, and risk contract.")
    ("semantic:meta-model"
     "docs/generated/verification-meta-model.json"
     ("meta_model_profile" "terms" "closure_rules")
     "Meta-model term, owner, and authority gate contract.")
    ("semantic:backup-steward"
     "docs/generated/verification-backup-steward.json"
     ("steward_profile" "assignments" "closure_rules")
     "Backup steward temporary authority and review-back contract.")
    ("semantic:revalidation-gate"
     "docs/generated/verification-revalidation-gate.json"
     ("revalidation_profile" "gates" "closure_rules")
     "Revalidation status, authority effect, and blocked action contract.")))

(defparameter *verification-semantic-hashes*
  (append *verification-semantic-core-hashes*
          *verification-semantic-tail-hashes*))
