(in-package #:dslraid.agent)
(defparameter *verification-bootstrap-stages*
  '((1 "bootstrap:lisp-to-ir" "common-lisp"
     "lisp/runtime/runscope.lisp" "examples/runscope/runscope.lisp.raid.json"
     "scripts/lisp-irgen.sh check" "checked"
     ("docs/generated/verification-conformance.json")
     "Common Lisp forms emit canonical Core IR as a file-backed artifact.")
    (2 "bootstrap:ir-to-rust" "generated-runtime"
     "examples/runscope/runscope.lisp.raid.json" "generated/runtime_fsm.rs"
     "scripts/lisp-rustgen.sh check" "checked"
     ("docs/generated/verification-codegen.json")
     "Rust runtime code is generated from canonical IR, not edited as SSOT.")
    (3 "bootstrap:graph-evidence" "verification-graph"
     "lisp/agent/verification.lisp" "docs/generated/verification-evidence.json"
     "scripts/verificationevidencegen.sh check" "checked"
     ("docs/generated/verification-ontology.json")
     "The verification graph emits the backend inventory used as evidence.")
    (4 "bootstrap:graph-schema" "schema"
     "lisp/agent/verification_manifest_schema.lisp"
     "schemas/dslraid-verification-manifest.schema.json"
     "scripts/verificationmanifestschemagen.sh check" "checked"
     ("docs/generated/verification-evidence.json")
     "Manifest schemas are generated from the verification language layer.")
    (5 "bootstrap:graph-tests" "tests"
     "lisp/agent/verification_tests.lisp" "tests/golden/verification-graph.generated.json"
     "scripts/verificationtestgen.sh check" "checked"
     ("docs/generated/verification-conformance.json")
     "Golden test manifests are generated from the same verification graph.")
    (6 "bootstrap:graph-pipeline" "github-actions"
     "lisp/agent/verification_release_checks.lisp" ".github/workflows/verification.yml"
     "scripts/workflowgen.sh check" "checked"
     ("docs/generated/verification-github-actions.json"
      "docs/generated/verification-backend-parity.json")
     "GitHub Actions is a generated projection of the verification graph.")))
(defparameter *verification-bootstrap-rules*
  '(("bootstrap:file-backed" "Every bootstrap stage reads and writes files.")
    ("bootstrap:checkable" "Every bootstrap stage has a check command.")
    ("bootstrap:evidence-linked" "Every bootstrap stage links generated evidence.")
    ("bootstrap:ssot-first" "Language and IR precede runtime and pipeline outputs.")))
(defun emit-verification-bootstrap-json (&optional stream)
  "Emit executable boot sequence evidence."
  (let ((json (with-output-to-string (out) (write-verification-bootstrap out))))
    (if stream (write-string json stream) json)))
(defun write-verification-bootstrap (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationbootstrapgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_bootstrap_sequence.lisp\",~%")
  (format out "  \"bootstrap_profile\": \"lisp-ssot-filegen\",~%")
  (write-bootstrap-stages out)
  (format out ",~%")
  (write-bootstrap-rules out)
  (format out "~%}~%"))
(defun write-bootstrap-stages (out)
  (format out "  \"stages\": [~%")
  (loop for row in *verification-bootstrap-stages* for first = t then nil
        do (unless first (format out ",~%")) (write-bootstrap-stage out row))
  (format out "~%  ]"))
(defun write-bootstrap-stage (out row)
  (destructuring-bind (order id layer input output check status evidence meaning) row
    (format out "    {\"order\": ~D, \"id\": \"~A\", \"layer\": \"~A\", " order id layer)
    (format out "\"input\": \"~A\", \"output\": \"~A\", " input output)
    (format out "\"check\": \"~A\", \"status\": \"~A\", " check status)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))
(defun write-bootstrap-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-bootstrap-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationbootstrapgen.sh check\"}")))
  (format out "~%  ]"))
