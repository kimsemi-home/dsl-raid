(in-package #:dslraid.agent)

(defparameter *verification-reasoning-access-records*
  '(("reasoning:r0-codegen" "agent:verification-daemon" "R0" "verification"
     "abac:deterministic-transform" "transform-only" "low"
     ("generate-artifact" "check-stale") ("approve-authority" "semantic-judgment" "production-change")
     ("docs/generated/verification-codegen.json" "docs/generated/verification-conformance.json")
     "owner:verification"
     "R0 agents perform deterministic transforms and cannot decide meaning.")
    ("reasoning:r2-analysis" "agent:quality-runner" "R2" "public-surface"
     "abac:low-risk-evidence" "analysis-only" "medium"
     ("inspect-ir" "draft-diagnostic") ("merge" "release" "security-boundary-change")
     ("docs/generated/verification-evidence-quality.json" "docs/generated/verification-semantic-diff.json")
     "owner:quality"
     "R2 agents may analyze bounded evidence but cannot promote artifacts.")
    ("reasoning:r3-root-cause" "agent:semantic-debugger" "R3" "incident"
     "abac:validated-evidence" "human-review" "medium"
     ("propose-root-cause" "map-ontology-conflict") ("confirm-root-cause" "approve-authority")
     ("docs/generated/verification-root-cause.json" "docs/generated/verification-semantic-debugger.json")
     "owner:review"
     "R3 agents propose causes and conflicts, then require human review.")
    ("reasoning:r4-governance" "agent:governance-reviewer" "R4" "ontology"
     "abac:governance-change" "governance-review" "high"
     ("review-policy" "review-ontology-change") ("self-approve" "bypass-authority-gate")
     ("docs/generated/verification-authority.json" "docs/generated/verification-adr-governance.json")
     "governance:ontology"
     "R4 reasoning can review governance but still cannot self-approve.")))

(defparameter *verification-reasoning-access-rules*
  '(("reasoning:tier-is-not-authority" "Higher reasoning tier never grants approval.")
    ("reasoning:context-gates-tier" "Sensitive contexts require governance review.")
    ("reasoning:evidence-required" "Reasoning authority cites generated evidence.")
    ("reasoning:r0-transform-only" "R0 agents only run deterministic transforms.")))

(defun emit-verification-reasoning-access-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-reasoning-access out))))
    (if stream (write-string json stream) json)))

(defun write-verification-reasoning-access (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationreasoninggen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_reasoning_access.lisp\",~%")
  (format out "  \"reasoning_access_profile\": \"reasoning-rbac-domain-abac\",~%")
  (write-reasoning-access-records out)
  (format out ",~%")
  (write-reasoning-access-rules out)
  (format out "~%}~%"))

(defun write-reasoning-access-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-reasoning-access-records* for first = t then nil
        do (unless first (format out ",~%")) (write-reasoning-access-record out row))
  (format out "~%  ]"))

(defun write-reasoning-access-record (out row)
  (destructuring-bind (id agent tier context abac effect ceiling allowed blocked evidence owner meaning) row
    (format out "    {\"id\": \"~A\", \"agent\": \"~A\", " id agent)
    (format out "\"reasoning_tier\": \"~A\", \"context\": \"~A\", " tier context)
    (format out "\"abac\": \"~A\", \"authority_effect\": \"~A\", " abac effect)
    (format out "\"confidence_ceiling\": \"~A\", " ceiling)
    (write-authority-list out "allowed" allowed) (format out ", ")
    (write-authority-list out "blocked" blocked) (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"owner\": \"~A\", \"meaning\": \"~A\"}" owner meaning)))

(defun write-reasoning-access-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-reasoning-access-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationreasoninggen.sh check\"}")))
  (format out "~%  ]"))
