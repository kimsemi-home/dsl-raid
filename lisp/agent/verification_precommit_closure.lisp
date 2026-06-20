(in-package #:dslraid.agent)

(defparameter *verification-precommit-commands*
  '(("precommit:go-lint" "go-lint" "bash scripts/go-lint.sh"
     "Generated Go code is linted before commit.")
    ("precommit:rust-format" "rust-format" "cargo fmt --all -- --check"
     "Rust formatting is enforced locally.")
    ("precommit:rust-clippy" "rust-clippy" "cargo clippy --workspace --all-targets -- -D warnings"
     "Rust lint warnings block local commits.")
    ("precommit:rust-test" "rust-test" "cargo test --workspace"
     "Rust tests run before local commits.")
    ("precommit:viewer-lint" "viewer-lint" "npm --prefix apps/viewer run lint"
     "Viewer type and lint checks run before commit.")
    ("precommit:viewer-test" "viewer-test" "npm --prefix apps/viewer test"
     "Viewer tests run before commit.")
    ("precommit:viewer-build" "viewer-build" "npm --prefix apps/viewer run build"
     "Viewer production build runs before commit.")
    ("precommit:quality" "quality" "cargo run -p dslraid-cli -- quality"
     "Unified DSLRaid quality gate runs before commit.")
    ("precommit:diff-check" "diff-check" "git diff --check"
     "Whitespace and patch hygiene are checked before commit.")))

(defparameter *verification-precommit-rules*
  '(("precommit:hook-installed" "Install script must set core.hooksPath.")
    ("precommit:commands-covered" "Required local commands must be in the hook.")
    ("precommit:quality-required" "The unified quality gate must be local.")
    ("precommit:private-safe" "Hook evidence must not expose local private paths.")))

(defun emit-verification-precommit-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-precommit out))))
    (if stream (write-string json stream) json)))

(defun write-verification-precommit (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationprecommitgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_precommit_closure.lisp\",~%")
  (format out "  \"precommit_profile\": \"local-quality-gate\",~%")
  (format out "  \"hook\": \".githooks/pre-commit\",~%")
  (format out "  \"install_script\": \"scripts/install-hooks.sh\",~%")
  (write-precommit-commands out) (format out ",~%")
  (write-precommit-rules out) (format out "~%}~%"))

(defun write-precommit-commands (out)
  (format out "  \"commands\": [~%")
  (loop for row in *verification-precommit-commands* for first = t then nil
        do (unless first (format out ",~%")) (write-precommit-command out row))
  (format out "~%  ]"))

(defun write-precommit-command (out row)
  (destructuring-bind (id kind command meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"meaning\": \"~A\"}" command meaning)))

(defun write-precommit-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-precommit-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationprecommitgen.sh check\"}")))
  (format out "~%  ]"))
