(in-package #:dslraid.agent)

(defun emit-verification-semantic-os-json (&optional stream)
  "Emit semantic operating system layer evidence."
  (let ((json (with-output-to-string (out) (write-verification-semantic-os out))))
    (if stream (write-string json stream) json)))

(defun write-verification-semantic-os (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationsemanticosgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_semantic_os.lisp\",~%")
  (format out "  \"semantic_os_profile\": \"meaning-operating-system\",~%")
  (write-semantic-os-layers out)
  (format out ",~%")
  (write-semantic-os-rules out)
  (format out "~%}~%"))

(defun write-semantic-os-layers (out)
  (format out "  \"layers\": [~%")
  (loop for row in *verification-semantic-os-layers* for first = t then nil
        do (unless first (format out ",~%")) (write-semantic-os-layer out row))
  (format out "~%  ]"))

(defun write-semantic-os-layer (out row)
  (destructuring-bind (id role layer source artifact command assertion evidence authority &optional meaning) row
    (format out "    {\"id\": \"~A\", \"role\": \"~A\", \"os_layer\": \"~A\", " id role layer)
    (format out "\"source\": \"~A\", \"artifact\": \"~A\", " source artifact)
    (format out "\"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"authority\": \"~A\", \"meaning\": \"~A\"}" authority (or meaning layer))))

(defun write-semantic-os-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-semantic-os-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationsemanticosgen.sh check\"}")))
  (format out "~%  ]"))
