(in-package #:dslraid.agent)

(defparameter *verification-semantic-tail-extra-hashes*
  '(("semantic:source-shape"
     "docs/generated/verification-source-shape.json"
     ("source_shape_profile" "budgets" "closure_rules")
     "Source line budget and public surface minimization contract.")
    ("semantic:objective-coverage"
     "docs/generated/verification-objective-coverage.json"
     ("objective_coverage_profile" "requirements" "closure_rules")
     "Active objective requirement-to-evidence coverage contract.")
    ("semantic:learning-loop" "docs/generated/verification-learning-loop.json"
     ("learning_loop_profile" "stages" "cycles" "closure_rules")
     "Agent Cluster reality-to-revalidation learning loop contract.")
    ("semantic:quality-closure" "docs/generated/verification-quality-closure.json"
     ("quality_closure_profile" "enforced_generators" "closure_rules")
     "Generated backend to quality gate closure contract.")
    ("semantic:precommit-closure" "docs/generated/verification-precommit-closure.json"
     ("precommit_profile" "hook" "commands" "closure_rules")
     "Local pre-commit hook quality gate closure contract.")
    ("semantic:merge-receipt" "docs/generated/verification-merge-receipt.json"
     ("merge_receipt_profile" "receipts" "closure_rules")
     "Post-push branch sync, workflow, and Pages closure receipt contract.")))
