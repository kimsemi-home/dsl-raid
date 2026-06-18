(in-package #:dslraid)

(defun run-language-malformed-smoke ()
  (let* ((ast (parse-fsm-form 'malformed-demo '(idle ())))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG013" "LANG013"))
    diagnostics))
