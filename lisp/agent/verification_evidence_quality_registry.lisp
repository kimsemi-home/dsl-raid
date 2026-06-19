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
     "Confidence evidence is suitable when self confidence is ignored.")
    ("evidence-quality:sidecar"
     "docs/generated/verification-sidecar.json" "high" "verification-sidecar"
     "gate:evidence-quality" ("receipts" "closure_rules")
     "Sidecar evidence is suitable when producer and verifier differ.")
    ("evidence-quality:orchestration"
     "docs/generated/verification-orchestration.json" "high" "orchestration"
     "gate:evidence-quality" ("routes" "closure_rules")
     "Orchestration evidence is suitable when routes link policy and outputs.")
    ("evidence-quality:evidence-before-change"
     "docs/generated/verification-evidence-before-change.json" "high"
     "evidence-before-change" "gate:evidence-quality" ("changes" "closure_rules")
     "Change evidence is suitable when routine changes link evidence.")
    ("evidence-quality:versioned-ssot"
     "docs/generated/verification-versioned-ssot.json" "high"
     "versioned-ssot" "gate:evidence-quality" ("scopes" "closure_rules")
     "Versioned SSOT evidence is suitable when context and versions are explicit.")
    ("evidence-quality:context-map"
     "docs/generated/verification-context-map.json" "high"
     "context-map" "gate:evidence-quality" ("translations" "closure_rules")
     "Context map evidence is suitable when translation loss is linked.")
    ("evidence-quality:historical-interpreter"
     "docs/generated/verification-historical-interpreter.json" "high"
     "historical-interpreter" "gate:evidence-quality"
     ("interpretations" "closure_rules")
     "Historical interpreter evidence is suitable when old evidence is bridged.")
    ("evidence-quality:ontology-transition"
     "docs/generated/verification-ontology-transition.json" "high"
     "ontology-transition" "gate:evidence-quality"
     ("transitions" "closure_rules")
     "Ontology transition evidence is suitable when lanes and cutover are gated.")))
