(in-package #:dslraid.lang)

(defun append-source-value-diagnostics (ast form tail diagnostics)
  (if (eq (dsl-form-head form) :defined-at)
      (append-defined-at-diagnostics ast form tail diagnostics)
      diagnostics))

(defun append-defined-at-diagnostics (ast form tail diagnostics)
  (let ((next diagnostics))
    (unless (valid-source-uri-p (getf tail :uri))
      (push (source-value-diagnostic ast form :uri (getf tail :uri))
            next))
    (dolist (key '(:start-line :end-line))
      (when (and (getf tail key)
                 (not (valid-source-line-p (getf tail key))))
        (push (source-value-diagnostic ast form key (getf tail key))
              next)))
    (when (invalid-source-range-order-p tail)
      (push (source-range-order-diagnostic ast form) next))
    next))

(defun valid-source-uri-p (value)
  (and (stringp value)
       (> (length value) 0)))

(defun valid-source-line-p (value)
  (and (integerp value)
       (> value 0)))

(defun source-value-diagnostic (ast form key value)
  (language-diagnostic
   (language-code :invalid-source-location-value)
   :error
   (ast-form-subject ast form)
   (source-value-message key value)
   "Use :uri with a non-empty string and positive integer line numbers."))

(defun source-value-message (key value)
  (format nil "Source location ~A has invalid value \"~A\"."
          key value))

(defun invalid-source-range-order-p (tail)
  (and (valid-source-line-p (getf tail :start-line))
       (valid-source-line-p (getf tail :end-line))
       (> (getf tail :start-line) (getf tail :end-line))))

(defun source-range-order-diagnostic (ast form)
  (language-diagnostic
   (language-code :invalid-source-range-order)
   :error
   (ast-form-subject ast form)
   "Source location start line must not be after end line."
   "Move :start-line before :end-line or fix the source range."))
