(in-package #:dslraid.agent)

(defparameter *verification-semantic-ssot-hashes*
  '(("semantic:evidence-before-change"
     "docs/generated/verification-evidence-before-change.json"
     ("evidence_before_change_profile" "changes" "closure_rules")
     "Evidence-before-change and evidence debt contract.")
    ("semantic:versioned-ssot" "docs/generated/verification-versioned-ssot.json"
     ("versioned_ssot_profile" "scopes" "closure_rules")
     "Context and version scoped SSOT authority contract.")
    ("semantic:migration-surface" "docs/generated/verification-migration-surface.json"
     ("migration_surface_profile" "surfaces" "closure_rules")
     "Version migration and compatibility command surface contract.")
    ("semantic:context-map" "docs/generated/verification-context-map.json"
     ("context_map_profile" "translations" "closure_rules")
     "Versioned context translation bridge contract.")
    ("semantic:historical-interpreter"
     "docs/generated/verification-historical-interpreter.json"
     ("historical_interpreter_profile" "interpretations" "closure_rules")
     "Historical evidence interpretation bridge contract.")
    ("semantic:ontology-transition"
     "docs/generated/verification-ontology-transition.json"
     ("ontology_transition_profile" "transitions" "closure_rules")
     "Ontology transition lane and cutover contract.")
    ("semantic:ssot-defect" "docs/generated/verification-ssot-defect.json"
     ("ssot_defect_profile" "defects" "closure_rules")
     "SSOT defect freeze, migration, and verification contract.")
    ("semantic:root-cause" "docs/generated/verification-root-cause.json"
     ("root_cause_profile" "cases" "closure_rules")
     "Root cause candidate validation contract.")
    ("semantic:semantic-debugger" "docs/generated/verification-semantic-debugger.json"
     ("semantic_debugger_profile" "sessions" "closure_rules")
     "Semantic debugger question and evidence contract.")
    ("semantic:evidence-pruning" "docs/generated/verification-pruning.json" ("evidence_pruning_profile" "decisions" "closure_rules") "Evidence pruning retention and tombstone contract.")
    ("semantic:security-audit" "docs/generated/verification-security-audit.json" ("security_audit_profile" "boundaries" "closure_rules") "Security audit permission boundary contract.")
    ("semantic:failure-conditions" "docs/generated/verification-failure-conditions.json" ("failure_profile" "conditions" "closure_rules") "Operational failure condition contract.")
    ("semantic:debt-register" "docs/generated/verification-debt.json" ("debt_profile" "records" "closure_rules") "Operational debt repayment contract.")))
