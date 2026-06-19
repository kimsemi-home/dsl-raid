(in-package #:dslraid.agent)

(defparameter *verification-backup-stewards*
  '(("backup-steward:review-capacity" "owner:review" "steward:backup-review"
     "active" "review-capacity-overload"
     ("domain-close" "prior-decision-history" "evidence-literate" "capacity-available")
     ("revalidation" "evidence-review" "low-risk-renewal")
     ("major-ontology-change" "security-boundary-change" "authority-model-change"
      "genesis-charter-rewrite" "permanent-owner-transfer")
     ("docs/generated/verification-review-capacity.json"
      "docs/generated/verification-debt.json")
     "review-back:owner-return"
     "Backup steward keeps review debt moving without permanent authority.")
    ("backup-steward:evidence-quality" "owner:evidence" "steward:evidence-backup"
     "candidate" "evidence-freshness-revalidation"
     ("domain-close" "evidence-literate" "ontology-context-known" "capacity-available")
     ("revalidation" "evidence-review" "risk-acceptance-proposal")
     ("security-boundary-change" "authority-model-change" "permanent-owner-transfer")
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-pruning.json")
     "review-back:evidence-owner"
     "Candidate steward may revalidate evidence but not expand authority.")))

(defparameter *verification-backup-steward-rules*
  '(("backup-steward:temporary-authority" "Backup steward authority is temporary.")
    ("backup-steward:forbidden-actions" "High-risk changes stay forbidden.")
    ("backup-steward:evidence-linked" "Steward decisions cite generated evidence.")
    ("backup-steward:review-back" "Original owner review-back is required.")))

(defun emit-verification-backup-steward-json (&optional stream)
  "Emit backup steward authority records for verification governance."
  (let ((json (with-output-to-string (out)
                (write-verification-backup-steward out))))
    (if stream (write-string json stream) json)))

(defun write-verification-backup-steward (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationstewardgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_backup_steward.lisp\",~%")
  (format out "  \"steward_profile\": \"temporary-owner-continuity\",~%")
  (write-backup-steward-assignments out)
  (format out ",~%")
  (write-backup-steward-rules out)
  (format out "~%}~%"))

(defun write-backup-steward-assignments (out)
  (format out "  \"assignments\": [~%")
  (loop for row in *verification-backup-stewards*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-backup-steward-assignment out row))
  (format out "~%  ]"))

(defun write-backup-steward-assignment (out row)
  (destructuring-bind (id owner steward status trigger criteria allowed forbidden evidence review-back meaning) row
    (format out "    {\"id\": \"~A\", \"missing_owner\": \"~A\", " id owner)
    (format out "\"steward\": \"~A\", \"status\": \"~A\", " steward status)
    (format out "\"trigger\": \"~A\", " trigger)
    (write-authority-list out "criteria" criteria) (format out ", ")
    (write-authority-list out "allowed" allowed) (format out ", ")
    (write-authority-list out "forbidden" forbidden) (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"review_back\": \"~A\", \"meaning\": \"~A\"}" review-back meaning)))

(defun write-backup-steward-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-backup-steward-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationstewardgen.sh check\"}")))
  (format out "~%  ]"))
