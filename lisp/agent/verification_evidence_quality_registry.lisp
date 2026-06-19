(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-assessments*
  '(("evidence-quality:generated-evidence" "docs/generated/verification-evidence.json" "high" "release-check" "gate:evidence-quality" ("generated_backends" "verification_nodes") "Generated backend inventory is suitable for conformance checks.")
    ("evidence-quality:semantic-diff" "docs/generated/verification-semantic-diff.json" "high" "review" "gate:evidence-quality" ("diffs" "closure_rules") "Semantic diff receipts are suitable for meaning-level review.")
    ("evidence-quality:authority" "docs/generated/verification-authority.json" "high" "authority-gate" "gate:evidence-quality" ("decisions" "closure_rules") "Authority decisions are suitable when linked generated evidence is fresh.")
    ("evidence-quality:access-policy" "docs/generated/verification-access-policy.json" "high" "access-policy" "gate:evidence-quality" ("policies" "closure_rules") "Access evidence is suitable when RBAC and ABAC gates are explicit.")
    ("evidence-quality:reliability" "docs/generated/verification-reliability.json" "high" "reliability-registry" "gate:evidence-quality" ("records" "closure_rules") "Reliability evidence is suitable when cold-start limits are explicit.")
    ("evidence-quality:agreement" "docs/generated/verification-agreement.json" "high" "cross-agent-agreement" "gate:evidence-quality" ("agreements" "closure_rules") "Agreement evidence is suitable when isolation and adversarial review are explicit.")
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
    ("evidence-quality:orchestration" "docs/generated/verification-orchestration.json" "high" "orchestration" "gate:evidence-quality" ("routes" "closure_rules") "Orchestration evidence is suitable when routes link policy and outputs.")
    ("evidence-quality:control-plane" "docs/generated/verification-control-plane.json" "high" "control-plane-verifier" "gate:evidence-quality" ("routes" "closure_rules") "Control-plane verifier evidence is suitable when shadow and sidecar checks are explicit.")
    ("evidence-quality:provider-compat" "docs/generated/verification-provider-compat.json" "high" "provider-compat" "gate:evidence-quality" ("records" "closure_rules") "Provider compatibility evidence is suitable when required capabilities are explicit.")
    ("evidence-quality:runtime-trace" "docs/generated/verification-runtime-trace.json" "high" "runtime-trace" "gate:evidence-quality" ("mappings" "closure_rules") "Runtime trace evidence is suitable when design and coverage checks pass.")
    ("evidence-quality:adr-governance" "docs/generated/verification-adr-governance.json" "high" "adr-governance" "gate:evidence-quality" ("records" "closure_rules") "ADR governance evidence is suitable when semantic changes are gated.")
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
     "Ontology transition evidence is suitable when lanes and cutover are gated.")
    ("evidence-quality:ssot-defect"
     "docs/generated/verification-ssot-defect.json" "high"
     "ssot-defect" "gate:evidence-quality" ("defects" "closure_rules")
     "SSOT defect evidence is suitable when freeze and migration are explicit.")
    ("evidence-quality:root-cause" "docs/generated/verification-root-cause.json" "medium" "root-cause" "gate:evidence-quality" ("cases" "closure_rules") "Root cause evidence is suitable while candidates remain validation-bound.")
    ("evidence-quality:semantic-debugger" "docs/generated/verification-semantic-debugger.json" "medium" "semantic-debugger" "gate:evidence-quality" ("sessions" "closure_rules") "Debugger evidence is suitable when missing evidence stays explicit.")
    ("evidence-quality:evidence-pruning" "docs/generated/verification-pruning.json" "high" "evidence-pruning" "gate:evidence-quality" ("decisions" "closure_rules") "Pruning evidence is suitable when tombstones and authority are explicit.")
    ("evidence-quality:security-audit" "docs/generated/verification-security-audit.json" "high" "security-audit" "gate:evidence-quality" ("boundaries" "closure_rules") "Security audit evidence is suitable when approval and rollback are explicit.")
    ("evidence-quality:failure-conditions" "docs/generated/verification-failure-conditions.json" "high" "failure-conditions" "gate:evidence-quality" ("conditions" "closure_rules") "Failure condition evidence is suitable when blocked authority is explicit.")
    ("evidence-quality:debt-register" "docs/generated/verification-debt.json" "high" "debt-register" "gate:evidence-quality" ("records" "closure_rules") "Debt evidence is suitable when owner and repayment are explicit.")))
