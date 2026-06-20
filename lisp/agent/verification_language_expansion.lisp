(in-package #:dslraid.agent)

(defparameter *verification-language-expansion-surfaces*
  '(("language-expansion:pipeline-doc" "pipeline-doc"
     "scripts/lisp-docgen.sh check" "lisp/lang/pipeline.lisp"
     "stdout:lisp generated doc ok"
     ("docs/generated/lisp-pipeline.md")
     "Language pipeline docs prove Lisp Form to Canonical IR order.")
    ("language-expansion:canonical-ir" "canonical-ir"
     "scripts/lisp-irgen.sh check" "lisp/runtime/runscope.lisp"
     "stdout:lisp generated ir ok"
     ("examples/runscope/runscope.lisp.raid.json")
     "Canonical IR is generated from Lisp authoring forms.")
    ("language-expansion:rust-backend" "backend-codegen"
     "scripts/lisp-rustgen.sh check" "examples/runscope/runscope.lisp.raid.json"
     "stdout:lisp rust backend ok"
     ("generated/runtime_fsm.rs")
     "Rust source is a deterministic generated backend, not the SSOT.")
    ("language-expansion:language-conformance" "language-conformance"
     "scripts/lisp-test.sh check" "lisp/tests/golden.lisp"
     "stdout:lisp language tests ok"
     ("lisp/tests/language.lisp" "lisp/tests/pipeline.lisp")
     "Language validation runs before Canonical IR consumers rely on it.")))

(defparameter *verification-language-expansion-rules*
  '(("language-expansion:ssot" "Common Lisp forms remain native truth.")
    ("language-expansion:canonical-ir" "Backends consume Canonical IR.")
    ("language-expansion:rust-output" "Rust can be generated runtime code.")))

(defun emit-verification-language-expansion-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-language-expansion out))))
    (if stream (write-string json stream) json)))

(defun write-verification-language-expansion (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationlanguagegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_language_expansion.lisp\",~%")
  (format out "  \"language_expansion_profile\": \"lisp-form-to-canonical-ir\",~%")
  (write-language-expansion-surfaces out)
  (format out ",~%")
  (write-language-expansion-rules out)
  (format out "~%}~%"))

(defun write-language-expansion-surfaces (out)
  (format out "  \"surfaces\": [~%")
  (loop for row in *verification-language-expansion-surfaces* for first = t then nil
        do (unless first (format out ",~%"))
           (write-language-expansion-surface out row))
  (format out "~%  ]"))

(defun write-language-expansion-surface (out row)
  (destructuring-bind (id kind command fixture assertion evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"fixture\": \"~A\", " command fixture)
    (format out "\"assertion\": \"~A\", " assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"checked\", \"meaning\": \"~A\"}" meaning)))

(defun write-language-expansion-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-language-expansion-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationlanguagegen.sh check\"}")))
  (format out "~%  ]"))
