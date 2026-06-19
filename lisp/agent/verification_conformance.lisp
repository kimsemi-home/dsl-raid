(in-package #:dslraid.agent)

(defun emit-verification-conformance-json (&optional stream)
  "Emit machine-readable conformance contract for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-conformance out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-conformance (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationconformancegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"rules\": [~%")
  (write-conformance-rules out)
  (format out "~%  ]~%")
  (format out "}~%"))

(defun write-conformance-rules (out)
  (write-conformance-rule out
                          "conformance:quality-gate"
                          "verify:daemon/conformance"
                          "cargo run -p dslraid-cli -- quality"
                          t)
  (loop for row in (verification-backends)
        do (write-conformance-backend-rule out row)))

(defun write-conformance-backend-rule (out row)
  (destructuring-bind (backend output generator) row
    (write-conformance-rule
     out
     (format nil "conformance:backend.~A" backend)
     output
     (format nil "~A check" generator)
     nil)))

(defun write-conformance-rule (out id subject check first)
  (unless first (format out ",~%"))
  (format out "    {\"id\": \"~A\", " id)
  (format out "\"subject\": \"~A\", " subject)
  (format out "\"check\": \"~A\", " check)
  (format out "\"status\": \"required\"}"))
