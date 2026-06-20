(in-package #:dslraid.agent)

(defparameter *verification-semantic-foundation-hashes*
  '(("semantic:verification-graph" "docs/generated/verification-evidence.json" ("form" "ontology_chain" "verification_nodes" "generated_backends") "Verification graph shape and generated backend contract.")
    ("semantic:codegen-map" "docs/generated/verification-codegen.json" ("axes") "Ontology codegen axes mapped to generated backends.")
    ("semantic:loss-ledger" "docs/generated/verification-loss-ledger.json" ("ledger") "Declared adapter translation loss without forbidden L4 loss.")
    ("semantic:conformance" "docs/generated/verification-conformance.json" ("rules") "Required checks for generated backend freshness.")))
