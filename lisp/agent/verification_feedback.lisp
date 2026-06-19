(in-package #:dslraid.agent)

(defparameter *verification-feedback-items*
  '(("feedback:stale-generated-output" "stale-output"
     "spec:update-generated-freshness" "gate:quality"
     "closed" "revalidate-generated-output"
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-semantic-diff.json")
     "Stale output feedback closes through regeneration and semantic recheck.")
    ("feedback:review-overload" "review-overload"
     "policy:update-review-capacity" "gate:review-capacity"
     "closed" "revalidate-review-capacity"
     ("docs/generated/verification-review-capacity.json"
      "docs/generated/verification-evidence-quality.json")
     "Review overload feedback closes through capacity policy update.")))

(defparameter *verification-feedback-rules*
  '(("feedback:owner-required" "Every feedback item has a governance owner.")
    ("feedback:update-required" "Every closed feedback item names a knowledge update.")
    ("feedback:evidence-linked" "Every feedback item links generated evidence.")
    ("feedback:revalidation-required" "Closure requires a revalidation target.")))

(defun emit-verification-feedback-json (&optional stream)
  "Emit feedback closure sidecar for verification graph learning loops."
  (let ((json (with-output-to-string (out)
                (write-verification-feedback out))))
    (if stream (write-string json stream) json)))

(defun write-verification-feedback (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationfeedbackgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"feedback_profile\": \"knowledge-closure-loop\",~%")
  (write-feedback-items out)
  (format out ",~%")
  (write-feedback-rules out)
  (format out "~%}~%"))

(defun write-feedback-items (out)
  (format out "  \"closures\": [~%")
  (loop for row in *verification-feedback-items*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-feedback-item out row))
  (format out "~%  ]"))

(defun write-feedback-item (out row)
  (destructuring-bind (id trigger update owner status revalidation evidence meaning) row
    (format out "    {\"id\": \"~A\", \"trigger\": \"~A\", " id trigger)
    (format out "\"update\": \"~A\", \"owner\": \"~A\", " update owner)
    (format out "\"status\": \"~A\", \"revalidation\": \"~A\", " status revalidation)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))
