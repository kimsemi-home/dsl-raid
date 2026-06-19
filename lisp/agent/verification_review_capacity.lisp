(in-package #:dslraid.agent)

(defparameter *verification-review-queues*
  '(("review-capacity:verification" "routine" 4 2 "available"
     ("authority-gate" "lease-and-abort")
     ("docs/generated/verification-authority.json"
      "docs/generated/verification-lease.json")
     "Routine verification review has spare capacity.")
    ("review-capacity:release" "release" 2 1 "available"
     ("release-pipelines" "semantic-diff")
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-evidence-quality.json")
     "Release review capacity is available for generated changes.")))

(defparameter *verification-review-overload-rules*
  '(("review-overload:freeze-high-risk" "overloaded"
     "freeze-governed-automation" "Governed automation freezes on overload.")
    ("review-overload:create-debt" "overloaded"
     "create-review-debt" "Overload creates explicit review debt.")))

(defparameter *verification-review-rules*
  '(("review-capacity:evidence-linked" "Every review queue links generated evidence.")
    ("review-capacity:capacity-valid" "Assigned review work cannot exceed capacity.")
    ("review-capacity:overload-freezes" "Overload freezes governed automation.")))

(defun emit-verification-review-capacity-json (&optional stream)
  "Emit review capacity sidecar for verification graph governance."
  (let ((json (with-output-to-string (out)
                (write-verification-review-capacity out))))
    (if stream (write-string json stream) json)))

(defun write-verification-review-capacity (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationreviewgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"review_capacity_profile\": \"governance-review-queue\",~%")
  (write-review-queues out)
  (format out ",~%")
  (write-review-overload-rules out)
  (format out ",~%")
  (write-review-rules out)
  (format out "~%}~%"))

(defun write-review-queues (out)
  (format out "  \"queues\": [~%")
  (loop for row in *verification-review-queues*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-review-queue out row))
  (format out "~%  ]"))

(defun write-review-queue (out row)
  (destructuring-bind (id scope capacity assigned status freezes evidence meaning) row
    (format out "    {\"id\": \"~A\", \"scope\": \"~A\", " id scope)
    (format out "\"capacity\": ~D, \"assigned\": ~D, " capacity assigned)
    (format out "\"status\": \"~A\", " status)
    (write-authority-list out "freezes" freezes)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))
