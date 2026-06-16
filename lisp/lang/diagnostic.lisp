(in-package #:dslraid.lang)

(defun language-diagnostic (code severity subject message suggestion)
  (list :code code
        :severity severity
        :subject subject
        :message message
        :suggestion suggestion))

(defun ast-form-subject (ast form)
  (format nil "~A/form:~D"
          (semantic-id "fsm" (fsm-ast-name ast))
          (dsl-form-ordinal form)))

(defun duplicate-form-message (label id)
  (format nil "Duplicate ~A form \"~A\"." label id))
