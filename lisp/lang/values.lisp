(in-package #:dslraid.lang)

(defparameter *boolean-keywords*
  '(:initial :terminal))

(defun value-diagnostics (ast)
  "Return diagnostics for supported keyword values before IR expansion."
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (let ((tail (keyword-tail form)))
        (when (and tail (not (malformed-keyword-tail-p tail)))
          (setf diagnostics
                (append-value-diagnostics ast form tail diagnostics)))))
    (nreverse diagnostics)))

(defun append-value-diagnostics (ast form tail diagnostics)
  (let ((next diagnostics))
    (loop for pair on tail by #'cddr
          for key = (first pair)
          for value = (second pair)
          when (invalid-boolean-value-p key value)
            do (push (value-diagnostic ast form key value) next))
    (append-collection-value-diagnostics ast form tail next)))

(defun invalid-boolean-value-p (key value)
  (and (member key *boolean-keywords*)
       (not (boolean-value-p value))))

(defun boolean-value-p (value)
  (or (eq value t) (null value)))

(defun value-diagnostic (ast form key value)
  (language-diagnostic
   (language-code :invalid-boolean-value)
   :error
   (ast-form-subject ast form)
   (value-message key value)
   "Use t or nil for boolean authoring flags."))

(defun value-message (key value)
  (format nil "Keyword ~A expects a boolean value; got \"~A\"."
          key value))
