(in-package #:dslraid.agent)

(defparameter *verification-actions-receipts*
  '(("actions-receipt:workflow-summary" "workflow-summary" "gh run list"
     "gh run list --repo kimsemi-home/dsl-raid --branch main --json name,headSha,status,conclusion,url"
     "Required workflow runs are visible for the pushed head SHA."
     ("headSha" "status" "conclusion" "url") ("docs/generated/verification-branch-protection.json" "docs/generated/verification-github-actions.json")
     "Remote CI summaries are accepted only as structured receipts.")
    ("actions-receipt:head-sha" "head-sha" "gh run list"
     "gh run list --repo kimsemi-home/dsl-raid --branch main --json headSha,status,conclusion,url"
     "Each receipt must match the pushed commit head SHA." ("headSha" "status" "conclusion" "url") ("docs/generated/verification-merge-automation.json")
     "A successful run is not transferable across commits.")
    ("actions-receipt:job-detail" "job-detail" "gh run view"
     "gh run view <run-id> --json jobs,status,conclusion,url"
     "Failed or incomplete runs must expose job-level detail." ("jobs" "status" "conclusion" "url") ("docs/generated/verification-conformance.json")
     "Debug evidence must keep the run URL and job verdicts.")
    ("actions-receipt:pages-health" "pages-health" "curl"
     "curl -I -L https://kimsemi-home.github.io/dsl-raid/"
     "HTTP/2 200" ("url" "http_status") (".github/workflows/pages.yml")
     "Pages deploy health is a post-push receipt.")
    ("actions-receipt:no-target-event" "forbidden-event" "grep"
     "grep -R pull_request_target .github/workflows"
     "pull_request_target absent" ("workflow" "forbidden_event") ("docs/generated/verification-security-audit.json")
     "Receipt collection must not rely on target-context execution.")))

(defparameter *verification-actions-receipt-rules*
  '(("actions-receipt:required-fields" "Receipts must declare decisive fields.")
    ("actions-receipt:allowed-tools" "Receipt collection tools are bounded.")
    ("actions-receipt:evidence-linked" "Each receipt must cite file evidence.")))

(defun emit-verification-actions-receipt-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-actions-receipt out))))
    (if stream (write-string json stream) json)))

(defun write-verification-actions-receipt (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationreceiptgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_actions_receipt.lisp\",~%")
  (format out "  \"actions_receipt_profile\": \"remote-run-receipts\",~%")
  (write-actions-receipts out)
  (format out ",~%")
  (write-actions-receipt-rules out)
  (format out "~%}~%"))

(defun write-actions-receipts (out)
  (format out "  \"receipts\": [~%")
  (loop for row in *verification-actions-receipts* for first = t then nil
        do (unless first (format out ",~%")) (write-actions-receipt out row))
  (format out "~%  ]"))

(defun write-actions-receipt (out row)
  (destructuring-bind (id kind tool command expected fields evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"tool\": \"~A\", \"command\": \"~A\", " tool command)
    (format out "\"expected\": \"~A\", " expected)
    (write-authority-list out "fields" fields)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-actions-receipt-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-actions-receipt-rules* for first = t then nil
        do (unless first (format out ",~%")) (write-actions-receipt-rule out row))
  (format out "~%  ]"))

(defun write-actions-receipt-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
    (format out "\"check\": \"scripts/verificationreceiptgen.sh check\"}")))
