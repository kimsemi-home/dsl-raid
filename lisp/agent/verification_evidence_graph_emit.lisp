(in-package #:dslraid.agent)

(defun write-evidence-graph-node (out row)
  (destructuring-bind (id kind artifact evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"artifact\": \"~A\", " artifact)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-evidence-graph-edges (out)
  (format out "  \"edges\": [~%")
  (loop for row in *verification-evidence-graph-edges* for first = t then nil
        do (unless first (format out ",~%")) (write-evidence-graph-edge out row))
  (format out "~%  ]"))

(defun write-evidence-graph-edge (out row)
  (destructuring-bind (id from to relation evidence status meaning) row
    (format out "    {\"id\": \"~A\", \"from\": \"~A\", " id from)
    (format out "\"to\": \"~A\", \"relation\": \"~A\", " to relation)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-evidence-graph-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-evidence-graph-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationevidencegraphgen.sh check\"}")))
  (format out "~%  ]"))
