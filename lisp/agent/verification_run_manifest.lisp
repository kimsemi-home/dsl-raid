(in-package #:dslraid.agent)

(defparameter *verification-run-manifest-records*
  '(("run-manifest:runscope-quality" "agent-run:runscope-quality-001"
     "examples/runscope/runscope.agent-run.json"
     "schemas/dslraid-agent-run.schema.json"
     "docs/generated/agent-run-manifest.md"
     "verified" "gate:quality"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-conformance.json")
     "Run manifest binds SSOT, orchestration, authority, lease, evidence, and artifacts.")
    ("run-manifest:quality-evidence" "agent-run:runscope-quality-001"
     "examples/runscope/runscope.agent-run.json"
     "schemas/dslraid-agent-run.schema.json"
     "docs/generated/agent-run-manifest.md"
     "verified" "sidecar:dslraid-quality"
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-sidecar.json")
     "Run evidence remains meaningful only through sidecar and quality snapshots.")))

(defparameter *verification-run-manifest-rules*
  '(("run-manifest:schema-valid" "Run manifests must validate against the agent-run schema.")
    ("run-manifest:doc-generated" "Human run docs are generated from the manifest.")
    ("run-manifest:authority-bound" "Each run declares authority and evidence.")
    ("run-manifest:file-backed" "Run records point to files, not runtime-only state.")))

(defun emit-verification-run-manifest-json (&optional stream)
  "Emit run manifest verification receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-run-manifest out))))
    (if stream (write-string json stream) json)))

(defun write-verification-run-manifest (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationrunmanifestgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_run_manifest.lisp\",~%")
  (format out "  \"run_manifest_profile\": \"file-backed-agent-runs\",~%")
  (write-run-manifest-records out)
  (format out ",~%")
  (write-run-manifest-rules out)
  (format out "~%}~%"))

(defun write-run-manifest-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-run-manifest-records*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-run-manifest-record out row))
  (format out "~%  ]"))

(defun write-run-manifest-record (out row)
  (destructuring-bind (id run manifest schema doc status authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"run\": \"~A\", " id run)
    (format out "\"manifest\": \"~A\", \"schema\": \"~A\", " manifest schema)
    (format out "\"generated_doc\": \"~A\", \"status\": \"~A\", " doc status)
    (format out "\"authority\": \"~A\", " authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-run-manifest-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-run-manifest-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationrunmanifestgen.sh check\"}")))
  (format out "~%  ]"))
