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
     "Meta-model term, owner, and authority gate contract.")))

(defparameter *verification-semantic-hashes*
  (append *verification-semantic-core-hashes*
          *verification-semantic-tail-hashes*))
