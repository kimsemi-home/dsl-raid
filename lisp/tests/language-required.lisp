(in-package #:dslraid)

(defun run-language-required-smoke ()
  (let* ((ast (parse-fsm-form
               'required-demo
               '((:state idle) (:transition broken))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG008" "LANG009"))
    diagnostics))
