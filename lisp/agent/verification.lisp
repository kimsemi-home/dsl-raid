(in-package #:dslraid.agent)

(defparameter *verification-graph*
  `(:id "verify:daemon"
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
      :commands ,(verification-release-check-commands)))))
