(in-package #:dslraid.agent)

(defparameter *verification-source-shape-budgets*
  '(("source-shape:line-budget" "line-budget" "source-files" "75"
     "bash scripts/check-source-lines.sh"
     ("scripts/check-source-lines.sh" ".github/workflows/ci.yml")
     "Source files stay within the 75-line cognitive window.")
    ("source-shape:quality-entrypoint" "surface-boundary" "local-quality" "single-command"
     "cargo run -p dslraid-cli -- quality"
     ("crates/dslraid-cli/src/commands/quality/lisp/scripts.rs"
      "crates/dslraid-cli/src/commands/quality/lisp/scripts/runtime.rs")
     "Developers enter quality checks through a narrow public command.")
    ("source-shape:lisp-ssot" "ssot-boundary" "verification-dsl" "lisp-form"
     "bash scripts/verificationdocgen.sh check"
     ("lisp/dslraid.asd" "docs/generated/verification-graph.md")
     "Verification meaning lives in Lisp SSOT modules, not generated output.")
    ("source-shape:generated-ownership" "generated-ownership" "derived-surfaces" "checked"
     "bash scripts/verificationconformancegen.sh check"
     ("docs/generated/verification-evidence.json")
     "Generated files keep an owner, generator, and check command.")))

(defparameter *verification-source-shape-rules*
  '(("source-shape:budget-required" "Line budget must be executable.")
    ("source-shape:evidence-linked" "Every shape budget cites evidence.")
    ("source-shape:surface-minimized" "Quality entrypoints hide internal file count.")))

(defun emit-verification-source-shape-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-source-shape out))))
    (if stream (write-string json stream) json)))

(defun write-verification-source-shape (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationsourcegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_source_shape.lisp\",~%")
  (format out "  \"source_shape_profile\": \"bounded-files-minimal-surface\",~%")
  (write-source-shape-budgets out)
  (format out ",~%")
  (write-source-shape-rules out)
  (format out "~%}~%"))

(defun write-source-shape-budgets (out)
  (format out "  \"budgets\": [~%")
  (loop for row in *verification-source-shape-budgets* for first = t then nil
        do (unless first (format out ",~%")) (write-source-shape-budget out row))
  (format out "~%  ]"))

(defun write-source-shape-budget (out row)
  (destructuring-bind (id kind scope limit command evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"scope\": \"~A\", \"limit\": \"~A\", " scope limit)
    (format out "\"command\": \"~A\", " command)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-source-shape-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-source-shape-rules* for first = t then nil
        do (unless first (format out ",~%")) (write-source-shape-rule out row))
  (format out "~%  ]"))

(defun write-source-shape-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
    (format out "\"check\": \"scripts/verificationsourcegen.sh check\"}")))
