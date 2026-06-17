(in-package #:dslraid.lang)

(defun primary-id-diagnostics (ast)
  "Return diagnostics for missing primary identifiers."
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (let ((rule (primary-id-rule (dsl-form-head form))))
        (when (and rule (not (primary-id-present-p form)))
          (push (missing-primary-id-diagnostic ast form rule)
                diagnostics))))
    (nreverse diagnostics)))

(defun primary-id-present-p (form)
  (let ((args (dsl-form-args form)))
    (and args (not (keywordp (first args))))))

(defun primary-id-rule (head)
  (case head
    (:state (list :missing-state-id "state"))
    (:event (list :missing-event-id "event"))
    (:transition (list :missing-transition-id "transition"))
    (:guard (list :missing-guard-id "guard"))
    (:action (list :missing-action-id "action"))
    (otherwise nil)))

(defun missing-primary-id-diagnostic (ast form rule)
  (destructuring-bind (key label) rule
    (language-diagnostic
     (language-code key)
     :error
     (ast-form-subject ast form)
     (missing-primary-id-message label)
     (format nil "Add a stable ~A id as the first form argument." label))))

(defun missing-primary-id-message (label)
  (format nil "~A form is missing its primary id." label))
