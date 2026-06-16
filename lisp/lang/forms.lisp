(in-package #:dslraid.lang)

(defparameter *known-fsm-form-heads*
  '(:state :event :transition :defined-at :tags :guard :action))

(defun unknown-form-diagnostics (ast)
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (unless (member (dsl-form-head form)
                      *known-fsm-form-heads*)
        (push (unknown-form-diagnostic ast form) diagnostics)))
    (nreverse diagnostics)))

(defun unknown-form-diagnostic (ast form)
  (language-diagnostic
   "LANG004"
   :error
   (ast-form-subject ast form)
   (unknown-form-message (dsl-form-head form))
   "Use a supported FSM form such as :state, :event, or :transition."))

(defun unknown-form-message (head)
  (format nil "Unknown FSM authoring form \"~A\"." head))
