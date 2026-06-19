(in-package #:dslraid.agent)

(defparameter *verification-evidence-separation-records*
  '(("separation:runtime-trace" "examples/runscope/run-001.trace.json"
     "interpretation:runtime-trace" "claim:trace-maps-design"
     "docs/generated/trace-catalog.md" "raw-to-claim-via-interpretation"
     ("docs/generated/verification-runtime-trace.json"
      "docs/generated/verification-evidence.json")
     "Runtime trace evidence becomes a claim only through design mapping.")
    ("separation:quality-run" "examples/runscope/runscope.agent-run.json"
     "interpretation:quality-sidecar" "claim:quality-gate-passed"
     "docs/generated/agent-run-manifest.md" "raw-to-claim-via-interpretation"
     ("docs/generated/verification-sidecar.json"
      "docs/generated/verification-confidence.json")
     "Agent run claims stay distinct from sidecar verification evidence.")
    ("separation:artifact-lock" "examples/runscope/runscope.lock.json"
     "interpretation:artifact-freshness" "claim:generated-artifacts-fresh"
     "docs/generated/verification-release-provenance.json"
     "raw-to-claim-via-interpretation"
     ("docs/generated/verification-release-provenance.json"
      "docs/generated/verification-codegen.json")
     "Artifact freshness is a claim derived from lock evidence.")))

(defparameter *verification-evidence-separation-rules*
  '(("separation:raw-not-claim" "Raw evidence is not itself a claim.")
    ("separation:claim-not-artifact" "Claims and artifacts remain separate.")
    ("separation:interpretation-required" "Every claim has an interpretation.")
    ("separation:evidence-linked" "Separation records cite generated evidence.")))

(defun emit-verification-evidence-separation-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-evidence-separation out))))
    (if stream (write-string json stream) json)))

(defun write-verification-evidence-separation (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationseparationgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_evidence_separation.lisp\",~%")
  (format out "  \"evidence_separation_profile\": \"raw-interpretation-claim-artifact\",~%")
  (write-evidence-separation-records out)
  (format out ",~%")
  (write-evidence-separation-rules out)
  (format out "~%}~%"))

(defun write-evidence-separation-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-evidence-separation-records*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-evidence-separation-record out row))
  (format out "~%  ]"))

(defun write-evidence-separation-record (out row)
  (destructuring-bind (id raw interpretation claim artifact policy evidence meaning) row
    (format out "    {\"id\": \"~A\", \"raw_evidence\": \"~A\", " id raw)
    (format out "\"interpretation\": \"~A\", \"claim\": \"~A\", " interpretation claim)
    (format out "\"artifact\": \"~A\", \"policy\": \"~A\", " artifact policy)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-evidence-separation-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-evidence-separation-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationseparationgen.sh check\"}")))
  (format out "~%  ]"))
