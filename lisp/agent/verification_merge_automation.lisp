(in-package #:dslraid.agent)

(defparameter *verification-merge-automation*
  '(("merge-auto:pr-source" "source"
     "pull_request" ".github/workflows/ci.yml"
     ("docs/generated/verification-github-actions.json")
     "Automatic merge authority starts from pull request checks.")
    ("merge-auto:required-checks" "required-checks"
     "CI, Security, Golden, Pages, Verification Graph"
     "docs/generated/verification-merge-readiness.json"
     ("docs/generated/verification-conformance.json")
     "Every required workflow must be represented in merge readiness.")
    ("merge-auto:no-target-event" "forbidden-event"
     "pull_request_target" ".github/workflows"
     ("docs/generated/verification-security-audit.json")
     "Merge automation must not use target-context PR execution.")
    ("merge-auto:least-permission" "permission"
     "contents: read" ".github/workflows/ci.yml"
     ("docs/generated/verification-access-policy.json")
     "Merge checks run with read-only repository contents.")
    ("merge-auto:delete-branch" "cleanup"
     "delete-branch" "docs/generated/verification-merge-readiness.json"
     ("docs/generated/verification-evidence-quality.json")
     "Successful automation may clean feature branches after merge.")))

(defparameter *verification-merge-automation-rules*
  '(("merge-auto:readiness-required" "Automation must depend on merge readiness.")
    ("merge-auto:no-target-event" "Automation must forbid pull_request_target.")
    ("merge-auto:required-workflows" "Automation must name required workflows.")
    ("merge-auto:least-permission" "Automation must keep check workflows read-only.")))

(defun emit-verification-merge-automation-json (&optional stream)
  "Emit safe automatic merge policy evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-merge-automation out))))
    (if stream (write-string json stream) json)))

(defun write-verification-merge-automation (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationautomergegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_merge_automation.lisp\",~%")
  (format out "  \"automation_profile\": \"pr-gated-automerge\",~%")
  (write-merge-automation-policies out)
  (format out ",~%")
  (write-merge-automation-rules out)
  (format out "~%}~%"))

(defun write-merge-automation-policies (out)
  (format out "  \"policies\": [~%")
  (loop for row in *verification-merge-automation*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-merge-automation-policy out row))
  (format out "~%  ]"))

(defun write-merge-automation-policy (out row)
  (destructuring-bind (id policy requirement target evidence meaning) row
    (format out "    {\"id\": \"~A\", \"policy\": \"~A\", " id policy)
    (format out "\"requirement\": \"~A\", \"target\": \"~A\", " requirement target)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-merge-automation-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-merge-automation-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationautomergegen.sh check\"}")))
  (format out "~%  ]"))
