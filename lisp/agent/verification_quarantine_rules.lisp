(in-package #:dslraid.agent)

(defun write-quarantine-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-quarantine-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationquarantinegen.sh check\"}")))
  (format out "~%  ]"))
