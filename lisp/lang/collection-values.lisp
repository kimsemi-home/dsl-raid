(in-package #:dslraid.lang)

(defparameter *collection-keywords*
  '(:guards :actions :requires :tags))

(defun append-collection-value-diagnostics (ast form tail diagnostics)
  (let ((next diagnostics))
    (loop for pair on tail by #'cddr
          for key = (first pair)
          for value = (second pair)
          when (invalid-collection-value-p key value)
            do (push (collection-value-diagnostic ast form key value)
                     next))
    next))

(defun invalid-collection-value-p (key value)
  (and (member key *collection-keywords*)
       (not (listp value))))

(defun collection-value-diagnostic (ast form key value)
  (language-diagnostic
   (language-code :invalid-collection-value)
   :error
   (ast-form-subject ast form)
   (collection-value-message key value)
   "Use a list value such as (:guards (can-start))."))

(defun collection-value-message (key value)
  (format nil "Keyword ~A expects a list value; got \"~A\"."
          key value))
