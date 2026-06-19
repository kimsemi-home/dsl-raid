(in-package #:dslraid.agent)

(defparameter *verification-ontology-transitions*
  '(("ontology-transition:verification-0.1.0" "ontology:verification.0.1.0"
     "ontology:verification.0.2.0" "compatibility-window:dual-run"
     ("lane:legacy" "lane:migration" "lane:new" "lane:audit")
     "gate:authority"
     ("docs/generated/verification-versioned-ssot.json"
      "docs/generated/verification-context-map.json"
      "docs/generated/verification-historical-interpreter.json")
     "Ontology transition keeps old and new meanings lane-scoped.")))

(defparameter *verification-ontology-transition-rules*
  '(("ontology-transition:dual-lanes" "Legacy, migration, new, audit lanes exist.")
    ("ontology-transition:bridge-first" "Transition links context and history bridges.")
    ("ontology-transition:cutover-gated" "Cutover is gated by governance authority.")))

(defun emit-verification-ontology-transition-json (&optional stream)
  "Emit ontology transition lane receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-ontology-transition out))))
    (if stream (write-string json stream) json)))

(defun write-verification-ontology-transition (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationtransitiongen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"ontology_transition_profile\": \"dual-lane-cutover\",~%")
  (write-ontology-transitions out)
  (format out ",~%")
  (write-ontology-transition-rules out)
  (format out "~%}~%"))

(defun write-ontology-transitions (out)
  (format out "  \"transitions\": [~%")
  (loop for row in *verification-ontology-transitions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-ontology-transition out row))
  (format out "~%  ]"))

(defun write-ontology-transition (out row)
  (destructuring-bind (id from to window lanes gate evidence meaning) row
    (format out "    {\"id\": \"~A\", \"from_version\": \"~A\", " id from)
    (format out "\"to_version\": \"~A\", " to)
    (format out "\"compatibility_window\": \"~A\", " window)
    (write-authority-list out "lanes" lanes)
    (format out ", \"cutover_gate\": \"~A\", " gate)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-ontology-transition-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-ontology-transition-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationtransitiongen.sh check\"}")))
  (format out "~%  ]"))
