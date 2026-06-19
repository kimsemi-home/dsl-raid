(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-hashes*
  '(("semantic:incident-learning"
     "docs/generated/verification-incident-learning.json"
     ("incident_learning_profile" "cycles" "closure_rules")
     "Incident learning observe, owner, update, and prevention contract.")))

(defparameter *verification-semantic-hashes*
  (append *verification-semantic-core-hashes*
          *verification-semantic-tail-hashes*))
