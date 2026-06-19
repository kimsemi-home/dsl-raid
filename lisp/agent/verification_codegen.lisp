(in-package #:dslraid.agent)

(defun emit-verification-codegen-json (&optional stream)
  "Emit machine-readable codegen axis coverage for verification."
  (let ((json (with-output-to-string (out)
                (write-verification-codegen out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-codegen (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationcodegengen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (write-codegen-coverage out)
  (format out ",~%")
  (write-codegen-rules out)
  (format out "~%}~%"))

(defun write-codegen-coverage (out)
  (format out "  \"axes\": [~%")
  (loop for row in *verification-codegen-map*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-codegen-axis out row))
  (format out "~%  ]"))

(defun write-codegen-axis (out row)
  (destructuring-bind (axis backends) row
    (format out "    {\"axis\": \"~A\", \"backends\": [" axis)
    (loop for backend in backends
          for first = t then nil
          do (unless first (format out ", "))
             (format out "\"~A\"" backend))
    (format out "]}")))

(defun write-codegen-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-codegen-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationcodegengen.sh check\"}")))
  (format out "~%  ]"))
