(in-package #:dslraid.agent)

(defparameter *verification-executable-knowledge-records*
  '(("executable-knowledge:dsl" "dsl" "lisp/runtime/runscope.lisp"
     ("examples/runscope/runscope.lisp.raid.json") "bash scripts/lisp-irgen.sh check"
     "stdout:lisp generated ir ok" ("docs/generated/lisp-pipeline.md")
     "gate:language-expansion" "DSL knowledge must expand to Canonical IR.")
    ("executable-knowledge:specification" "specification"
     "examples/runscope/runscope.assertions.json" ("docs/generated/assertion-catalog.md")
     "bash scripts/assertiongen.sh check" "stdout:assertion generated doc ok"
     ("examples/runscope/runscope.validation.json") "gate:conformance"
     "Specification knowledge must be checked and rendered from assertion data.")
    ("executable-knowledge:ontology" "ontology" "lisp/agent/verification_ontology.lisp"
     ("docs/generated/verification-ontology.json") "bash scripts/verificationontologygen.sh check"
     "stdout:verification ontology generated output ok" ("docs/generated/verification-codegen.json")
     "gate:ontology" "Ontology knowledge is executable when generated deterministically.")
    ("executable-knowledge:policy" "policy" "lisp/agent/verification_access_policy.lisp"
     ("docs/generated/verification-access-policy.json") "bash scripts/verificationaccessgen.sh check"
     "stdout:verification access policy check ok" ("docs/generated/verification-authority.json")
     "gate:access-policy" "Policy knowledge must reject unsafe authority paths.")
    ("executable-knowledge:schema" "schema" "lisp/agent/verification_manifest_schema.lisp"
     ("schemas/dslraid-verification-manifest.schema.json")
     "bash scripts/verificationmanifestschemagen.sh check"
     "stdout:verification manifest schema generated output ok"
     ("docs/generated/verification-conformance.json") "gate:schema"
     "Schema knowledge must be generated from Lisp SSOT.")
    ("executable-knowledge:contract" "contract" "lisp/agent/verification_runtime_contract.lisp"
     ("docs/generated/verification-runtime-contract.json")
     "bash scripts/verificationruntimecontractgen.sh check"
     "stdout:verification runtime contract check ok" ("examples/runscope/run-001.trace.json")
     "gate:runtime-contract" "Contract knowledge must be checked against runtime evidence.")
    ("executable-knowledge:manifest" "manifest" "examples/runscope/runscope.agent-run.json"
     ("docs/generated/agent-run-manifest.md") "bash scripts/agentmanifestgen.sh check"
     "stdout:agent run manifest generated doc ok" ("schemas/dslraid-agent-run.schema.json")
     "gate:run-manifest" "Manifest knowledge must render from schema-valid run data.")
    ("executable-knowledge:ir" "ir" "examples/runscope/runscope.lisp.raid.json"
     ("generated/runtime_fsm.go" "generated/runtime_fsm.rs")
     "cargo run --quiet -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json --lock examples/runscope/runscope.lock.json"
     "stdout:artifact verification passed" ("examples/runscope/runscope.lock.json")
     "gate:artifact-freshness" "IR knowledge must prove generated artifacts are fresh.")
    ("executable-knowledge:verification-rule" "verification-rule"
     "lisp/agent/verification_knowledge_conversion.lisp"
     ("docs/generated/verification-knowledge-conversion.json")
     "bash scripts/verificationknowledgegen.sh check"
     "stdout:verification knowledge conversion check ok"
     ("docs/generated/verification-learning-loop.json") "gate:knowledge-conversion"
     "Verification rules are executable when they run evidence-backed checks.")
    ("executable-knowledge:migration-rule" "migration-rule"
     "lisp/agent/verification_migration_surface.lisp"
     ("docs/generated/verification-migration-surface.json")
     "bash scripts/verificationmigrationgen.sh check"
     "stdout:verification migration surface check ok"
     ("docs/generated/verification-versioned-ssot.json") "gate:migration"
     "Migration knowledge must keep version transitions executable.")
    ("executable-knowledge:translation-manifest" "translation-manifest"
     "lisp/agent/verification_translation_verifier.lisp"
     ("docs/generated/verification-translation-verifier.json")
     "bash scripts/verificationtranslationgen.sh check"
     "stdout:verification translation verifier check ok"
     ("docs/generated/verification-loss-ledger.json") "gate:translation"
     "Translation knowledge must prove loss policy through generated evidence.")
    ("executable-knowledge:evidence-policy" "evidence-policy"
     "lisp/agent/verification_evidence_quality.lisp"
     ("docs/generated/verification-evidence-quality.json")
     "bash scripts/verificationevidencequalitygen.sh check"
     "stdout:verification evidence quality check ok"
     ("docs/generated/verification-evidence.json") "gate:evidence-quality"
     "Evidence policy must be machine-checked before confidence increases.")))

(defparameter *verification-executable-knowledge-rules*
  '(("executable-knowledge:no-prose-ssot" "Human prose explains but never owns SSOT.")
    ("executable-knowledge:command-backed" "Every knowledge record has a passing command.")
    ("executable-knowledge:artifact-linked" "Executable knowledge generates or checks artifacts.")))
