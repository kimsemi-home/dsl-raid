(in-package #:dslraid.emit)

(defun write-defined-at-json (defined-at out)
  (format out "{\"uri\": ~A" (json-string (defined-at-uri defined-at)))
  (when (or (defined-at-start-line defined-at)
            (defined-at-end-line defined-at))
    (format out ", \"range\": {")
    (write-source-range defined-at out)
    (format out "}"))
  (format out "}"))

(defun write-source-range (defined-at out)
  (let ((fields (source-range-fields defined-at)))
    (loop for (key value) in fields
          for index from 0
          do (progn
               (when (> index 0) (format out ", "))
               (format out "\"~A\": ~D" key value)))))

(defun source-range-fields (defined-at)
  (remove nil
          (list (when (defined-at-start-line defined-at)
                  (list "start_line" (defined-at-start-line defined-at)))
                (when (defined-at-end-line defined-at)
                  (list "end_line" (defined-at-end-line defined-at))))))

(defun write-defined-at-property (defined-at out)
  (when defined-at
    (format out ", \"defined_at\": ")
    (write-defined-at-json defined-at out)))
