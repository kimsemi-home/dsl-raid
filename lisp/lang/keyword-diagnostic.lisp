(in-package #:dslraid.lang)

(defun keyword-diagnostic (ast form key value)
  (language-diagnostic
   (language-code key)
   :error
   (ast-form-subject ast form)
   (keyword-message key value)
   (keyword-suggestion key)))

(defun keyword-message (key value)
  (case key
    (:unknown-keyword
     (format nil "Unsupported keyword argument \"~A\"." value))
    (otherwise
     (format nil "Malformed keyword argument list \"~A\"." value))))

(defun keyword-suggestion (key)
  (case key
    (:unknown-keyword "Use one of the supported keywords for this form.")
    (otherwise "Write keyword arguments as alternating :key value pairs.")))
