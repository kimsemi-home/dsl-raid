(in-package #:dslraid.agent)

(defparameter *verification-provider-compat*
  '(("provider-compat:codex-runtime" "provider:codex" "runtime:codex"
     "protocol:exec-jsonl" ("exec" "reset" "workspace" "patch")
     ("exec" "reset" "workspace" "patch" "quality-gate") "compatible"
     ("docs/generated/verification-access-policy.json"
      "docs/generated/verification-reliability.json")
     "Codex runtime supports required local workspace and reset capabilities.")
    ("provider-compat:claude-code" "provider:claude-code" "runtime:claude-code"
     "protocol:stream-json" ("hooks" "workspace" "streaming")
     ("hooks" "workspace" "streaming" "memory-file") "compatible"
     ("docs/generated/verification-sidecar.json"
      "docs/generated/verification-control-plane.json")
     "Claude Code is compatible when hooks and streaming are available.")
    ("provider-compat:new-agent" "provider:candidate" "runtime:unknown"
     "protocol:unknown" ("workspace" "security-policy")
     ("workspace") "blocked"
     ("docs/generated/verification-reliability.json"
      "docs/generated/verification-debt.json")
     "Candidate providers stay blocked until capability evidence exists.")))

(defparameter *verification-provider-compat-rules*
  '(("provider-compat:requirements-covered" "Required capabilities must be supported.")
    ("provider-compat:evidence-required" "Compatibility needs generated evidence.")
    ("provider-compat:blocked-on-missing" "Missing required capability blocks use.")))

(defun emit-verification-provider-compat-json (&optional stream)
  "Emit provider and capability compatibility receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-provider-compat out))))
    (if stream (write-string json stream) json)))

(defun write-verification-provider-compat (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationprovidergen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_provider_compat.lisp\",~%")
  (format out "  \"provider_compat_profile\": \"capability-matrix\",~%")
  (write-provider-compat-records out)
  (format out ",~%")
  (write-provider-compat-rules out)
  (format out "~%}~%"))

(defun write-provider-compat-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-provider-compat*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-provider-compat-record out row))
  (format out "~%  ]"))

(defun write-provider-compat-record (out row)
  (destructuring-bind (id provider runtime protocol requires supports status evidence meaning) row
    (format out "    {\"id\": \"~A\", \"provider\": \"~A\", " id provider)
    (format out "\"runtime\": \"~A\", \"protocol\": \"~A\", " runtime protocol)
    (write-authority-list out "requires" requires)
    (format out ", ")
    (write-authority-list out "supports" supports)
    (format out ", \"status\": \"~A\", " status)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-provider-compat-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-provider-compat-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationprovidergen.sh check\"}")))
  (format out "~%  ]"))
