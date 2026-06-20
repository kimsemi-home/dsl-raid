(in-package #:dslraid.agent)

(defparameter *verification-runtime-contract-surfaces*
  '(("runtime-contract:canonical-ir" "canonical-ir"
     "cargo run --quiet -p dslraid-cli -- validate examples/runscope/runscope.lisp.raid.json --schema schemas/dslraid-core.schema.json --format text --deny warning"
     "examples/runscope/runscope.lisp.raid.json" "stdout:validation passed"
     ("schemas/dslraid-core.schema.json")
     "Lisp-expanded Canonical IR must satisfy the Rust validator.")
    ("runtime-contract:artifact-freshness" "artifact-freshness"
     "cargo run --quiet -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json --lock examples/runscope/runscope.lock.json"
     "examples/runscope/runscope.lock.json" "stdout:artifact verification passed"
     ("generated/runtime_fsm.rs" "generated/runtime_fsm.go")
     "Generated runtime artifacts must match the lock hash contract.")
    ("runtime-contract:source-map" "source-map"
     "cargo run --quiet -p dslraid-cli -- schema validate schemas/dslraid-sourcemap.schema.json examples/runscope/runscope.sourcemap.json"
     "examples/runscope/runscope.sourcemap.json" "stdout:schema ok"
     ("schemas/dslraid-sourcemap.schema.json")
     "Runtime artifacts must remain traceable to design subjects.")
    ("runtime-contract:rust-compile" "rust-compile"
     "cargo run --quiet -p dslraid-cli -- codegen examples/runscope/runscope.lisp.raid.json --target rust | rustc --edition=2021 --crate-type=lib - -o /tmp/dslraid-runtime-contract.rlib && rm -f /tmp/dslraid-runtime-contract.rlib && echo rust codegen compile ok"
     "examples/runscope/runscope.lisp.raid.json" "stdout:rust codegen compile ok"
     ("crates/dslraid-codegen/src/code/rust.rs")
     "Generated Rust runtime code must compile from Canonical IR.")))

(defparameter *verification-runtime-contract-rules*
  '(("runtime-contract:ir" "Expanded IR must validate before runtime use.")
    ("runtime-contract:artifact" "Runtime artifacts must be fresh.")
    ("runtime-contract:sourcemap" "Generated runtime code must be traceable.")
    ("runtime-contract:compile" "Rust runtime output must compile.")))

(defun emit-verification-runtime-contract-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-runtime-contract out))))
    (if stream (write-string json stream) json)))

(defun write-verification-runtime-contract (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationruntimecontractgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_runtime_contract.lisp\",~%")
  (format out "  \"runtime_contract_profile\": \"canonical-ir-to-runtime-artifact\",~%")
  (write-runtime-contract-surfaces out)
  (format out ",~%")
  (write-runtime-contract-rules out)
  (format out "~%}~%"))

(defun write-runtime-contract-surfaces (out)
  (format out "  \"surfaces\": [~%")
  (loop for row in *verification-runtime-contract-surfaces* for first = t then nil
        do (unless first (format out ",~%"))
           (write-runtime-contract-surface out row))
  (format out "~%  ]"))

(defun write-runtime-contract-surface (out row)
  (destructuring-bind (id kind command fixture assertion evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"fixture\": \"~A\", " command fixture)
    (format out "\"assertion\": \"~A\", " assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"checked\", \"meaning\": \"~A\"}" meaning)))

(defun write-runtime-contract-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-runtime-contract-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationruntimecontractgen.sh check\"}")))
  (format out "~%  ]"))
