(in-package #:dslraid.agent)

(defun emit-verification-executable-knowledge-json (&optional stream)
  "Emit executable knowledge evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-executable-knowledge out))))
    (if stream (write-string json stream) json)))

(defun write-verification-executable-knowledge (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationexecutablegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_executable_knowledge.lisp\",~%")
  (format out "  \"executable_knowledge_profile\": \"machine-readable-ssot\",~%")
  (write-executable-knowledge-records out)
  (format out ",~%")
  (write-executable-knowledge-rules out)
  (format out "~%}~%"))

(defun write-executable-knowledge-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-executable-knowledge-records* for first = t then nil
        do (unless first (format out ",~%")) (write-executable-knowledge-record out row))
  (format out "~%  ]"))

(defun write-executable-knowledge-record (out row)
  (destructuring-bind (id kind source generated command assertion evidence gate meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"source\": \"~A\", " source)
    (write-authority-list out "generated" generated)
    (format out ", \"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"gate\": \"~A\", \"meaning\": \"~A\"}" gate meaning)))

(defun write-executable-knowledge-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-executable-knowledge-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationexecutablegen.sh check\"}")))
  (format out "~%  ]"))
