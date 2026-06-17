(in-package #:dslraid.emit)

(defun write-guard-json (guard out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"kind\": ~A"
          (json-string (guard-id guard))
          (json-string (guard-kind guard)))
  (when (guard-expression guard)
    (format out ", \"expression\": ~A"
            (json-expression (guard-expression guard))))
  (when (guard-input guard)
    (format out ", \"input\": ~A" (json-string (guard-input guard))))
  (write-defined-at-property (guard-defined-at guard) out)
  (write-tags-property (guard-tags guard) out)
  (format out "}"))

(defun write-action-json (action out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"kind\": ~A"
          (json-string (action-id action))
          (json-string (action-kind action)))
  (when (action-command action)
    (format out ", \"command\": ~A"
            (json-string (action-command action))))
  (when (action-emits action)
    (format out ", \"emits\": ~A"
            (json-string-array (action-emits action))))
  (when (action-expression action)
    (format out ", \"expression\": ~A"
            (json-expression (action-expression action))))
  (write-defined-at-property (action-defined-at action) out)
  (write-tags-property (action-tags action) out)
  (format out "}"))

(defun write-tags-property (tags out)
  (when tags
    (format out ", \"tags\": ~A" (json-string-array tags))))

(defun json-expression (expression)
  (format nil "{\"language\": \"lisp\", \"source\": ~A}"
          (json-string (expression-source expression))))

(defun expression-source (expression)
  (etypecase expression
    (string expression)
    (symbol (dslraid.ir::kebab-name expression))
    (cons (prin1-to-string expression))))
