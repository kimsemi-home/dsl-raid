(in-package #:dslraid.agent)

(defparameter *verification-semantic-governance-hashes*
  '(("semantic:authority" "docs/generated/verification-authority.json" ("authority_profile" "decisions" "closure_rules") "Governance authority gate and review decision contract.")
    ("semantic:access-policy" "docs/generated/verification-access-policy.json" ("access_profile" "policies" "closure_rules") "Reasoning RBAC and domain ABAC access contract.")
    ("semantic:reliability" "docs/generated/verification-reliability.json" ("reliability_profile" "records" "closure_rules") "Agent reliability and cold-start contract.")
    ("semantic:agreement" "docs/generated/verification-agreement.json" ("agreement_profile" "agreements" "closure_rules") "Cross-agent agreement and adversarial review contract.")
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
     "Quarantine promotion blocking contract for suspicious outputs.")
    ("semantic:confidence" "docs/generated/verification-confidence.json"
     ("confidence_profile" "ceilings" "closure_rules")
     "External confidence ceiling contract for verification outputs.")
    ("semantic:sidecar" "docs/generated/verification-sidecar.json"
     ("sidecar_profile" "receipts" "closure_rules")
     "Independent verification sidecar receipt contract.")
    ("semantic:execution-projection"
     "docs/generated/verification-execution-projection.json"
     ("execution_projection_profile" "projections" "closure_rules")
     "Command-level execution projection parity contract.")))
