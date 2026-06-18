(in-package #:dslraid)

(defun run-language-smoke ()
  (let* ((ast (parse-fsm-form
               'duplicate-demo
               '((:state idle) (:state idle) (:unknown-form value))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG001" "LANG004"))
    diagnostics))
