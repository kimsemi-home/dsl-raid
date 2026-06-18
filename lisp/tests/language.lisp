(in-package #:dslraid)

(defun assert-language-codes (diagnostics expected)
  (assert (equal (mapcar (lambda (diag) (getf diag :code)) diagnostics)
                 expected)))
