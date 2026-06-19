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
