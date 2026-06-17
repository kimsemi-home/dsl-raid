(in-package #:dslraid.lang)

(defun keyword-argument-diagnostics (ast)
  "Return diagnostics for supported form keyword argument lists."
  (let ((diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (let ((allowed (allowed-keywords (dsl-form-head form)))
            (tail (keyword-tail form)))
        (when (and allowed tail)
          (setf diagnostics
                (append-keyword-diagnostics ast form tail allowed diagnostics)))))
    (nreverse diagnostics)))

(defun keyword-tail (form)
  (case (dsl-form-head form)
    ((:state :event :transition :guard :action)
     (when (primary-id-present-p form)
       (rest (dsl-form-args form))))
    (:defined-at (dsl-form-args form))
    (otherwise nil)))

(defun allowed-keywords (head)
  (case head
    (:state '(:kind :initial :terminal :terminal-semantics :defined-at :tags))
    (:event '(:kind))
    (:transition '(:from :to :on :guards :actions :requires :defined-at :tags))
    (:guard '(:kind :expression :input :defined-at :tags))
    (:action '(:kind :command :emits :expression :defined-at :tags))
    (:defined-at '(:uri :start-line :end-line))
    (otherwise nil)))

(defun append-keyword-diagnostics (ast form tail allowed diagnostics)
  (if (malformed-keyword-tail-p tail)
      (cons (keyword-diagnostic ast form :malformed-keyword-list tail)
            diagnostics)
      (append-unknown-keyword-diagnostics ast form tail allowed diagnostics)))

(defun malformed-keyword-tail-p (tail)
  (or (oddp (length tail))
      (loop for pair on tail by #'cddr
            thereis (not (keywordp (first pair))))))

(defun append-unknown-keyword-diagnostics (ast form tail allowed diagnostics)
  (let ((next diagnostics))
    (loop for pair on tail by #'cddr
          for key = (first pair)
          when (not (member key allowed))
            do (push (keyword-diagnostic ast form :unknown-keyword key)
                     next))
    next))

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
