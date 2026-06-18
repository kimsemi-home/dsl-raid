(in-package #:dslraid.emit)

(defun write-event-json (event out level)
  (indent out level)
  (format out "{\"id\": ~A, \"kind\": ~A}"
          (json-string (event-id event))
          (json-string (event-kind event))))
