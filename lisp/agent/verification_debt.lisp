(in-package #:dslraid.agent)

(defparameter *verification-debt-records*
  '(("debt:review-capacity" "review" "docs/generated/verification-review-capacity.json" "open" "owner:review" "scope:human-review" ("docs/generated/verification-review-capacity.json" "docs/generated/verification-feedback.json") "repay:add-backup-steward" "revalidate:next-release" "Review overload becomes review debt until capacity is restored.")
    ("debt:evidence-freshness" "evidence" "docs/generated/verification-evidence-quality.json" "open" "owner:evidence" "scope:evidence-quality" ("docs/generated/verification-evidence-quality.json" "docs/generated/verification-pruning.json") "repay:refresh-snapshots" "revalidate:on-semantic-change" "Stale quality snapshots stay visible as evidence debt.")
    ("debt:verification-surface" "verification" "docs/generated/verification-conformance.json" "mitigated" "owner:verification" "scope:release-check" ("docs/generated/verification-conformance.json" "docs/generated/verification-codegen.json") "repay:add-generated-gate" "revalidate:each-release" "Conformance gaps are tracked until generated gates cover them.")
    ("debt:automation-freeze" "automation" "docs/generated/verification-failure-conditions.json" "open" "owner:control-plane" "scope:authority" ("docs/generated/verification-failure-conditions.json" "docs/generated/verification-authority.json") "repay:restore-authority-path" "revalidate:after-failure" "Frozen automation is debt until authority can route safely again.")))

(defparameter *verification-debt-rules*
  '(("debt:owner-required" "Every debt record has a non-agent owner.")
    ("debt:evidence-required" "Debt must cite generated evidence.")
    ("debt:repayment-required" "Open debt must name repayment and revalidation.")))

(defun emit-verification-debt-json (&optional stream)
  "Emit debt register for verification graph operations."
  (let ((json (with-output-to-string (out) (write-verification-debt out))))
    (if stream (write-string json stream) json)))

(defun write-verification-debt (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationdebtgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_debt.lisp\",~%")
  (format out "  \"debt_profile\": \"operational-learning\",~%")
  (write-debt-records out)
  (format out ",~%")
  (write-debt-rules out)
  (format out "~%}~%"))

(defun write-debt-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-debt-records*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-debt-record out row))
  (format out "~%  ]"))

(defun write-debt-record (out row)
  (destructuring-bind (id kind source status owner owed evidence repay reval meaning) row
    (format out "    {\"id\": \"~A\", \"debt_kind\": \"~A\", " id kind)
    (format out "\"source\": \"~A\", \"status\": \"~A\", " source status)
    (format out "\"owner\": \"~A\", \"owed_to\": \"~A\", " owner owed)
    (write-authority-list out "evidence" evidence)
    (format out ", \"repayment\": \"~A\", \"revalidation\": \"~A\", " repay reval)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-debt-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-debt-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationdebtgen.sh check\"}")))
  (format out "~%  ]"))
