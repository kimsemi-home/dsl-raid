(in-package #:dslraid.agent)

(defparameter *verification-backends*
  '(("github-actions" ".github/workflows/verification.yml" "scripts/workflowgen.sh")
    ("gitlab-ci" ".gitlab-ci.yml" "scripts/gitlabgen.sh")
    ("local-makefile" "Makefile" "scripts/makegen.sh")
    ("bazel" "BUILD.bazel" "scripts/bazelgen.sh")
    ("github-release" ".github/workflows/release.yml" "scripts/releasegen.sh")
    ("privacy-manifest" "docs/generated/verification-privacy.json"
     "scripts/verificationprivacygen.sh")
    ("pdca-manifest" "docs/generated/verification-pdca.json"
     "scripts/verificationpdcagen.sh")
    ("loss-ledger" "docs/generated/verification-loss-ledger.json"
     "scripts/verificationlossgen.sh")
    ("semantic-hash" "docs/generated/verification-semantic-hash.json"
     "scripts/verificationsemanticgen.sh")
    ("semantic-diff" "docs/generated/verification-semantic-diff.json"
     "scripts/verificationdiffgen.sh")
    ("authority-manifest" "docs/generated/verification-authority.json"
     "scripts/verificationauthoritygen.sh")
    ("evidence-quality" "docs/generated/verification-evidence-quality.json"
     "scripts/verificationevidencequalitygen.sh")
    ("lease-manifest" "docs/generated/verification-lease.json"
     "scripts/verificationleasegen.sh")
    ("review-capacity" "docs/generated/verification-review-capacity.json"
     "scripts/verificationreviewgen.sh")
    ("feedback-closure" "docs/generated/verification-feedback.json"
     "scripts/verificationfeedbackgen.sh")
    ("quarantine-manifest" "docs/generated/verification-quarantine.json"
     "scripts/verificationquarantinegen.sh")
    ("confidence-manifest" "docs/generated/verification-confidence.json"
     "scripts/verificationconfidencegen.sh")
    ("sidecar-manifest" "docs/generated/verification-sidecar.json"
     "scripts/verificationsidecargen.sh")
    ("orchestration-manifest" "docs/generated/verification-orchestration.json"
     "scripts/verificationorchestrationgen.sh")
    ("evidence-before-change" "docs/generated/verification-evidence-before-change.json"
     "scripts/verificationevidencebeforechangegen.sh")
    ("versioned-ssot" "docs/generated/verification-versioned-ssot.json"
     "scripts/verificationversionedssotgen.sh")
    ("context-map" "docs/generated/verification-context-map.json"
     "scripts/verificationcontextmapgen.sh")
    ("historical-interpreter" "docs/generated/verification-historical-interpreter.json"
     "scripts/verificationhistoricalgen.sh")
    ("ontology-transition" "docs/generated/verification-ontology-transition.json"
     "scripts/verificationtransitiongen.sh")
    ("ssot-defect" "docs/generated/verification-ssot-defect.json"
     "scripts/verificationssotdefectgen.sh")
    ("root-cause" "docs/generated/verification-root-cause.json"
     "scripts/verificationrootcausegen.sh")
    ("rust-code" "generated/runtime_fsm.rs" "scripts/lisp-rustgen.sh")
    ("verification-doc" "docs/generated/verification-graph.md"
     "scripts/verificationdocgen.sh")
    ("docs-index" "docs/generated/generated-docs.md" "scripts/gendocindex.sh")
    ("codegen-map" "docs/generated/verification-codegen.json"
     "scripts/verificationcodegengen.sh")
    ("ontology-manifest" "docs/generated/verification-ontology.json"
     "scripts/verificationontologygen.sh")
    ("conformance-report" "docs/generated/verification-conformance.json"
     "scripts/verificationconformancegen.sh")
    ("evidence-schema" "schemas/dslraid-verification-evidence.schema.json"
     "scripts/verificationschemagen.sh")
    ("manifest-schema" "schemas/dslraid-verification-manifest.schema.json"
     "scripts/verificationmanifestschemagen.sh")
    ("test-manifest" "tests/golden/verification-graph.generated.json"
     "scripts/verificationtestgen.sh")
    ("evidence-json" "docs/generated/verification-evidence.json"
     "scripts/verificationevidencegen.sh")))
