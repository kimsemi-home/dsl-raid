(in-package #:dslraid.agent)

(defparameter *verification-evidence-quality-rules*
  '(("evidence-quality:target-generated" "Every assessed target is generated evidence.")
    ("evidence-quality:assessor-governed" "Assessment is owned by a gate, not an agent.")
    ("evidence-quality:signals-present" "Every assessment names reviewable signals.")))

(defun emit-verification-evidence-quality-json (&optional stream)
  "Emit evidence quality sidecar for verification graph evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-evidence-quality out))))
    (if stream (write-string json stream) json)))

(defun write-verification-evidence-quality (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationevidencequalitygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"evidence_quality_profile\": \"generated-evidence-review\",~%")
  (write-evidence-quality-assessments out)
  (format out ",~%")
  (write-evidence-quality-rules out)
  (format out "~%}~%"))

(defun write-evidence-quality-assessments (out)
  (format out "  \"assessments\": [~%")
  (loop for row in *verification-evidence-quality-assessments*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-evidence-quality-assessment out row))
  (format out "~%  ]"))

(defun write-evidence-quality-assessment (out row)
  (destructuring-bind (id target quality purpose assessor signals meaning) row
    (format out "    {\"id\": \"~A\", \"target\": \"~A\", " id target)
    (format out "\"quality\": \"~A\", \"purpose\": \"~A\", " quality purpose)
    (format out "\"assessed_by\": \"~A\", " assessor)
    (write-authority-list out "signals" signals)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-evidence-quality-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-evidence-quality-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationevidencequalitygen.sh check\"}")))
  (format out "~%  ]"))
