(in-package #:dslraid.agent)

(defparameter *verification-authority-decisions*
  '(("authority:quality-gate" "routine" "approved" "gate:quality"
     ("verification:quality" "semantic:diff")
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-semantic-diff.json")
     "Agents produce claims; quality gate owns merge authority.")
    ("authority:release-check" "release" "approved" "gate:release"
     ("verification:freshness" "evidence:generated")
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-codegen.json")
     "Generated outputs must be fresh before release authority applies.")))

(defparameter *verification-authority-rules*
  '(("authority:no-agent-self-approval" "Agents cannot approve their own outputs.")
    ("authority:evidence-linked" "Every authority decision links generated evidence.")
    ("authority:public-safe" "Authority manifest contains public paths only.")))

(defun emit-verification-authority-json (&optional stream)
  "Emit authority sidecar for verification graph decisions."
  (let ((json (with-output-to-string (out)
                (write-verification-authority out))))
    (if stream (write-string json stream) json)))

(defun write-verification-authority (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationauthoritygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"authority_profile\": \"governance-sidecar\",~%")
  (write-authority-decisions out)
  (format out ",~%")
  (write-authority-rules out)
  (format out "~%}~%"))

(defun write-authority-decisions (out)
  (format out "  \"decisions\": [~%")
  (loop for row in *verification-authority-decisions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-authority-decision out row))
  (format out "~%  ]"))

(defun write-authority-decision (out row)
  (destructuring-bind (id scope decision approver requires evidence meaning) row
    (format out "    {\"id\": \"~A\", \"scope\": \"~A\", " id scope)
    (format out "\"decision\": \"~A\", \"approved_by\": \"~A\", " decision approver)
    (write-authority-list out "requires" requires)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-authority-list (out key values)
  (format out "\"~A\": [" key)
  (loop for value in values
        for first = t then nil
        do (unless first (format out ", "))
           (format out "\"~A\"" value))
  (format out "]"))

(defun write-authority-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-authority-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationauthoritygen.sh check\"}")))
  (format out "~%  ]"))
