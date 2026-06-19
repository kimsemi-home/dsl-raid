(in-package #:dslraid.agent)

(defparameter *verification-semantic-diffs*
  '(("semantic-diff:verification-graph" "semantic:verification-graph"
     "Verification graph semantic receipt.")
    ("semantic-diff:codegen-map" "semantic:codegen-map"
     "Codegen axis semantic receipt.")
    ("semantic-diff:loss-ledger" "semantic:loss-ledger"
     "Translation loss semantic receipt.")
    ("semantic-diff:conformance" "semantic:conformance"
     "Conformance contract semantic receipt.")
    ("semantic-diff:authority" "semantic:authority"
     "Authority gate semantic receipt.")
    ("semantic-diff:evidence-quality" "semantic:evidence-quality"
     "Evidence quality semantic receipt.")))

(defparameter *verification-diff-rules*
  '(("diff:hash-backed" "Every semantic diff references a semantic hash.")
    ("diff:status-derived" "Diff status is derived from base and head hashes.")
    ("diff:evidence-linked" "Every semantic diff links generated evidence.")))

(defun emit-verification-diff-json (&optional stream)
  "Emit semantic diff inputs before sidecar materializes hashes."
  (let ((json (with-output-to-string (out)
                (write-verification-diff out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-diff (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationdiffgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"base\": \"docs/generated/verification-semantic-hash.json\",~%")
  (format out "  \"head\": \"docs/generated/verification-semantic-hash.json\",~%")
  (write-diff-entries out)
  (format out ",~%")
  (write-diff-rules out)
  (format out "~%}~%"))

(defun write-diff-entries (out)
  (format out "  \"diffs\": [~%")
  (loop for row in *verification-semantic-diffs*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-diff-entry out row))
  (format out "~%  ]"))

(defun write-diff-entry (out row)
  (destructuring-bind (id hash-id summary) row
    (format out "    {\"id\": \"~A\", \"hash_id\": \"~A\", " id hash-id)
    (format out "\"summary\": \"~A\"}" summary)))

(defun write-diff-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-diff-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationdiffgen.sh check\"}")))
  (format out "~%  ]"))
