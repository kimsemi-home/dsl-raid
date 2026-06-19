(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-assessments*
  '(("evidence-quality:generated-evidence"
     "docs/generated/verification-evidence.json" "high" "release-check"
     "gate:evidence-quality" ("generated_backends" "verification_nodes")
     "Generated backend inventory is suitable for conformance checks.")
    ("evidence-quality:semantic-diff"
     "docs/generated/verification-semantic-diff.json" "high" "review"
     "gate:evidence-quality" ("diffs" "closure_rules")
     "Semantic diff receipts are suitable for meaning-level review.")
    ("evidence-quality:authority"
     "docs/generated/verification-authority.json" "high" "authority-gate"
     "gate:evidence-quality" ("decisions" "closure_rules")
     "Authority decisions are suitable when linked generated evidence is fresh.")
    ("evidence-quality:lease"
     "docs/generated/verification-lease.json" "high" "lease-and-abort"
     "gate:evidence-quality" ("leases" "abort_rules")
     "Lease evidence is suitable when abort preserves evidence.")
    ("evidence-quality:review-capacity"
     "docs/generated/verification-review-capacity.json" "high" "review-capacity"
     "gate:evidence-quality" ("queues" "overload_rules")
     "Review capacity evidence is suitable when overload freezes automation.")
    ("evidence-quality:feedback"
     "docs/generated/verification-feedback.json" "high" "feedback-closure"
     "gate:evidence-quality" ("closures" "closure_rules")
     "Feedback evidence is suitable when closures name update and revalidation.")
    ("evidence-quality:quarantine"
     "docs/generated/verification-quarantine.json" "high" "quarantine"
     "gate:evidence-quality" ("bundles" "closure_rules")
     "Quarantine evidence is suitable when promotion blocks are explicit.")
    ("evidence-quality:confidence"
     "docs/generated/verification-confidence.json" "high" "external-confidence"
     "gate:evidence-quality" ("ceilings" "closure_rules")
     "Confidence evidence is suitable when self confidence is ignored.")))
