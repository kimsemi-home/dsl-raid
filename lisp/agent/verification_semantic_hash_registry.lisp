(in-package #:dslraid.agent)

(defparameter *verification-semantic-hashes*
  '(("semantic:verification-graph" "docs/generated/verification-evidence.json"
     ("form" "ontology_chain" "verification_nodes" "generated_backends")
     "Verification graph shape and generated backend contract.")
    ("semantic:codegen-map" "docs/generated/verification-codegen.json"
     ("axes")
     "Ontology codegen axes mapped to generated backends.")
    ("semantic:loss-ledger" "docs/generated/verification-loss-ledger.json"
     ("ledger")
     "Declared adapter translation loss without forbidden L4 loss.")
    ("semantic:conformance" "docs/generated/verification-conformance.json"
     ("rules")
     "Required checks for generated backend freshness.")
    ("semantic:authority" "docs/generated/verification-authority.json"
     ("authority_profile" "decisions" "closure_rules")
     "Governance authority gate and review decision contract.")
    ("semantic:evidence-quality" "docs/generated/verification-evidence-quality.json"
     ("evidence_quality_profile" "assessments" "closure_rules")
     "Evidence quality review contract for generated verification evidence.")
    ("semantic:lease" "docs/generated/verification-lease.json"
     ("lease_profile" "leases" "abort_rules" "closure_rules")
     "Lease and abort authority boundary for verification work.")
    ("semantic:review-capacity" "docs/generated/verification-review-capacity.json"
     ("review_capacity_profile" "queues" "overload_rules" "closure_rules")
     "Review capacity and overload freeze boundary for verification work.")
    ("semantic:feedback-closure" "docs/generated/verification-feedback.json"
     ("feedback_profile" "closures" "closure_rules")
     "Feedback closure and revalidation contract for verification learning.")
    ("semantic:quarantine" "docs/generated/verification-quarantine.json"
     ("quarantine_profile" "bundles" "closure_rules")
     "Quarantine promotion blocking contract for suspicious outputs.")))
