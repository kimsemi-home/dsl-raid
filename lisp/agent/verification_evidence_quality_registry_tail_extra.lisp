(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-tail-extra-assessments*
  '(("evidence-quality:merge-readiness"
     "docs/generated/verification-merge-readiness.json"
     "high" "merge-readiness" "gate:evidence-quality"
     ("gates" "closure_rules")
     "Merge evidence is suitable when privacy and generated workflow gates are required.")
    ("evidence-quality:merge-automation"
     "docs/generated/verification-merge-automation.json"
     "high" "merge-automation" "gate:evidence-quality"
     ("policies" "closure_rules")
     "Merge automation evidence is suitable when PR source and forbidden events are checked.")
    ("evidence-quality:branch-protection"
     "docs/generated/verification-branch-protection.json"
     "high" "branch-protection" "gate:evidence-quality"
     ("requirements" "closure_rules")
     "Branch protection evidence is suitable when required checks map to workflows.")
    ("evidence-quality:actions-receipt"
     "docs/generated/verification-actions-receipt.json"
     "high" "actions-receipt" "gate:evidence-quality"
     ("receipts" "closure_rules")
     "Actions receipt evidence is suitable when remote verdict fields are explicit.")
    ("evidence-quality:source-shape"
     "docs/generated/verification-source-shape.json"
     "high" "source-shape" "gate:evidence-quality"
     ("budgets" "closure_rules")
     "Source shape evidence is suitable when budget and public entrypoint are explicit.")
    ("evidence-quality:domain-fsm-dsl"
     "docs/generated/verification-domain-fsm.json"
     "high" "domain-fsm-dsl" "gate:evidence-quality"
     ("surfaces" "closure_rules")
     "Domain FSM evidence is suitable when runtime code and Mermaid are generated.")
    ("evidence-quality:semantic-os"
     "docs/generated/verification-semantic-os.json"
     "high" "semantic-os" "gate:evidence-quality"
     ("layers" "closure_rules")
     "Semantic OS evidence is suitable when each OS role has executable evidence.")
    ("evidence-quality:objective-coverage"
     "docs/generated/verification-objective-coverage.json"
     "high" "objective-coverage" "gate:evidence-quality"
     ("requirements" "closure_rules")
     "Objective coverage evidence is suitable when each requirement cites evidence.")
    ("evidence-quality:incompleteness-ledger"
     "docs/generated/verification-incompleteness-ledger.json"
     "high" "incompleteness-ledger" "gate:evidence-quality"
     ("unknowns" "closure_rules")
     "Incomplete knowledge evidence is suitable when unknowns are owned and classified.")
    ("evidence-quality:shadow-orchestrator"
     "docs/generated/verification-shadow-orchestrator.json"
     "high" "shadow-orchestrator" "gate:evidence-quality"
     ("routes" "closure_rules")
     "Shadow evidence is suitable when primary and shadow routes are separated.")))

(defparameter *verification-evidence-quality-assessments*
  (append *verification-evidence-quality-core-assessments*
          *verification-evidence-quality-tail-assessments*
          *verification-evidence-quality-tail-extra-assessments*))
