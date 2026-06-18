(in-package #:dslraid)

(defun run-language-keyword-smoke ()
  (let* ((ast (parse-fsm-form
               'keyword-demo
               '((:state idle :bogus t)
                 (:event happened :kind))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG014" "LANG015"))
    diagnostics))
