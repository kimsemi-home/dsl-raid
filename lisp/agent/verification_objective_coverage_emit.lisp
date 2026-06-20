(in-package #:dslraid.agent)

(defun emit-verification-objective-coverage-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-objective-coverage out))))
    (if stream (write-string json stream) json)))

(defun write-verification-objective-coverage (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationobjectivegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_objective_coverage.lisp\",~%")
  (format out "  \"objective_coverage_profile\": \"active-goal-trace\",~%")
  (write-objective-coverage-items out)
  (format out ",~%")
  (write-objective-coverage-rules out)
  (format out "~%}~%"))

(defun write-objective-coverage-items (out)
  (format out "  \"requirements\": [~%")
  (loop for row in *verification-objective-coverage* for first = t then nil
        do (unless first (format out ",~%")) (write-objective-coverage-item out row))
  (format out "~%  ]"))

(defun write-objective-coverage-item (out row)
  (destructuring-bind (id kind requirement gate evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"requirement\": \"~A\", \"gate\": \"~A\", " requirement gate)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"tracked\", \"meaning\": \"~A\"}" meaning)))

(defun write-objective-coverage-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-objective-coverage-rules* for first = t then nil
        do (unless first (format out ",~%")) (write-objective-coverage-rule out row))
  (format out "~%  ]"))

(defun write-objective-coverage-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
    (format out "\"check\": \"scripts/verificationobjectivegen.sh check\"}")))
