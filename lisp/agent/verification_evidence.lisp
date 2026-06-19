(in-package #:dslraid.agent)

(defun emit-verification-evidence-json (&optional stream)
  "Emit machine-readable evidence for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-evidence out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-evidence (out)
  (format out "{~%")
  (format out "  \"$schema\": \"schemas/dslraid-verification-evidence.schema.json\",~%")
  (format out "  \"schema_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationevidencegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"ssot\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"form\": \"~A\",~%" (getf (verification-graph) :form))
  (format out "  \"ontology_chain\": [~%")
  (write-json-items out '("ontology" "executable-ssot" "verification-graph" "codegen") 4)
  (format out "  ],~%")
  (write-evidence-backends out)
  (format out ",~%")
  (write-evidence-checks out)
  (format out ",~%")
  (write-evidence-pdca out)
  (format out "~%}~%"))

(defun write-evidence-backends (out)
  (format out "  \"generated_backends\": [~%")
  (loop for row in (verification-backends)
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-evidence-backend out row))
  (format out "~%  ]"))

(defun write-evidence-backend (out row)
  (destructuring-bind (backend output generator) row
    (format out "    {\"backend\": \"~A\", \"output\": \"~A\", " backend output)
    (format out "\"generator\": \"~A\", \"check\": \"~A check\"}" generator generator)))

(defun write-evidence-checks (out)
  (format out "  \"verification_nodes\": [~%")
  (loop for node in (verification-nodes)
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-evidence-node out node))
  (format out "~%  ]"))

(defun write-evidence-node (out node)
  (format out "    {\"id\": \"~A\", \"commands\": ~D, \"evidence\": \"~A\"}"
          (verification-id node)
          (length (verification-field node :commands))
          (verification-field node :evidence)))

(defun write-evidence-pdca (out)
  (format out "  \"pdca\": [~%")
  (write-json-items out '("plan" "do" "check" "act") 4)
  (format out "  ]"))

(defun write-json-items (out items indent)
  (loop for item in items
        for first = t then nil
        do (unless first (format out ",~%"))
           (format out "~VT\"~A\"" indent item))
  (terpri out))
