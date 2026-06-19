(in-package #:dslraid.agent)

(defun write-review-overload-rules (out)
  (format out "  \"overload_rules\": [~%")
  (loop for row in *verification-review-overload-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id when effect meaning) row
             (format out "    {\"id\": \"~A\", \"when\": \"~A\", " id when)
             (format out "\"effect\": \"~A\", \"meaning\": \"~A\"}" effect meaning)))
  (format out "~%  ]"))

(defun write-review-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-review-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationreviewgen.sh check\"}")))
  (format out "~%  ]"))
