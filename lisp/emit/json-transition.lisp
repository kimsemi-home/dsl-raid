(in-package #:dslraid.emit)

(defun write-transition-json (transition out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"from\": ~A, \"to\": ~A"
          (json-string (transition-id transition))
          (json-string (transition-from transition))
          (json-string (transition-to transition)))
  (write-optional-transition-fields transition out)
  (write-defined-at-property (transition-defined-at transition) out)
  (format out "}"))

(defun write-optional-transition-fields (transition out)
  (when (transition-on transition)
    (format out ", \"on\": ~A" (json-string (transition-on transition))))
  (when (transition-guards transition)
    (format out ", \"guards\": ~A"
            (json-string-array (transition-guards transition))))
  (when (transition-actions transition)
    (format out ", \"actions\": ~A"
            (json-string-array (transition-actions transition))))
  (when (transition-requires transition)
    (format out ", \"requires\": ~A"
            (json-string-array (transition-requires transition)))))
