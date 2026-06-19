(in-package #:dslraid.agent)

(defparameter *verification-sidecar-receipts*
  '(("sidecar:generated-workflow" "generator:verification-workflow"
     ".github/workflows/verification.yml" "gate:quality"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-conformance.json")
     "Generated workflow is checked by an independent quality gate.")
    ("sidecar:semantic-hash" "generator:semantic-hash"
     "docs/generated/verification-semantic-hash.json" "gate:semantic-review"
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-evidence-quality.json")
     "Semantic hashes are reviewed through separate diff evidence.")
    ("sidecar:confidence" "generator:confidence"
     "docs/generated/verification-confidence.json" "gate:evidence-quality"
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-authority.json")
     "Confidence ceilings are checked by evidence quality and authority gates.")))

(defparameter *verification-sidecar-rules*
  '(("sidecar:producer-separated" "Producer and final verifier differ.")
    ("sidecar:evidence-generated" "Sidecar evidence is generated evidence.")
    ("sidecar:independent" "Final verification is independent.")))

(defun emit-verification-sidecar-json (&optional stream)
  "Emit sidecar receipts for independent verification evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-sidecar out))))
    (if stream (write-string json stream) json)))

(defun write-verification-sidecar (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationsidecargen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"sidecar_profile\": \"independent-verifier\",~%")
  (write-sidecar-receipts out)
  (format out ",~%")
  (write-sidecar-rules out)
  (format out "~%}~%"))

(defun write-sidecar-receipts (out)
  (format out "  \"receipts\": [~%")
  (loop for row in *verification-sidecar-receipts*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-sidecar-receipt out row))
  (format out "~%  ]"))

(defun write-sidecar-receipt (out row)
  (destructuring-bind (id producer output verifier evidence meaning) row
    (format out "    {\"id\": \"~A\", \"producer\": \"~A\", " id producer)
    (format out "\"output\": \"~A\", \"verifier\": \"~A\", " output verifier)
    (format out "\"independent\": true, ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-sidecar-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-sidecar-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationsidecargen.sh check\"}")))
  (format out "~%  ]"))
