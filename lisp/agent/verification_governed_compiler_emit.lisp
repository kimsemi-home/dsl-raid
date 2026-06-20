(in-package #:dslraid.agent)

(defun emit-verification-governed-compiler-json (&optional stream)
  "Emit governed compiler farm evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-governed-compiler out))))
    (if stream (write-string json stream) json)))

(defun write-verification-governed-compiler (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationcompilergen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_governed_compiler.lisp\",~%")
  (format out "  \"compiler_farm_profile\": \"spec-candidate-evidence-authority\",~%")
  (write-governed-compiler-stages out)
  (format out ",~%")
  (write-governed-compiler-rules out)
  (format out "~%}~%"))

(defun write-governed-compiler-stages (out)
  (format out "  \"stages\": [~%")
  (loop for row in *verification-governed-compiler-stages* for first = t then nil
        do (unless first (format out ",~%")) (write-governed-compiler-stage out row))
  (format out "~%  ]"))

(defun write-governed-compiler-stage (out row)
  (destructuring-bind (order id stage input output command assertion trust evidence meaning) row
    (format out "    {\"order\": ~D, \"id\": \"~A\", \"stage\": \"~A\", " order id stage)
    (format out "\"input\": \"~A\", \"output\": \"~A\", " input output)
    (format out "\"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (format out "\"trust\": \"~A\", " trust)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-governed-compiler-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-governed-compiler-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationcompilergen.sh check\"}")))
  (format out "~%  ]"))
