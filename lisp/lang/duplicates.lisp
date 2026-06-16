(in-package #:dslraid.lang)

(defun duplicate-form-diagnostics (ast head label code)
  (let ((seen (make-hash-table :test 'equal))
        (diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (when (eq (dsl-form-head form) head)
        (let ((key (form-key label form)))
          (when key
            (if (gethash key seen)
                (push (duplicate-form-diagnostic ast form label key code)
                      diagnostics)
                (setf (gethash key seen) t))))))
    (nreverse diagnostics)))

(defun form-key (label form)
  (let ((id (first (dsl-form-args form))))
    (when id
      (semantic-id label id))))

(defun duplicate-form-diagnostic (ast form label key code)
  (language-diagnostic
   code
   :error
   (ast-form-subject ast form)
   (duplicate-form-message label key)
   (format nil "Remove or rename the repeated ~A form." label)))
