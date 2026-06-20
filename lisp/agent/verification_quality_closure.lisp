(in-package #:dslraid.agent)

(defparameter *verification-quality-closure-rules*
  '(("quality-closure:backend-covered"
     "Every generated backend generator is referenced by the quality gate.")
    ("quality-closure:evidence-matches"
     "Quality closure rows mirror generated backend evidence.")
    ("quality-closure:private-safe"
     "Quality closure evidence does not expose local private paths.")))

(defun emit-verification-quality-closure-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-quality-closure out))))
    (if stream (write-string json stream) json)))

(defun write-verification-quality-closure (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationqualitygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_quality_closure.lisp\",~%")
  (format out "  \"quality_closure_profile\": \"generated-backend-gate\",~%")
  (format out "  \"quality_gate\": \"cargo run -p dslraid-cli -- quality\",~%")
  (format out "  \"registry_source\": \"lisp/agent/verification_backends.lisp\",~%")
  (write-quality-closure-generators out)
  (format out ",~%")
  (write-quality-closure-rules out)
  (format out "~%}~%"))

(defun write-quality-closure-generators (out)
  (format out "  \"enforced_generators\": [~%")
  (loop for row in (verification-backends)
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-quality-closure-generator out row))
  (format out "~%  ]"))

(defun write-quality-closure-generator (out row)
  (destructuring-bind (backend output generator) row
    (format out "    {\"backend\": \"~A\", \"output\": \"~A\", " backend output)
    (format out "\"generator\": \"~A\", " generator)
    (format out "\"quality_gate\": \"dslraid-cli quality\", ")
    (format out "\"evidence\": \"docs/generated/verification-evidence.json\"}")))

(defun write-quality-closure-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-quality-closure-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationqualitygen.sh check\"}")))
  (format out "~%  ]"))
