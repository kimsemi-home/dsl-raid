(in-package #:dslraid.agent)

(defparameter *verification-semantic-runtime-hashes*
  '(("semantic:orchestration" "docs/generated/verification-orchestration.json"
     ("orchestration_profile" "routes" "closure_rules")
     "Policy-bound orchestration routing receipt contract.")
    ("semantic:control-plane" "docs/generated/verification-control-plane.json" ("control_plane_profile" "routes" "closure_rules") "Control-plane sidecar and shadow verifier contract.")
    ("semantic:shadow-orchestrator" "docs/generated/verification-shadow-orchestrator.json" ("shadow_orchestrator_profile" "routes" "closure_rules") "Shadow orchestrator divergence and authority gate contract.")
    ("semantic:provider-compat" "docs/generated/verification-provider-compat.json" ("provider_compat_profile" "records" "closure_rules") "Provider protocol and capability compatibility contract.")
    ("semantic:runtime-trace" "docs/generated/verification-runtime-trace.json" ("runtime_trace_profile" "mappings" "closure_rules") "Runtime trace mapping and coverage overlay contract.")
    ("semantic:domain-fsm-dsl" "docs/generated/verification-domain-fsm.json" ("domain_fsm_profile" "surfaces" "closure_rules") "Domain FSM DSL to generated runtime and diagram contract.")
    ("semantic:semantic-os" "docs/generated/verification-semantic-os.json" ("semantic_os_profile" "layers" "closure_rules") "Semantic operating system layer contract.")
    ("semantic:query-surface" "docs/generated/verification-query-surface.json" ("query_surface_profile" "surfaces" "closure_rules") "Query language and lazy composition observable surface contract.")
    ("semantic:run-manifest" "docs/generated/verification-run-manifest.json" ("run_manifest_profile" "records" "closure_rules") "Agent run manifest file-backed contract.")
    ("semantic:adr-governance" "docs/generated/verification-adr-governance.json" ("adr_profile" "records" "closure_rules") "ADR boundary contract for semantic and implementation changes.")
    ("semantic:backend-parity" "docs/generated/verification-backend-parity.json" ("parity_profile" "projections" "closure_rules") "Generated execution backend graph parity contract.")
    ("semantic:github-actions-suite" "docs/generated/verification-github-actions.json" ("workflow_suite_profile" "workflows" "closure_rules") "GitHub Actions workflow ownership and permission contract.")
    ("semantic:release-provenance" "docs/generated/verification-release-provenance.json" ("release_profile" "gates" "closure_rules") "Release promotion provenance and evidence contract.")))
