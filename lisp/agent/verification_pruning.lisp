(in-package #:dslraid.agent)

(defparameter *verification-pruning-decisions*
  '(("pruning:semantic-hash-retention" "docs/generated/verification-semantic-hash.json"
     "retain" "protected" ("approved-semantic-hash" "audit-evidence")
     "tombstone:not-allowed" "gate:authority"
     ("docs/generated/verification-semantic-hash.json"
      "docs/generated/verification-authority.json")
     "Approved semantic evidence is retained, never pruned.")
    ("pruning:debugger-refresh" "docs/generated/verification-semantic-debugger.json"
     "replace-with-tombstone" "candidate" ("generated-evidence")
     "tombstone:semantic-debugger-refresh" "gate:authority"
     ("docs/generated/verification-semantic-debugger.json"
      "docs/generated/verification-evidence.json")
     "Generated evidence may refresh only with tombstone and authority.")))

(defparameter *verification-pruning-rules*
  '(("pruning:protected-retained" "Protected audit evidence is retained.")
    ("pruning:tombstone-required" "Replacement or deletion keeps a tombstone.")
    ("pruning:authority-required" "Evidence pruning is governance-approved.")))

(defun emit-verification-pruning-json (&optional stream)
  "Emit evidence pruning governance decisions."
  (let ((json (with-output-to-string (out)
                (write-verification-pruning out))))
    (if stream (write-string json stream) json)))

(defun write-verification-pruning (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationpruninggen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"evidence_pruning_profile\": \"governed-retention\",~%")
  (write-pruning-decisions out)
  (format out ",~%")
  (write-pruning-rules out)
  (format out "~%}~%"))

(defun write-pruning-decisions (out)
  (format out "  \"decisions\": [~%")
  (loop for row in *verification-pruning-decisions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-pruning-decision out row))
  (format out "~%  ]"))

(defun write-pruning-decision (out row)
  (destructuring-bind (id target action status reasons tombstone authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"target\": \"~A\", " id target)
    (format out "\"action\": \"~A\", \"status\": \"~A\", " action status)
    (write-authority-list out "immutable_reasons" reasons)
    (format out ", \"tombstone\": \"~A\", \"authority\": \"~A\", " tombstone authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-pruning-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-pruning-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationpruninggen.sh check\"}")))
  (format out "~%  ]"))
