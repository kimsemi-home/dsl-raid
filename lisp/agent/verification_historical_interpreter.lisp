(in-package #:dslraid.agent)

(defparameter *verification-historical-interpretations*
  '(("historical:workflow-evidence" "docs/generated/verification-evidence.json"
     "ontology:verification.0.1.0" "ontology:github-actions.0.1.0"
     "context-map:lisp-to-github-actions" "keep-original"
     ("docs/generated/verification-context-map.json"
      "docs/generated/verification-semantic-diff.json")
     "Generated workflow evidence is reinterpreted through a bridge.")
    ("historical:schema-evidence" "docs/generated/verification-evidence.json"
     "ontology:verification.0.1.0" "ontology:manifest-contract.0.1.0"
     "context-map:lisp-to-manifest-schema" "keep-original"
     ("docs/generated/verification-context-map.json"
      "docs/generated/verification-versioned-ssot.json")
     "Schema evidence is not rewritten under the target ontology.")))

(defparameter *verification-historical-rules*
  '(("historical:no-silent-rewrite" "Old evidence keeps its original ontology.")
    ("historical:bridge-required" "Reinterpretation uses a context-map bridge.")
    ("historical:evidence-linked" "Interpreter decisions link generated evidence.")))

(defun emit-verification-historical-json (&optional stream)
  "Emit historical interpretation receipts for versioned evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-historical out))))
    (if stream (write-string json stream) json)))

(defun write-verification-historical (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationhistoricalgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"historical_interpreter_profile\": \"bridge-not-rewrite\",~%")
  (write-historical-interpretations out)
  (format out ",~%")
  (write-historical-rules out)
  (format out "~%}~%"))

(defun write-historical-interpretations (out)
  (format out "  \"interpretations\": [~%")
  (loop for row in *verification-historical-interpretations*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-historical-interpretation out row))
  (format out "~%  ]"))

(defun write-historical-interpretation (out row)
  (destructuring-bind (id evidence old new bridge policy support meaning) row
    (format out "    {\"id\": \"~A\", \"evidence\": \"~A\", " id evidence)
    (format out "\"interpreted_under\": \"~A\", " old)
    (format out "\"reinterpreted_under\": \"~A\", " new)
    (format out "\"translation_edge\": \"~A\", " bridge)
    (format out "\"policy\": \"~A\", " policy)
    (write-authority-list out "supporting_evidence" support)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-historical-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-historical-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationhistoricalgen.sh check\"}")))
  (format out "~%  ]"))
