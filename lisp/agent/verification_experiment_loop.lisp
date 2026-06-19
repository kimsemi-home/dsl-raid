(in-package #:dslraid.agent)
(defparameter *verification-experiments*
  '(("experiment:bootstrap-sequence" "Bootstrap order should be explicit evidence."
     "lisp/agent/verification_bootstrap_sequence.lisp"
     "docs/generated/verification-bootstrap-sequence.json"
     "scripts/verificationbootstrapgen.sh check"
     "docs/generated/verification-conformance.json"
     ("docs/generated/verification-bootstrap-sequence.json"
      "docs/generated/verification-semantic-diff.json")
     "checked" t
     "Bootstrap sequence is promoted when schema, semantic diff, and release checks pass.")
    ("experiment:run-manifest" "Agent runs should be file-backed evidence."
     "lisp/agent/verification_run_manifest.lisp"
     "docs/generated/verification-run-manifest.json"
     "scripts/verificationrunmanifestgen.sh check"
     "docs/generated/verification-evidence-quality.json"
     ("docs/generated/verification-run-manifest.json"
      "docs/generated/verification-evidence-quality.json")
     "checked" t
     "Run manifests are promoted when schema and generated docs agree.")))

(defparameter *verification-experiment-rules*
  '(("experiment:plan-file" "Each experiment names the Lisp plan file.")
    ("experiment:do-file" "Each experiment creates a generated evidence file.")
    ("experiment:check-command" "Each experiment has a deterministic check command.")
    ("experiment:act-evidence" "Each experiment names the follow-up act evidence.")))

(defun emit-verification-experiment-json (&optional stream)
  "Emit PDCA experiment records."
  (let ((json (with-output-to-string (out) (write-verification-experiment out))))
    (if stream (write-string json stream) json)))

(defun write-verification-experiment (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationexperimentgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_experiment_loop.lisp\",~%")
  (format out "  \"experiment_profile\": \"pdca-file-backed-experiments\",~%")
  (write-experiments out)
  (format out ",~%")
  (write-experiment-rules out)
  (format out "~%}~%"))

(defun write-experiments (out)
  (format out "  \"experiments\": [~%")
  (loop for row in *verification-experiments* for first = t then nil
        do (unless first (format out ",~%")) (write-experiment out row))
  (format out "~%  ]"))

(defun write-experiment (out row)
  (destructuring-bind (id hypothesis plan do check act evidence status promoted meaning) row
    (format out "    {\"id\": \"~A\", \"hypothesis\": \"~A\", " id hypothesis)
    (format out "\"plan\": \"~A\", \"do\": \"~A\", " plan do)
    (format out "\"check\": \"~A\", \"act\": \"~A\", " check act)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"~A\", \"promoted\": ~A, "
            status (if promoted "true" "false"))
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-experiment-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-experiment-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationexperimentgen.sh check\"}")))
  (format out "~%  ]"))
