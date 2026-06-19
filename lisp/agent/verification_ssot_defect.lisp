(in-package #:dslraid.agent)

(defparameter *verification-ssot-defects*
  '(("ssot-defect:verification-graph-contract" "lisp/agent/verification.lisp"
     "contract-drift" "drill"
     ("verify:daemon" "versioned-ssot:verification-graph")
     "freeze:affected-verification-scope"
     "migration:verification-graph-patch" "verification:full-gate"
     "gate:authority"
     ("docs/generated/verification-versioned-ssot.json"
      "docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-evidence-before-change.json")
     "SSOT defect drill freezes derived backends before patch.")))

(defparameter *verification-ssot-defect-rules*
  '(("ssot-defect:freeze-first" "Affected generated surfaces freeze first.")
    ("ssot-defect:migration-required" "SSOT patch names migration plan.")
    ("ssot-defect:authority-gated" "Defect closure is governance-gated.")))

(defun emit-verification-ssot-defect-json (&optional stream)
  "Emit SSOT defect handling receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-ssot-defect out))))
    (if stream (write-string json stream) json)))

(defun write-verification-ssot-defect (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationssotdefectgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"ssot_defect_profile\": \"freeze-migrate-verify\",~%")
  (write-ssot-defects out)
  (format out ",~%")
  (write-ssot-defect-rules out)
  (format out "~%}~%"))

(defun write-ssot-defects (out)
  (format out "  \"defects\": [~%")
  (loop for row in *verification-ssot-defects*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-ssot-defect out row))
  (format out "~%  ]"))

(defun write-ssot-defect (out row)
  (destructuring-bind (id ssot kind status scope freeze migration plan authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"ssot\": \"~A\", " id ssot)
    (format out "\"defect_kind\": \"~A\", \"status\": \"~A\", " kind status)
    (write-authority-list out "affected_scope" scope)
    (format out ", \"freeze\": \"~A\", " freeze)
    (format out "\"migration_plan\": \"~A\", " migration)
    (format out "\"verification_plan\": \"~A\", " plan)
    (format out "\"authority\": \"~A\", " authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-ssot-defect-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-ssot-defect-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationssotdefectgen.sh check\"}")))
  (format out "~%  ]"))
