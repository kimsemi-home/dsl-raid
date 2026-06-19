(in-package #:dslraid.agent)

(defparameter *verification-graph*
  '(:id "verify:daemon"
    :form "(verify daemon (lint) (unit-test) (integration-test) (conformance) (release-check))"
    :nodes
    ((:id "lint" :name "Lint" :needs nil :tooling (:go :rust-lint)
      :evidence "Source shape, workflow lint, Go lint, Rust fmt/clippy."
      :commands
      ("go install github.com/rhysd/actionlint/cmd/actionlint@v1.7.12"
       "actionlint_bin=\"$(go env GOPATH)/bin/actionlint\" && \"$actionlint_bin\""
       "bash scripts/check-source-lines.sh"
       "bash scripts/go-lint.sh"
       "cargo fmt --all -- --check"
       "cargo clippy --workspace --all-targets -- -D warnings"))
     (:id "unit-test" :name "Unit Test" :needs ("lint") :tooling (:rust)
      :evidence "Rust workspace unit test evidence."
      :commands ("cargo test --workspace"))
     (:id "integration-test" :name "Integration Test"
      :needs ("unit-test") :tooling (:node)
      :evidence "Viewer test and build evidence."
      :commands
      ("npm --prefix apps/viewer ci"
       "npm --prefix apps/viewer test"
       "npm --prefix apps/viewer run build"))
     (:id "conformance" :name "Conformance"
      :needs ("integration-test") :tooling (:rust :lisp)
      :evidence "Unified DSLRaid semantic and generated-output gate."
      :commands ("cargo run -p dslraid-cli -- quality"))
     (:id "release-check" :name "Release Check"
      :needs ("conformance") :tooling (:rust :lisp)
      :evidence "Generated workflow, docs index, and artifact freshness."
      :commands
      ("bash scripts/workflowgen.sh check"
       "bash scripts/gitlabgen.sh check"
       "bash scripts/makegen.sh check"
       "bash scripts/bazelgen.sh check"
       "bash scripts/releasegen.sh check"
       "bash scripts/privacycheck.sh check"
       "bash scripts/verificationprivacygen.sh check"
       "bash scripts/verificationpdcagen.sh check"
       "bash scripts/verificationontologygen.sh check"
       "bash scripts/verificationconformancegen.sh check"
       "bash scripts/verificationschemagen.sh check"
       "bash scripts/verificationtestgen.sh check"
       "bash scripts/verificationevidencegen.sh check"
       "bash scripts/gendocindex.sh check"
       "cargo run -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json")))))

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
    ("ontology-manifest" "docs/generated/verification-ontology.json"
     "scripts/verificationontologygen.sh")
    ("conformance-report" "docs/generated/verification-conformance.json"
     "scripts/verificationconformancegen.sh")
    ("evidence-schema" "schemas/dslraid-verification-evidence.schema.json"
     "scripts/verificationschemagen.sh")
    ("test-manifest" "tests/golden/verification-graph.generated.json"
     "scripts/verificationtestgen.sh")
    ("evidence-json" "docs/generated/verification-evidence.json"
     "scripts/verificationevidencegen.sh")))
