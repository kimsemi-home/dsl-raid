(in-package #:dslraid.agent)

(defparameter *verification-merge-receipts*
  '(("merge-receipt:head-sync" "head-sync"
     "git status -sb" "## main...origin/main"
     ("docs/generated/verification-actions-receipt.json"
      "docs/generated/verification-merge-automation.json")
     ("branch" "headSha" "remote") "closed"
     "A pushed merge is complete only when local and remote main agree.")
    ("merge-receipt:required-workflows" "required-workflows"
     "gh run list --repo kimsemi-home/dsl-raid --branch main --limit 6"
     "CI, Security, Golden, Pages, and Verification Graph success"
     ("docs/generated/verification-actions-receipt.json"
      "docs/generated/verification-github-actions.json")
     ("name" "headSha" "status" "conclusion" "url") "closed"
     "Remote completion must cover every generated required workflow.")
    ("merge-receipt:pages-health" "pages-health"
     "curl -L -s -o /tmp/dslraid-pages.html -w '%{http_code}' https://kimsemi-home.github.io/dsl-raid/"
     "200" ("docs/generated/verification-actions-receipt.json" ".github/workflows/pages.yml")
     ("url" "http_status") "closed"
     "Published viewer health closes the post-push receipt.")))

(defparameter *verification-merge-receipt-rules*
  '(("merge-receipt:closed" "Every post-push receipt must close.")
    ("merge-receipt:evidence-linked" "Every receipt cites generated evidence.")
    ("merge-receipt:required-workflows" "Workflow receipts must cover required jobs.")))

(defun emit-verification-merge-receipt-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-merge-receipt out))))
    (if stream (write-string json stream) json)))

(defun write-verification-merge-receipt (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationmergereceiptgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_merge_receipt.lisp\",~%")
  (format out "  \"merge_receipt_profile\": \"post-push-closure\",~%")
  (write-merge-receipts out)
  (format out ",~%")
  (write-merge-receipt-rules out)
  (format out "~%}~%"))

(defun write-merge-receipts (out)
  (format out "  \"receipts\": [~%")
  (loop for row in *verification-merge-receipts* for first = t then nil
        do (unless first (format out ",~%")) (write-merge-receipt out row))
  (format out "~%  ]"))

(defun write-merge-receipt (out row)
  (destructuring-bind (id kind command expected evidence fields status meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"expected\": \"~A\", " command expected)
    (write-authority-list out "evidence" evidence)
    (format out ", ")
    (write-authority-list out "fields" fields)
    (format out ", \"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-merge-receipt-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-merge-receipt-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationmergereceiptgen.sh check\"}")))
  (format out "~%  ]"))
