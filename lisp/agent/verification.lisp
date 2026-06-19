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
       "bash scripts/verificationlossgen.sh check"
       "bash scripts/verificationontologygen.sh check"
       "bash scripts/verificationconformancegen.sh check"
       "bash scripts/verificationschemagen.sh check"
       "bash scripts/verificationmanifestschemagen.sh check"
       "bash scripts/verificationtestgen.sh check"
       "bash scripts/verificationevidencegen.sh check"
       "bash scripts/verificationsemanticgen.sh check"
       "bash scripts/verificationdiffgen.sh check"
       "bash scripts/verificationauthoritygen.sh check"
       "bash scripts/verificationevidencequalitygen.sh check"
       "bash scripts/verificationleasegen.sh check"
       "bash scripts/verificationreviewgen.sh check"
       "bash scripts/verificationfeedbackgen.sh check"
       "bash scripts/verificationquarantinegen.sh check"
       "bash scripts/verificationconfidencegen.sh check"
       "bash scripts/verificationsidecargen.sh check"
       "bash scripts/verificationorchestrationgen.sh check"
       "bash scripts/verificationevidencebeforechangegen.sh check"
       "bash scripts/verificationversionedssotgen.sh check"
       "bash scripts/verificationcontextmapgen.sh check"
       "bash scripts/verificationhistoricalgen.sh check"
       "bash scripts/verificationtransitiongen.sh check"
       "bash scripts/verificationssotdefectgen.sh check"
       "bash scripts/verificationrootcausegen.sh check"
       "bash scripts/verificationdebuggergen.sh check"
       "bash scripts/verificationpruninggen.sh check"
       "bash scripts/lisp-rustgen.sh check"
       "bash scripts/verificationdocgen.sh check"
       "bash scripts/verificationcodegengen.sh check"
       "bash scripts/gendocindex.sh check"
       "cargo run -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json")))))
