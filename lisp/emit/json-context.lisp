(in-package #:dslraid.emit)

(defparameter *runtime-owned-subjects*
  '("fsm:runtime"
    "fsm:agent"
    "fsm:workspace"
    "composition:runscope"))

(defun write-runtime-context (out)
  (indent out 1)
  (format out "\"contexts\": [~%")
  (indent out 2)
  (format out "{")
  (format out "\"id\": \"context:runtime\", \"name\": \"Runtime Context\", ")
  (format out "\"kind\": \"bounded_context\", ")
  (format out "\"owns\": ~A" (json-string-array *runtime-owned-subjects*))
  (format out "}~%")
  (indent out 1)
  (format out "],~%"))
