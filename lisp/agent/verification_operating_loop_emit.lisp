(in-package #:dslraid.agent)

(defun emit-verification-operating-loop-json (&optional stream)
  "Emit normal operating loop evidence."
  (let ((json (with-output-to-string (out) (write-verification-operating-loop out))))
    (if stream (write-string json stream) json)))

(defun write-verification-operating-loop (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationoperatingloopgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_operating_loop.lisp\",~%")
  (format out "  \"operating_loop_profile\": \"observation-to-knowledge-update\",~%")
  (write-operating-loop-stages out)
  (format out ",~%")
  (write-operating-loop-rules out)
  (format out "~%}~%"))

(defun write-operating-loop-stages (out)
  (format out "  \"stages\": [~%")
  (loop for row in *verification-operating-loop-stages* for first = t then nil
        do (unless first (format out ",~%")) (write-operating-loop-stage out row))
  (format out "~%  ]"))

(defun write-operating-loop-stage (out row)
  (destructuring-bind (order id phase input output command assertion gate evidence meaning) row
    (format out "    {\"order\": ~D, \"id\": \"~A\", \"phase\": \"~A\", " order id phase)
    (format out "\"input\": \"~A\", \"output\": \"~A\", " input output)
    (format out "\"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (format out "\"gate\": \"~A\", " gate)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-operating-loop-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-operating-loop-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationoperatingloopgen.sh check\"}")))
  (format out "~%  ]"))
