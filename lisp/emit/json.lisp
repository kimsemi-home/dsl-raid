(in-package #:dslraid.emit)

(defun emit-fsm-json (fsm &optional (stream nil))
  "Emit deterministic JSON for a single FSM object."
  (let ((json (with-output-to-string (out)
                (write-fsm-json fsm out 0))))
    (if stream
        (write-string json stream)
        json)))

(defun emit-project-json (project-id project-name fsms &optional (stream nil))
  "Emit a minimal canonical Core IR document containing FSMs."
  (let ((json (with-output-to-string (out)
                (write-project-json project-id project-name fsms out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-project-json (project-id project-name fsms out)
  (format out "{~%")
  (json-field out 1 "ir_version" "0.1.0" t)
  (indent out 1)
  (format out "\"project\": {\"id\": ~A, \"name\": ~A},~%"
          (json-string project-id)
          (json-string project-name))
  (indent out 1)
  (format out "\"fsms\": [~%")
  (write-json-array fsms out 2 #'write-fsm-json)
  (format out "~%}~%"))
