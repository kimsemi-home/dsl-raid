(in-package #:dslraid.agent)

(defparameter *verification-bounded-contexts*
  '(("context:verification" "VerificationContext" "ontology:verification"
     "0.1.0" "0.1.0" "gate:authority"
     ("VerificationGraph@VerificationContext" "Evidence@VerificationContext")
     ("docs/generated/verification-versioned-ssot.json")
     "Verification graph terms are scoped to verification governance.")
    ("context:manifest-contract" "ManifestContractContext" "ontology:verification"
     "0.1.0" "0.1.0" "gate:schema"
     ("Manifest@ManifestContractContext" "Schema@ManifestContractContext")
     ("schemas/dslraid-verification-manifest.schema.json")
     "Manifest terms are scoped to schema contract generation.")
    ("context:github-actions" "GithubActionsContext" "ontology:verification"
     "0.1.0" "0.1.0" "gate:workflow"
     ("Workflow@GithubActionsContext" "Job@GithubActionsContext")
     ("docs/generated/verification-github-actions.json")
     "GitHub Actions terms are scoped to workflow execution.")))

(defparameter *verification-bounded-context-rules*
  '(("bounded-context:cataloged" "Every referenced context is cataloged.")
    ("bounded-context:versioned" "Each context declares ontology and contract version.")
    ("bounded-context:no-bare-term" "Context terms include an explicit context suffix.")))

(defun emit-verification-bounded-context-json (&optional stream)
  "Emit bounded context catalog for verification semantics."
  (let ((json (with-output-to-string (out) (write-verification-bounded-context out))))
    (if stream (write-string json stream) json)))

(defun write-verification-bounded-context (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationboundedcontextgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_bounded_context.lisp\",~%")
  (format out "  \"bounded_context_profile\": \"context-address-catalog\",~%")
  (write-bounded-contexts out)
  (format out ",~%")
  (write-bounded-context-rules out)
  (format out "~%}~%"))

(defun write-bounded-contexts (out)
  (format out "  \"contexts\": [~%")
  (loop for row in *verification-bounded-contexts* for first = t then nil
        do (unless first (format out ",~%")) (write-bounded-context out row))
  (format out "~%  ]"))

(defun write-bounded-context (out row)
  (destructuring-bind (id name ontology ontology-v contract-v authority terms evidence meaning) row
    (format out "    {\"id\": \"~A\", \"name\": \"~A\", " id name)
    (format out "\"ontology\": \"~A\", " ontology)
    (format out "\"ontology_version\": \"~A\", \"contract_version\": \"~A\", " ontology-v contract-v)
    (format out "\"authority\": \"~A\", " authority)
    (write-authority-list out "terms" terms)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-bounded-context-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-bounded-context-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationboundedcontextgen.sh check\"}")))
  (format out "~%  ]"))
