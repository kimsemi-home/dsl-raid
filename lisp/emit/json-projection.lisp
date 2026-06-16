(in-package #:dslraid.emit)

(defparameter *default-projections*
  '(("view:runtime" "fsm:runtime" ("states" "transitions" "events" "artifacts"))
    ("view:agent" "fsm:agent" ("states" "transitions" "events" "artifacts"))
    ("view:workspace" "fsm:workspace" ("states" "transitions" "events" "artifacts"))
    ("view:runscope" "composition:runscope"
     ("states" "transitions" "events" "artifacts" "coverage"))))

(defun write-projections (out)
  (indent out 1)
  (format out "\"projections\": [~%")
  (loop for item in *default-projections*
        for last = (eq item (car (last *default-projections*)))
        do (write-projection out item last))
  (indent out 1)
  (format out "],~%"))

(defun write-projection (out item last)
  (destructuring-bind (id source show) item
    (indent out 2)
    (format out "{")
    (format out "\"id\": ~A, \"kind\": \"projection\", " (json-string id))
    (format out "\"source\": ~A, " (json-string source))
    (format out "\"show\": ~A" (json-string-array show))
    (when (string= id "view:runscope")
      (format out ", \"filters\": {\"reachable_only\": true}"))
    (format out "}~:[,~;~]~%" last)))
