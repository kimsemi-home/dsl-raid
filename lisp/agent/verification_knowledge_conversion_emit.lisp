(in-package #:dslraid.agent)

(defun emit-verification-knowledge-conversion-json (&optional stream)
  "Emit error-to-knowledge conversion evidence."
  (let ((json (with-output-to-string (out) (write-verification-knowledge-conversion out))))
    (if stream (write-string json stream) json)))

(defun write-verification-knowledge-conversion (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationknowledgegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_knowledge_conversion.lisp\",~%")
  (format out "  \"knowledge_conversion_profile\": \"error-to-knowledge\",~%")
  (write-knowledge-conversion-steps out)
  (format out ",~%")
  (write-knowledge-conversion-rules out)
  (format out "~%}~%"))

(defun write-knowledge-conversion-steps (out)
  (format out "  \"steps\": [~%")
  (loop for row in *verification-knowledge-conversion-steps* for first = t then nil
        do (unless first (format out ",~%")) (write-knowledge-conversion-step out row))
  (format out "~%  ]"))

(defun write-knowledge-conversion-step (out row)
  (destructuring-bind (order id phase input output command assertion gate evidence meaning) row
    (format out "    {\"order\": ~D, \"id\": \"~A\", \"phase\": \"~A\", " order id phase)
    (format out "\"input\": \"~A\", \"output\": \"~A\", " input output)
    (format out "\"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (format out "\"gate\": \"~A\", " gate)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-knowledge-conversion-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-knowledge-conversion-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationknowledgegen.sh check\"}")))
  (format out "~%  ]"))
