(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-extra-hashes*
  '(("semantic:source-shape"
     "docs/generated/verification-source-shape.json"
     ("source_shape_profile" "budgets" "closure_rules")
     "Source line budget and public surface minimization contract.")
    ("semantic:objective-coverage"
     "docs/generated/verification-objective-coverage.json"
     ("objective_coverage_profile" "requirements" "closure_rules")
     "Active objective requirement-to-evidence coverage contract.")))
