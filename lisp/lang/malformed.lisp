(in-package #:dslraid.lang)

(defun malformed-form-diagnostics (ast)
  "Return diagnostics for surface forms the reader delivered as atoms."
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (when (eq (dsl-form-head form) :malformed)
        (push (malformed-form-diagnostic ast form) diagnostics)))
    (nreverse diagnostics)))

(defun malformed-form-diagnostic (ast form)
  (language-diagnostic
   (language-code :malformed-form)
   :error
   (ast-form-subject ast form)
   (malformed-form-message (first (dsl-form-args form)))
   "Use a list form such as (:state idle) or (:transition id ...)."))

(defun malformed-form-message (value)
  (format nil "FSM authoring form must be a list, got \"~A\"." value))
