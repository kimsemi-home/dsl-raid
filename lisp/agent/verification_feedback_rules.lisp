(in-package #:dslraid.agent)

(defun write-feedback-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-feedback-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationfeedbackgen.sh check\"}")))
  (format out "~%  ]"))
