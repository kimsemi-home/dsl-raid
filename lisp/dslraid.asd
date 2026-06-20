(asdf:defsystem #:dslraid
  :description "Common Lisp DSL layer for DSLRaid typed executable IR."
  :license "Apache-2.0"
  :serial t
  :components
  ((:module "packages"
    :serial t
    :components
    ((:file "ir") (:file "lang") (:file "dsl") (:file "expansion")
     (:file "conformance") (:file "emit") (:file "surface")))
   (:module "agent"
    :serial t
    :components
    ((:file "package") (:file "principles") (:file "markdown")
     (:file "verification_release_checks_core") (:file "verification_release_checks_governance") (:file "verification_release_checks_runtime") (:file "verification_release_checks_ssot") (:file "verification_release_checks_artifacts") (:file "verification_release_checks") (:file "verification") (:file "verification_backends_core") (:file "verification_backends_tail") (:file "verification_backends") (:file "verification_accessors")
     (:file "verification_markdown")
     (:file "verification_workflow_names") (:file "verification_workflow") (:file "verification_ci_workflow_core") (:file "verification_ci_workflow_jobs_core") (:file "verification_ci_workflow_jobs_runtime") (:file "verification_ci_workflow_jobs_quality") (:file "verification_ci_workflow_jobs_viewer") (:file "verification_ci_workflow_emit") (:file "verification_golden_workflow") (:file "verification_security_workflow_core") (:file "verification_security_workflow_jobs") (:file "verification_security_workflow_emit") (:file "verification_pages_workflow_core") (:file "verification_pages_workflow_job") (:file "verification_pages_workflow_emit") (:file "verification_gitlab")
     (:file "verification_makefile") (:file "verification_bazel")
     (:file "verification_release") (:file "verification_ontology") (:file "verification_codegen_registry") (:file "verification_codegen")
     (:file "verification_privacy") (:file "verification_pdca")
     (:file "verification_evidence_ops") (:file "verification_loss")
     (:file "verification_semantic_hash_registry_foundation") (:file "verification_semantic_hash_registry_governance") (:file "verification_semantic_hash_registry_runtime") (:file "verification_semantic_hash_registry_ssot") (:file "verification_semantic_hash_registry_tail") (:file "verification_semantic_hash_registry_tail_extra") (:file "verification_semantic_hash_registry")
     (:file "verification_semantic_diff_registry_foundation") (:file "verification_semantic_diff_registry_governance") (:file "verification_semantic_diff_registry_runtime") (:file "verification_semantic_diff_registry_ssot") (:file "verification_semantic_diff_registry_tail") (:file "verification_semantic_diff_registry")
     (:file "verification_evidence_quality_registry") (:file "verification_evidence_quality_registry_tail") (:file "verification_evidence_quality_registry_tail_extra")
     (:file "verification_semantic_hash") (:file "verification_semantic_diff")
     (:file "verification_authority") (:file "verification_access_policy") (:file "verification_reasoning_access") (:file "verification_reliability") (:file "verification_agreement") (:file "verification_adversarial_review") (:file "verification_evidence_quality") (:file "verification_lease") (:file "verification_lease_rules") (:file "verification_review_capacity") (:file "verification_review_capacity_rules") (:file "verification_feedback") (:file "verification_feedback_rules") (:file "verification_quarantine") (:file "verification_quarantine_rules") (:file "verification_confidence")
     (:file "verification_sidecar") (:file "verification_orchestration") (:file "verification_control_plane") (:file "verification_provider_compat") (:file "verification_runtime_trace") (:file "verification_runtime_contract") (:file "verification_query_surface") (:file "verification_run_manifest")
     (:file "verification_bootstrap_sequence") (:file "verification_experiment_loop") (:file "verification_merge_readiness") (:file "verification_merge_automation") (:file "verification_branch_protection") (:file "verification_actions_receipt") (:file "verification_source_shape") (:file "verification_objective_coverage") (:file "verification_objective_coverage_emit") (:file "verification_adr_governance") (:file "verification_backend_parity") (:file "verification_execution_projection") (:file "verification_github_actions") (:file "verification_release_provenance") (:file "verification_incident_learning") (:file "verification_learning_loop") (:file "verification_learning_loop_emit") (:file "verification_quality_closure") (:file "verification_precommit_closure")
     (:file "verification_genesis_charter") (:file "verification_meta_model") (:file "verification_backup_steward") (:file "verification_revalidation_gate") (:file "verification_cold_start_gate") (:file "verification_evidence_separation") (:file "verification_evidence_before_change") (:file "verification_versioned_ssot") (:file "verification_migration_surface") (:file "verification_language_expansion") (:file "verification_context_map") (:file "verification_historical_interpreter") (:file "verification_ontology_transition") (:file "verification_ssot_defect") (:file "verification_root_cause") (:file "verification_semantic_debugger") (:file "verification_pruning") (:file "verification_security_audit") (:file "verification_failure_conditions") (:file "verification_debt") (:file "verification_incompleteness_ledger")
     (:file "verification_conformance")
     (:file "verification_evidence") (:file "verification_schema")
     (:file "verification_manifest_schema_variants_core") (:file "verification_manifest_schema_variants_tail") (:file "verification_manifest_schema_variants") (:file "verification_manifest_schema_defs_core") (:file "verification_manifest_schema_defs_quality") (:file "verification_manifest_schema_defs_governance") (:file "verification_manifest_schema_defs_ontology") (:file "verification_manifest_schema_defs_runtime") (:file "verification_manifest_schema_defs_tail") (:file "verification_manifest_schema_defs") (:file "verification_manifest_schema")
     (:file "verification_tests")))
   (:module "ir"
    :serial t
    :components
    ((:file "model")
     (:file "ids")
     (:file "display")))
   (:module "lang"
    :serial t
    :components
    ((:file "ast") (:file "pipeline")
     (:file "boundaries")
     (:file "code-authoring")
     (:file "code-transition")
     (:file "code-identifiers")
     (:file "code-keywords")
     (:file "code-values") (:file "codes")
     (:file "diagnostic")
     (:file "forms") (:file "malformed")
     (:file "identifiers") (:file "keyword-shape")
     (:file "keyword-diagnostic") (:file "keywords")
     (:file "collection-values")
     (:file "enum-values")
     (:file "source-values")
     (:file "values")
     (:file "duplicates")
     (:file "references")
     (:file "required")
     (:file "conformance")
     (:file "expand-items")
     (:file "expand")))
   (:module "dsl" :serial t :components ((:file "expand") (:file "syntax")))
   (:module "expansion" :serial t :components ((:file "normalize")))
   (:module "conformance" :serial t :components ((:file "validation")))
   (:module "emit"
    :serial t
    :components
    ((:file "json-values") (:file "json-source") (:file "json-effect")
     (:file "json-state") (:file "json-event") (:file "json-transition")
     (:file "json-fsm") (:file "json-context") (:file "json-composition")
     (:file "json-projection") (:file "json-derived") (:file "json")
     (:file "backend") (:file "markdown") (:file "diagnostics")))
   (:module "runtime" :serial t :components ((:file "runscope")))))
