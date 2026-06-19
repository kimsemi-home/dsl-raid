(in-package #:dslraid.agent)

(defparameter *verification-genesis-charter*
  '("genesis:verification-daemon"
    "Executable verification graph for generated quality surfaces."
    ("context:verification" "context:codegen" "context:release")
    ("agent:verification-daemon" "gate:quality" "owner:verification")
    ("lisp/agent/verification.lisp" ".github/workflows/verification.yml")
    ("evidence:semantic-hash" "evidence:conformance" "evidence:artifact-freshness")
    ("authority:governance" "gate:quality" "gate:release")
    ("assumption:file-generated-artifacts" "assumption:generated-checks-are-authoritative")
    ("non-goal:manual-diagram-source" "non-goal:private-data-publication")
    ("risk:private-data" "risk:authority-bypass" "risk:ssot-drift")
    "owner:verification" "revalidate:on-ontology-change"
    ("docs/generated/verification-ontology.json"
     "docs/generated/verification-evidence.json")
    "Genesis charter names the first bounded responsibility record."))

(defparameter *verification-genesis-rules*
  '(("genesis:owner-required" "Genesis charter has a non-agent owner.")
    ("genesis:revalidation-required" "Genesis charter names revalidation.")
    ("genesis:evidence-linked" "Genesis charter cites generated evidence.")
    ("genesis:non-goals-explicit" "Genesis charter names what it will not do.")))

(defun emit-verification-genesis-json (&optional stream)
  "Emit genesis charter for verification graph governance."
  (let ((json (with-output-to-string (out)
                (write-verification-genesis out))))
    (if stream (write-string json stream) json)))

(defun write-verification-genesis (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationgenesisgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_genesis_charter.lisp\",~%")
  (format out "  \"genesis_profile\": \"bounded-starting-contract\",~%")
  (write-genesis-charter out *verification-genesis-charter*)
  (format out ",~%")
  (write-genesis-rules out)
  (format out "~%}~%"))

(defun write-genesis-charter (out row)
  (destructuring-bind (id purpose contexts actors artifacts types authority assumptions non-goals risks owner reval evidence meaning) row
    (format out "  \"charter\": {\"id\": \"~A\", \"purpose\": \"~A\", " id purpose)
    (write-authority-list out "bounded_contexts" contexts) (format out ", ")
    (write-authority-list out "actors" actors) (format out ", ")
    (write-authority-list out "artifacts" artifacts) (format out ", ")
    (write-authority-list out "evidence_types" types) (format out ", ")
    (write-authority-list out "authority_rules" authority) (format out ", ")
    (write-authority-list out "assumptions" assumptions) (format out ", ")
    (write-authority-list out "non_goals" non-goals) (format out ", ")
    (write-authority-list out "risk_boundaries" risks) (format out ", ")
    (format out "\"review_owner\": \"~A\", \"revalidation\": \"~A\", " owner reval)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-genesis-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-genesis-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationgenesisgen.sh check\"}")))
  (format out "~%  ]"))
