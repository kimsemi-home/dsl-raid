(in-package #:dslraid.agent)

(defun write-lease-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-lease-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationleasegen.sh check\"}")))
  (format out "~%  ]"))
