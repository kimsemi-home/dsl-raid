(in-package #:dslraid.agent)

(defun write-quarantine-release-gate (out row)
  (destructuring-bind
      (id containment releaser verdict conditions reusable invalid discarded evidence debt meaning)
      row
    (format out "    {\"id\": \"~A\", \"containment\": \"~A\", " id containment)
    (format out "\"released_by\": \"~A\", \"verdict\": \"~A\", " releaser verdict)
    (write-authority-list out "conditions" conditions)
    (format out ", ")
    (write-authority-list out "reusable" reusable)
    (format out ", ")
    (write-authority-list out "invalidated" invalid)
    (format out ", ")
    (write-authority-list out "discarded" discarded)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", ")
    (write-authority-list out "debt" debt)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-quarantine-release-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-quarantine-release-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationquarantinereleasegen.sh check\"}")))
  (format out "~%  ]"))
