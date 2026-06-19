(in-package #:dslraid.agent)

(defparameter *verification-leases*
  '(("lease:verification-daemon" "control-plane:verification" "verify:daemon"
     "finished" "authority-after-checks"
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-authority.json")
     "Verification work is leased to automation; authority remains gated.")
    ("lease:release-check" "control-plane:verification" "verify:daemon.release-check"
     "finished" "authority-after-checks"
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-semantic-diff.json")
     "Release checks finish only when generated evidence is fresh.")))

(defparameter *verification-abort-rules*
  '(("abort:evidence-retained" "failed-check" "authority-blocked"
     "Abort preserves evidence and blocks authority promotion.")
    ("abort:rebase-required" "stale-output" "authority-blocked"
     "Stale generated output requires rebase or regeneration.")))

(defparameter *verification-lease-rules*
  '(("lease:evidence-linked" "Every lease links generated evidence.")
    ("lease:abort-blocks-authority" "Abort rules block authority promotion.")
    ("lease:no-evidence-deletion" "Abort never deletes evidence.")))

(defun emit-verification-lease-json (&optional stream)
  "Emit lease and abort sidecar for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-lease out))))
    (if stream (write-string json stream) json)))

(defun write-verification-lease (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationleasegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"lease_profile\": \"verification-work-lease\",~%")
  (write-verification-leases out)
  (format out ",~%")
  (write-abort-rules out)
  (format out ",~%")
  (write-lease-rules out)
  (format out "~%}~%"))

(defun write-verification-leases (out)
  (format out "  \"leases\": [~%")
  (loop for row in *verification-leases*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-verification-lease-row out row))
  (format out "~%  ]"))

(defun write-verification-lease-row (out row)
  (destructuring-bind (id holder scope status effect evidence meaning) row
    (format out "    {\"id\": \"~A\", \"holder\": \"~A\", " id holder)
    (format out "\"scope\": \"~A\", \"status\": \"~A\", " scope status)
    (format out "\"authority_effect\": \"~A\", " effect)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-abort-rules (out)
  (format out "  \"abort_rules\": [~%")
  (loop for row in *verification-abort-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id trigger effect meaning) row
             (format out "    {\"id\": \"~A\", \"trigger\": \"~A\", " id trigger)
             (format out "\"effect\": \"~A\", \"meaning\": \"~A\"}" effect meaning)))
  (format out "~%  ]"))
