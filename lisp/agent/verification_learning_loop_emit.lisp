(in-package #:dslraid.agent)

(defun emit-verification-learning-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-learning out))))
    (if stream (write-string json stream) json)))

(defun write-verification-learning (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationlearninggen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_learning_loop.lisp\",~%")
  (format out "  \"learning_loop_profile\": \"reality-to-revalidation\",~%")
  (write-learning-stages out) (format out ",~%")
  (write-learning-cycles out) (format out ",~%")
  (write-learning-rules out) (format out "~%}~%"))

(defun write-learning-stages (out)
  (format out "  \"stages\": [~%")
  (loop for row in *verification-learning-stages* for first = t then nil
        do (unless first (format out ",~%")) (write-learning-stage out row))
  (format out "~%  ]"))

(defun write-learning-stage (out row)
  (destructuring-bind (id order from to evidence meaning) row
    (format out "    {\"id\": \"~A\", \"order\": ~D, " id order)
    (format out "\"from\": \"~A\", \"to\": \"~A\", " from to)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-learning-cycles (out)
  (format out "  \"cycles\": [~%")
  (loop for row in *verification-learning-cycles* for first = t then nil
        do (unless first (format out ",~%")) (write-learning-cycle out row))
  (format out "~%  ]"))

(defun write-learning-cycle (out row)
  (destructuring-bind (id trigger incident stages evidence owner update reval status meaning) row
    (format out "    {\"id\": \"~A\", \"trigger\": \"~A\", " id trigger)
    (format out "\"incident\": \"~A\", " incident)
    (write-authority-list out "stages" stages) (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"owner\": \"~A\", " owner)
    (format out "\"knowledge_update\": \"~A\", " update)
    (format out "\"revalidation\": \"~A\", " reval)
    (format out "\"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-learning-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-learning-rules* for first = t then nil
        do (unless first (format out ",~%")) (write-learning-rule out row))
  (format out "~%  ]"))

(defun write-learning-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
    (format out "\"check\": \"scripts/verificationlearninggen.sh check\"}")))
