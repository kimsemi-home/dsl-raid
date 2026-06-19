(in-package #:dslraid.agent)

(defparameter *verification-meta-model-terms*
  '(("metamodel:actor" "Actor" "Subject that proposes, verifies, reviews, or governs work."
     "context:governance" "owner:ontology" "gate:authority")
    ("metamodel:artifact" "Artifact" "Generated or authored output with provenance."
     "context:codegen" "owner:verification" "gate:quality")
    ("metamodel:evidence" "Evidence" "Observed record linked to version and meaning."
     "context:evidence" "owner:evidence" "gate:evidence-quality")
    ("metamodel:claim" "Claim" "Interpretation that requires supporting evidence."
     "context:evidence" "owner:verification" "gate:confidence")
    ("metamodel:decision" "Decision" "Governed choice with approver and evidence."
     "context:governance" "owner:governance" "gate:authority")
    ("metamodel:change" "Change" "Candidate update to SSOT or generated surfaces."
     "context:versioned-ssot" "owner:verification" "gate:evidence-before-change")
    ("metamodel:review" "Review" "Independent examination before authority."
     "context:review" "owner:review" "gate:review-capacity")
    ("metamodel:risk" "Risk" "Bounded harm scenario that changes authority."
     "context:security" "owner:security" "gate:security")
    ("metamodel:version" "Version" "Scoped semantic coordinate for SSOT and evidence."
     "context:ontology" "owner:ontology" "gate:versioned-ssot")
    ("metamodel:migration" "Migration" "Controlled transition between semantic versions."
     "context:ontology-transition" "owner:ontology" "gate:ontology-transition")
    ("metamodel:audit" "Audit" "Independent check of authority and evidence boundary."
     "context:audit" "owner:governance" "gate:security-audit")))

(defparameter *verification-meta-model-rules*
  '(("meta-model:terms-owned" "Every term has an owner.")
    ("meta-model:gate-linked" "Every term names an authority gate.")
    ("meta-model:evidence-linked" "Meta-model cites generated evidence.")
    ("meta-model:amendment-governed" "Meta-model changes require governance.")))

(defun emit-verification-meta-model-json (&optional stream)
  "Emit base governance vocabulary for verification graph operations."
  (let ((json (with-output-to-string (out)
                (write-verification-meta-model out))))
    (if stream (write-string json stream) json)))

(defun write-verification-meta-model (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationmetamodelgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_meta_model.lisp\",~%")
  (format out "  \"meta_model_profile\": \"governance-vocabulary\",~%")
  (write-meta-model-terms out)
  (format out ",~%")
  (write-meta-model-rules out)
  (format out "~%}~%"))

(defun write-meta-model-terms (out)
  (format out "  \"terms\": [~%")
  (loop for row in *verification-meta-model-terms*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id term meaning context owner gate) row
             (format out "    {\"id\": \"~A\", \"term\": \"~A\", " id term)
             (format out "\"meaning\": \"~A\", \"context\": \"~A\", " meaning context)
             (format out "\"owner\": \"~A\", \"authority_gate\": \"~A\"}" owner gate)))
  (format out "~%  ]"))

(defun write-meta-model-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-meta-model-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationmetamodelgen.sh check\"}")))
  (format out "~%  ]"))
