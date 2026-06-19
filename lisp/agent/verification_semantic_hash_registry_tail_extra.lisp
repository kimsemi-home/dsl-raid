(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-extra-hashes*
  '(("semantic:source-shape"
     "docs/generated/verification-source-shape.json"
     ("source_shape_profile" "budgets" "closure_rules")
     "Source line budget and public surface minimization contract.")))

(defparameter *verification-semantic-hashes*
  (append *verification-semantic-core-hashes*
          *verification-semantic-tail-hashes*
          *verification-semantic-tail-extra-hashes*))
