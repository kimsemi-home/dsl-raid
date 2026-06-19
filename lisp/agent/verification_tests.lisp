(in-package #:dslraid.agent)

(defun emit-verification-test-manifest-json (&optional stream)
  "Emit generated golden-test manifest for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-test-manifest out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-test-manifest (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationtestgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"assertions\": [~%")
  (write-test-node-assertions out t)
  (write-test-backend-assertions out nil)
  (format out "~%  ]~%")
  (format out "}~%"))

(defun write-test-node-assertions (out first)
  (loop for node in (verification-nodes)
        for initial = first then nil
        do (unless initial (format out ",~%"))
           (write-test-node-assertion out node)))

(defun write-test-node-assertion (out node)
  (let ((id (verification-id node)))
    (format out "    {\"id\": \"test:verification.node.~A\", " id)
    (format out "\"subject\": \"~A/~A\", " (getf (verification-graph) :id) id)
    (format out "\"kind\": \"verification-node\", ")
    (format out "\"commands\": ~D}" (length (verification-field node :commands)))))

(defun write-test-backend-assertions (out first)
  (loop for row in (verification-backends)
        for initial = first then nil
        do (unless initial (format out ",~%"))
           (write-test-backend-assertion out row)))

(defun write-test-backend-assertion (out row)
  (destructuring-bind (backend output generator) row
    (format out "    {\"id\": \"test:verification.backend.~A\", " backend)
    (format out "\"subject\": \"~A\", " output)
    (format out "\"kind\": \"generated-backend\", ")
    (format out "\"generator\": \"~A\"}" generator)))
