(in-package #:dslraid.agent)

(defparameter *verification-semantic-hashes*
  '(("semantic:verification-graph" "docs/generated/verification-evidence.json"
     ("form" "ontology_chain" "verification_nodes" "generated_backends")
     "Verification graph shape and generated backend contract.")
    ("semantic:codegen-map" "docs/generated/verification-codegen.json"
     ("axes")
     "Ontology codegen axes mapped to generated backends.")
    ("semantic:loss-ledger" "docs/generated/verification-loss-ledger.json"
     ("ledger")
     "Declared adapter translation loss without forbidden L4 loss.")
    ("semantic:conformance" "docs/generated/verification-conformance.json"
     ("rules")
     "Required checks for generated backend freshness.")
    ("semantic:authority" "docs/generated/verification-authority.json"
     ("authority_profile" "decisions" "closure_rules")
     "Governance authority gate and review decision contract.")))

(defparameter *verification-semantic-rules*
  '(("semantic:hash-recomputes" "Every semantic hash recomputes from source fields.")
    ("semantic:source-exists" "Every semantic hash source is generated evidence.")
    ("semantic:stable-canonical-json" "Hash input uses sorted canonical JSON.")))

(defun emit-verification-semantic-json (&optional stream)
  "Emit semantic hash inputs before sidecar materializes digests."
  (let ((json (with-output-to-string (out)
                (write-verification-semantic out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-semantic (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationsemanticgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"algorithm\": \"sha256\",~%")
  (write-semantic-hashes out)
  (format out ",~%")
  (write-semantic-rules out)
  (format out "~%}~%"))

(defun write-semantic-hashes (out)
  (format out "  \"hashes\": [~%")
  (loop for row in *verification-semantic-hashes*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-semantic-entry out row))
  (format out "~%  ]"))

(defun write-semantic-entry (out row)
  (destructuring-bind (id source fields meaning) row
    (format out "    {\"id\": \"~A\", \"source\": \"~A\", \"fields\": [" id source)
    (loop for field in fields for first = t then nil
          do (unless first (format out ", ")) (format out "\"~A\"" field))
    (format out "], \"meaning\": \"~A\"}" meaning)))

(defun write-semantic-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-semantic-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationsemanticgen.sh check\"}")))
  (format out "~%  ]"))
