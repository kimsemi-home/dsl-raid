(in-package #:dslraid.agent)

(defparameter *verification-migration-surfaces*
  '(("migration-surface:compat-check" "compat-check"
     "cargo run -p dslraid-cli -- compat check examples/runscope/runscope.raid.json"
     "examples/runscope/runscope.raid.json"
     "stdout:compat ok: ir_version=0.1.0 project=runscope"
     ("crates/dslraid-cli/src/commands/workspace/compat.rs")
     "Compatibility check exposes the accepted IR contract version.")
    ("migration-surface:no-op-migrate" "migration"
     "cargo run -p dslraid-cli -- migrate examples/runscope/runscope.raid.json --from 0.1.0 --to 0.1.0"
     "examples/runscope/runscope.raid.json" "json-ir-version:0.1.0"
     ("crates/dslraid-cli/src/commands/workspace/migrate.rs")
     "No-op migration is deterministic and preserves the declared IR version.")
    ("migration-surface:wrong-from" "migration-guard"
     "cargo run -p dslraid-cli -- migrate examples/runscope/runscope.raid.json --from 9.9.9 --to 9.9.9"
     "examples/runscope/runscope.raid.json" "fails:input IR version"
     ("crates/dslraid-cli/src/commands/workspace/migrate.rs")
     "Migration refuses inputs whose declared version does not match --from.")))

(defparameter *verification-migration-surface-rules*
  '(("migration-surface:compat-command" "Compatibility is checked by a CLI command.")
    ("migration-surface:no-op" "Same-version migration must stay stable.")
    ("migration-surface:guard" "Version mismatch must fail before output.")))

(defun emit-verification-migration-surface-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-migration-surface out))))
    (if stream (write-string json stream) json)))

(defun write-verification-migration-surface (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationmigrationgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_migration_surface.lisp\",~%")
  (format out "  \"migration_surface_profile\": \"versioned-command-surface\",~%")
  (write-migration-surfaces out)
  (format out ",~%")
  (write-migration-surface-rules out)
  (format out "~%}~%"))

(defun write-migration-surfaces (out)
  (format out "  \"surfaces\": [~%")
  (loop for row in *verification-migration-surfaces* for first = t then nil
        do (unless first (format out ",~%")) (write-migration-surface out row))
  (format out "~%  ]"))

(defun write-migration-surface (out row)
  (destructuring-bind (id kind command fixture assertion evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"fixture\": \"~A\", " command fixture)
    (format out "\"assertion\": \"~A\", " assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"checked\", \"meaning\": \"~A\"}" meaning)))

(defun write-migration-surface-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-migration-surface-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationmigrationgen.sh check\"}")))
  (format out "~%  ]"))
