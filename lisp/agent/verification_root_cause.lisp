(in-package #:dslraid.agent)

(defparameter *verification-root-cause-cases*
  '(("root-cause:ssot-defect-drill" "observation:ssot-defect-drill"
     "candidate-set"
     ("cause:contract-drift" "cause:generation-drift" "cause:evidence-drift")
     ("cause:evidence-drift")
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-conformance.json")
     "medium" "gate:authority"
     ("docs/generated/verification-ssot-defect.json"
      "docs/generated/verification-evidence-quality.json")
     "Root cause remains a bounded hypothesis until validation evidence closes it.")))

(defparameter *verification-root-cause-rules*
  '(("root-cause:candidates-first" "Root cause starts as candidates, not assertion.")
    ("root-cause:validation-required" "Validation evidence narrows candidates.")
    ("root-cause:confidence-capped" "Unconfirmed root cause cannot be high confidence.")))

(defun emit-verification-root-cause-json (&optional stream)
  "Emit root cause candidate and validation receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-root-cause out))))
    (if stream (write-string json stream) json)))

(defun write-verification-root-cause (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationrootcausegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"root_cause_profile\": \"candidate-validation\",~%")
  (write-root-cause-cases out)
  (format out ",~%")
  (write-root-cause-rules out)
  (format out "~%}~%"))

(defun write-root-cause-cases (out)
  (format out "  \"cases\": [~%")
  (loop for row in *verification-root-cause-cases*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-root-cause-case out row))
  (format out "~%  ]"))

(defun write-root-cause-case (out row)
  (destructuring-bind (id observation status candidates eliminated validation confidence authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"observation\": \"~A\", " id observation)
    (format out "\"status\": \"~A\", " status)
    (write-authority-list out "candidates" candidates)
    (format out ", ")
    (write-authority-list out "eliminated_causes" eliminated)
    (format out ", ")
    (write-authority-list out "validation_evidence" validation)
    (format out ", \"confidence_ceiling\": \"~A\", " confidence)
    (format out "\"authority\": \"~A\", " authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-root-cause-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-root-cause-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationrootcausegen.sh check\"}")))
  (format out "~%  ]"))
