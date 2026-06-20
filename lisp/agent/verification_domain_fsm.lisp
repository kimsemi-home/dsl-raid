(in-package #:dslraid.agent)

(defparameter *verification-domain-fsm-surfaces*
  '(("domain-fsm:lisp-dsl" "lisp-dsl" "lisp/runtime/runscope.lisp"
     ("examples/runscope/runscope.lisp.raid.json")
     "scripts/lisp-irgen.sh check" "stdout:lisp generated ir ok" nil
     ("docs/generated/lisp-pipeline.md")
     "Common Lisp Domain FSM forms are the authoring source.")
    ("domain-fsm:canonical-ir" "canonical-ir" "examples/runscope/runscope.lisp.raid.json"
     ("generated/runtime_fsm.go" "generated/runtime_fsm.rs")
     "cargo run --quiet -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json --lock examples/runscope/runscope.lock.json"
     "stdout:artifact verification passed" nil
     ("examples/runscope/runscope.lock.json")
     "Canonical IR drives runtime artifacts through the lock contract.")
    ("domain-fsm:go-runtime" "runtime-code" "examples/runscope/runscope.lisp.raid.json"
     ("generated/runtime_fsm.go")
     "cargo run --quiet -p dslraid-cli -- codegen examples/runscope/runscope.lisp.raid.json --target go"
     "stdout:package generated" nil
     ("crates/dslraid-codegen/src/code/go.rs")
     "Go runtime code is generated from Canonical IR.")
    ("domain-fsm:mermaid-doc" "human-diagram" "examples/runscope/runscope.lisp.raid.json"
     ("stdout:stateDiagram-v2")
     "cargo run --quiet -p dslraid-cli -- export mermaid examples/runscope/runscope.lisp.raid.json"
     "stdout:stateDiagram-v2" t
     ("docs/generated/backend-targets.md")
     "Mermaid is generated human documentation, never the SSOT.")))

(defparameter *verification-domain-fsm-rules*
  '(("domain-fsm:ssot" "Domain FSM Lisp forms expand to Canonical IR.")
    ("domain-fsm:runtime-generated" "Runtime code is generated from Canonical IR.")
    ("domain-fsm:diagram-lossy" "Mermaid diagrams are generated and lossy.")))

(defun emit-verification-domain-fsm-json (&optional stream)
  "Emit Domain FSM DSL to generated artifact evidence."
  (let ((json (with-output-to-string (out) (write-verification-domain-fsm out))))
    (if stream (write-string json stream) json)))

(defun write-verification-domain-fsm (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationdomainfsmgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_domain_fsm.lisp\",~%")
  (format out "  \"domain_fsm_profile\": \"lisp-fsm-to-generated-runtime-and-diagram\",~%")
  (write-domain-fsm-surfaces out)
  (format out ",~%")
  (write-domain-fsm-rules out)
  (format out "~%}~%"))

(defun write-domain-fsm-surfaces (out)
  (format out "  \"surfaces\": [~%")
  (loop for row in *verification-domain-fsm-surfaces* for first = t then nil
        do (unless first (format out ",~%")) (write-domain-fsm-surface out row))
  (format out "~%  ]"))

(defun write-domain-fsm-surface (out row)
  (destructuring-bind (id kind source generated command assertion lossy evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"source\": \"~A\", " source)
    (write-authority-list out "generated" generated)
    (format out ", \"command\": \"~A\", \"assertion\": \"~A\", " command assertion)
    (format out "\"lossy\": ~A, " (if lossy "true" "false"))
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-domain-fsm-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-domain-fsm-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationdomainfsmgen.sh check\"}")))
  (format out "~%  ]"))
