(in-package #:dslraid.agent)

(defparameter *verification-semantic-os-layers*
  '(("semantic-os:firmware" "firmware" "Common Lisp DSL"
     "lisp/agent/verification_language_expansion.lisp"
     "docs/generated/verification-language-expansion.json"
     "scripts/verificationlanguagegen.sh check"
     "stdout:verification language expansion check ok"
     ("docs/generated/lisp-pipeline.md") "gate:language-expansion"
     "Common Lisp forms are the firmware for executable meaning.")
    ("semantic-os:kernel" "kernel" "Ontology"
     "lisp/agent/verification_ontology.lisp" "docs/generated/verification-ontology.json"
     "scripts/verificationontologygen.sh check"
     "stdout:verification ontology generated output ok"
     ("docs/generated/verification-ontology.json") "gate:ontology"
     "Ontology supplies the kernel meaning space.")
    ("semantic-os:filesystem" "filesystem" "Versioned SSOT"
     "lisp/agent/verification_versioned_ssot.lisp"
     "docs/generated/verification-versioned-ssot.json"
     "scripts/verificationversionedssotgen.sh check"
     "stdout:verification versioned ssot check ok"
     ("docs/generated/verification-versioned-ssot.json") "gate:ssot")
    ("semantic-os:userland" "userland" "Go generated runtime"
     "examples/runscope/runscope.lisp.raid.json" "generated/runtime_fsm.go"
     "cargo run --quiet -p dslraid-cli -- codegen examples/runscope/runscope.lisp.raid.json --target go"
     "stdout:package generated" ("generated/runtime_fsm.go") "gate:runtime")
    ("semantic-os:driver" "driver" "Rust/WASM sidecar"
     "lisp/agent/verification_sidecar.lisp" "docs/generated/verification-sidecar.json"
     "scripts/verificationsidecargen.sh check" "stdout:verification sidecar check ok"
     ("docs/generated/verification-sidecar.json") "gate:sidecar")
    ("semantic-os:log" "log" "Evidence Graph"
     "lisp/agent/verification_evidence.lisp" "docs/generated/verification-evidence.json"
     "scripts/verificationevidencegen.sh check"
     "stdout:verification evidence generated output ok"
     ("docs/generated/verification-evidence.json") "gate:evidence")
    ("semantic-os:scheduler" "scheduler" "Control Plane"
     "lisp/agent/verification_orchestration.lisp"
     "docs/generated/verification-orchestration.json"
     "scripts/verificationorchestrationgen.sh check"
     "stdout:verification orchestration check ok"
     ("docs/generated/verification-control-plane.json") "gate:control-plane")
    ("semantic-os:court" "court" "Governance"
     "lisp/agent/verification_authority.lisp" "docs/generated/verification-authority.json"
     "bash scripts/verificationauthoritygen.sh check" "stdout:verification authority check ok"
     ("docs/generated/verification-authority.json") "gate:authority")))

(defparameter *verification-semantic-os-rules*
  '(("semantic-os:file-backed" "Every operating layer is backed by a file artifact.")
    ("semantic-os:authority-gated" "Self improvement still passes evidence and authority gates.")
    ("semantic-os:generated-runtime" "Userland runtime is generated from Canonical IR.")))
