(in-package #:dslraid.agent)

(defparameter *verification-runtime-trace-maps*
  '(("runtime-trace:start-requested" "examples/runscope/runscope.raid.json"
     "examples/runscope/run-001.trace.json" "examples/runscope/run-001.coverage.json"
     "event:runtime.start_requested" "event:runtime.start_requested" "covered"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-conformance.json")
     "Runtime start event maps to the designed RuntimeFSM event.")
    ("runtime-trace:idle-to-starting" "examples/runscope/runscope.raid.json"
     "examples/runscope/run-001.trace.json" "examples/runscope/run-001.coverage.json"
     "transition:runtime.idle_to_starting" "transition:runtime.idle_to_starting" "covered"
     ("docs/generated/verification-control-plane.json"
      "docs/generated/verification-provider-compat.json")
     "Executed transition matches the designed idle to starting edge.")
    ("runtime-trace:running-to-completed" "examples/runscope/runscope.raid.json"
     "examples/runscope/run-001.trace.json" "examples/runscope/run-001.coverage.json"
     "transition:runtime.running_to_completed" "transition:runtime.running_to_completed"
     "covered" ("docs/generated/verification-evidence-quality.json"
                "docs/generated/verification-semantic-diff.json")
     "Runtime completion edge is covered by the imported trace.")))

(defparameter *verification-runtime-trace-rules*
  '(("runtime-trace:design-known" "Trace subjects must map to design subjects.")
    ("runtime-trace:coverage-linked" "Trace evidence must build a coverage overlay.")
    ("runtime-trace:no-contradiction" "Transition endpoints cannot contradict design IR.")))

(defun emit-verification-runtime-trace-json (&optional stream)
  "Emit runtime trace mapping receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-runtime-trace out))))
    (if stream (write-string json stream) json)))

(defun write-verification-runtime-trace (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationruntimegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_runtime_trace.lisp\",~%")
  (format out "  \"runtime_trace_profile\": \"design-coverage-overlay\",~%")
  (write-runtime-trace-maps out)
  (format out ",~%")
  (write-runtime-trace-rules out)
  (format out "~%}~%"))

(defun write-runtime-trace-maps (out)
  (format out "  \"mappings\": [~%")
  (loop for row in *verification-runtime-trace-maps*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-runtime-trace-map out row))
  (format out "~%  ]"))

(defun write-runtime-trace-map (out row)
  (destructuring-bind (id design trace coverage runtime design-subject status evidence meaning) row
    (format out "    {\"id\": \"~A\", \"design_ir\": \"~A\", " id design)
    (format out "\"trace\": \"~A\", \"coverage\": \"~A\", " trace coverage)
    (format out "\"runtime_subject\": \"~A\", \"design_subject\": \"~A\", " runtime design-subject)
    (format out "\"coverage_status\": \"~A\", " status)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-runtime-trace-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-runtime-trace-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationruntimegen.sh check\"}")))
  (format out "~%  ]"))
