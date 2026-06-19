(in-package #:dslraid.agent)

(defparameter *verification-incompleteness-items*
  '(("unknown:ontology-context" "unknown" "ontology"
     "docs/generated/verification-ontology.json" "open" "owner:ontology"
     "classify:bounded-context" "authority-blocked"
     ("docs/generated/verification-ontology.json" "docs/generated/verification-context-map.json")
     "Ontology context gaps block meaning changes until classified.")
    ("unknown:evidence-link" "gap" "evidence"
     "docs/generated/verification-evidence.json" "open" "owner:evidence"
     "classify:evidence-graph" "review-required"
     ("docs/generated/verification-evidence.json" "docs/generated/verification-evidence-quality.json")
     "Unlinked evidence stays visible until linked or rejected.")
    ("unknown:runtime-drift" "drift" "runtime"
     "docs/generated/verification-runtime-trace.json" "triaged" "owner:runtime"
     "classify:trace-mapping" "review-required"
     ("docs/generated/verification-runtime-trace.json" "docs/generated/verification-root-cause.json")
     "Runtime drift needs trace mapping before promotion.")
    ("unknown:release-assumption" "assumption" "release"
     "docs/generated/verification-release-provenance.json" "open" "owner:release"
     "classify:release-risk" "human-review"
     ("docs/generated/verification-release-provenance.json" "docs/generated/verification-adversarial-review.json")
     "Release assumptions require human review until proven.")))

(defparameter *verification-incompleteness-rules*
  '(("incomplete:owner-required" "Every incomplete item has a non-agent owner.")
    ("incomplete:evidence-required" "Unknowns cite generated evidence.")
    ("incomplete:open-needs-action" "Open unknowns name the next classification action.")
    ("incomplete:authority-explicit" "Every unknown declares authority effect.")))

(defun emit-verification-incompleteness-json (&optional stream)
  "Emit incompleteness ledger for tracked unknowns."
  (let ((json (with-output-to-string (out)
                (write-verification-incompleteness out))))
    (if stream (write-string json stream) json)))

(defun write-verification-incompleteness (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationincompletegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_incompleteness_ledger.lisp\",~%")
  (format out "  \"incompleteness_profile\": \"tracked-unknowns\",~%")
  (write-incompleteness-items out)
  (format out ",~%")
  (write-incompleteness-rules out)
  (format out "~%}~%"))

(defun write-incompleteness-items (out)
  (format out "  \"unknowns\": [~%")
  (loop for row in *verification-incompleteness-items*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-incompleteness-item out row))
  (format out "~%  ]"))

(defun write-incompleteness-item (out row)
  (destructuring-bind (id kind domain source status owner action effect evidence meaning) row
    (format out "    {\"id\": \"~A\", \"unknown_kind\": \"~A\", " id kind)
    (format out "\"domain\": \"~A\", \"source\": \"~A\", " domain source)
    (format out "\"status\": \"~A\", \"owner\": \"~A\", " status owner)
    (format out "\"next_action\": \"~A\", \"authority_effect\": \"~A\", " action effect)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-incompleteness-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-incompleteness-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationincompletegen.sh check\"}")))
  (format out "~%  ]"))
