(in-package #:dslraid.agent)

(defparameter *verification-query-surfaces*
  '(("query-surface:policy-terminal" "query"
     "cargo run -p dslraid-cli -- query examples/runscope/runscope.raid.json 'kind=transition and requires~=policy:no_secret_leak or terminal=true' --format json"
     "examples/runscope/runscope.raid.json"
     "contains-subject:transition:runtime.running_to_completed"
     ("crates/dslraid-cli/src/commands/query/tests/values.rs")
     "Query finds policy-gated transitions without reading internal indexes.")
    ("query-surface:guard-action" "query"
     "cargo run -p dslraid-cli -- query tests/golden/query/guard-action.input.json 'kind=guard and expression.source~=ready' --format json"
     "tests/golden/query/guard-action.input.json"
     "contains-subject:guard:runtime.can_start"
     ("crates/dslraid-cli/src/commands/query/tests/item_map.rs")
     "Query can inspect guard and action semantic fields.")
    ("query-surface:lazy-diagnostics" "lazy-composition"
     "cargo run -p dslraid-cli -- compose examples/runscope/runscope.raid.json --materialize diagnostics-only --format json"
     "examples/runscope/runscope.raid.json" "compose-lazy-empty"
     ("crates/dslraid-cli/src/commands/compose/tests.rs")
     "Diagnostics-only composition stays lazy and avoids state materialization.")
    ("query-surface:lazy-focused" "lazy-composition"
     "cargo run -p dslraid-cli -- compose examples/runscope/runscope.raid.json --materialize reachable --limit 5000 --focus state:runtime.running --depth 1 --format json"
     "examples/runscope/runscope.raid.json" "compose-lazy-focused"
     ("crates/dslraid-cli/src/commands/compose/product/materialize.rs"
      "crates/dslraid-cli/src/commands/compose/product/focus.rs")
     "Focused reachable composition materializes only the requested surface.")))

(defparameter *verification-query-surface-rules*
  '(("query-surface:command-backed" "Every query surface is exercised by CLI output.")
    ("query-surface:lazy-default" "Composition surfaces must keep lazy metadata.")
    ("query-surface:surface-only" "Consumers need query and compose commands, not internals.")))

(defun emit-verification-query-surface-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-query-surface out))))
    (if stream (write-string json stream) json)))

(defun write-verification-query-surface (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationquerygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_query_surface.lisp\",~%")
  (format out "  \"query_surface_profile\": \"query-and-lazy-observable-surface\",~%")
  (write-query-surfaces out)
  (format out ",~%")
  (write-query-surface-rules out)
  (format out "~%}~%"))

(defun write-query-surfaces (out)
  (format out "  \"surfaces\": [~%")
  (loop for row in *verification-query-surfaces* for first = t then nil
        do (unless first (format out ",~%")) (write-query-surface out row))
  (format out "~%  ]"))

(defun write-query-surface (out row)
  (destructuring-bind (id kind command fixture assertion evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"command\": \"~A\", \"fixture\": \"~A\", " command fixture)
    (format out "\"assertion\": \"~A\", " assertion)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"checked\", \"meaning\": \"~A\"}" meaning)))

(defun write-query-surface-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-query-surface-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationquerygen.sh check\"}")))
  (format out "~%  ]"))
