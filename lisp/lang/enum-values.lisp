(in-package #:dslraid.lang)

(defparameter *enum-keyword-values*
  '((:state :kind ("atomic" "compound" "parallel" "history"))
    (:state :terminal-semantics
     ("success" "cancelled" "timeout" "failed" "retriable_failed"
      "policy_blocked" "degraded"))
    (:event :kind ("external" "internal" "generated" "timer" "error"))))

(defun append-enum-value-diagnostics (ast form tail diagnostics)
  (let ((next diagnostics))
    (loop for pair on tail by #'cddr
          for key = (first pair)
          for value = (second pair)
          when (invalid-enum-value-p (dsl-form-head form) key value)
            do (push (enum-value-diagnostic ast form key value) next))
    next))

(defun invalid-enum-value-p (head key value)
  (let ((allowed (enum-allowed-values head key)))
    (and allowed
         (not (valid-enum-value-p value allowed)))))

(defun enum-allowed-values (head key)
  (third (find (list head key) *enum-keyword-values*
               :test #'equal
               :key (lambda (entry) (subseq entry 0 2)))))

(defun valid-enum-value-p (value allowed)
  (let ((normalized (authoring-enum-value value)))
    (and normalized (member normalized allowed :test #'string=))))

(defun authoring-enum-value (value)
  (typecase value
    ((or string symbol) (dslraid.ir::kebab-name value))
    (otherwise nil)))

(defun enum-value-diagnostic (ast form key value)
  (language-diagnostic
   (language-code :invalid-enum-value)
   :error
   (ast-form-subject ast form)
   (enum-value-message key value)
   "Use one of the enum values defined by Core IR."))

(defun enum-value-message (key value)
  (format nil "Keyword ~A has unsupported enum value \"~A\"."
          key value))
