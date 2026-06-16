(in-package #:dslraid.lang)

(defun transition-required-diagnostics (ast)
  "Return diagnostics for required transition slots before IR expansion."
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (when (eq (dsl-form-head form) :transition)
        (setf diagnostics
              (append-transition-required-diagnostics ast form diagnostics))))
    (nreverse diagnostics)))

(defun append-transition-required-diagnostics (ast form diagnostics)
  (destructuring-bind (id &key from to &allow-other-keys)
      (dsl-form-args form)
    (declare (ignore id))
    (let ((next diagnostics))
      (unless from
        (setf next
              (cons (missing-transition-slot-diagnostic
                     ast form :missing-transition-from "from")
                    next)))
      (unless to
        (setf next
              (cons (missing-transition-slot-diagnostic
                     ast form :missing-transition-to "to")
                    next)))
      next)))

(defun missing-transition-slot-diagnostic (ast form key slot)
  (language-diagnostic
   (language-code key)
   :error
   (ast-form-subject ast form)
   (missing-transition-slot-message slot)
   (format nil "Add :~A with a declared state id." slot)))

(defun missing-transition-slot-message (slot)
  (format nil "Transition is missing required :~A state." slot))
