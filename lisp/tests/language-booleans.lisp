(in-package #:dslraid)

(defun run-language-value-smoke ()
  (let* ((ast (parse-fsm-form
               'value-demo
               '((:state idle :initial yes)
                 (:state done :terminal "true"))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG016" "LANG016"))
    diagnostics))
