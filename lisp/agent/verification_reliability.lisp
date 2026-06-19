(in-package #:dslraid.agent)

(defparameter *verification-reliability-records*
  '(("reliability:verification-daemon" "agent:verification-daemon" "T3" "bounded" "medium" 2 98 "bounded-auto" ("docs/generated/verification-conformance.json" "docs/generated/verification-evidence.json") ("no-private-data" "security-escalates") "Verification daemon may automate bounded release checks.")
    ("reliability:quality-runner" "agent:quality-runner" "T3" "bounded" "medium" 1 99 "bounded-auto" ("docs/generated/verification-evidence-quality.json" "docs/generated/verification-semantic-diff.json") ("requires-generated-evidence" "no-authority-approval") "Quality runner may assess evidence but cannot approve authority.")
    ("reliability:new-agent" "agent:candidate" "T0" "candidate" "low" 0 0 "authority-blocked" ("docs/generated/verification-access-policy.json" "docs/generated/verification-debt.json") ("read-only" "proposal-only") "Cold-start agents begin with proposal-only authority.")
    ("reliability:shadow-agent" "agent:shadow" "T1" "shadow" "low" 0 0 "human-review" ("docs/generated/verification-orchestration.json" "docs/generated/verification-sidecar.json") ("shadow-only" "compare-results") "Shadow agents compare results without changing operations.")))

(defparameter *verification-reliability-rules*
  '(("reliability:evidence-required" "Reliability records cite generated evidence.")
    ("reliability:cold-start-blocked" "Cold-start agents cannot auto-approve.")
    ("reliability:confidence-bounded" "Confidence ceiling follows measured reliability.")))

(defun emit-verification-reliability-json (&optional stream)
  "Emit agent reliability and cold-start gates."
  (let ((json (with-output-to-string (out) (write-verification-reliability out))))
    (if stream (write-string json stream) json)))

(defun write-verification-reliability (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationreliabilitygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_reliability.lisp\",~%")
  (format out "  \"reliability_profile\": \"agent-scorecard\",~%")
  (write-reliability-records out)
  (format out ",~%")
  (write-reliability-rules out)
  (format out "~%}~%"))

(defun write-reliability-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-reliability-records*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-reliability-record out row))
  (format out "~%  ]"))

(defun write-reliability-record (out row)
  (destructuring-bind (id agent tier status ceiling failures rate effect evidence restrictions meaning) row
    (format out "    {\"id\": \"~A\", \"agent\": \"~A\", " id agent)
    (format out "\"tier\": \"~A\", \"status\": \"~A\", " tier status)
    (format out "\"confidence_ceiling\": \"~A\", \"failure_rate\": ~A, " ceiling failures)
    (format out "\"verification_rate\": ~A, \"authority_effect\": \"~A\", " rate effect)
    (write-authority-list out "evidence" evidence)
    (format out ", ")
    (write-authority-list out "restrictions" restrictions)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-reliability-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-reliability-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationreliabilitygen.sh check\"}")))
  (format out "~%  ]"))
