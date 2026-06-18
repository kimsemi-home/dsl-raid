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

(defun append-keyword-diagnostics (ast form tail allowed diagnostics)
  (if (malformed-keyword-tail-p tail)
      (cons (keyword-diagnostic ast form :malformed-keyword-list tail)
            diagnostics)
      (append-unknown-keyword-diagnostics ast form tail allowed diagnostics)))

(defun append-unknown-keyword-diagnostics (ast form tail allowed diagnostics)
  (let ((next diagnostics))
    (loop for pair on tail by #'cddr
          for key = (first pair)
          when (not (member key allowed))
            do (push (keyword-diagnostic ast form :unknown-keyword key)
                     next))
    next))
