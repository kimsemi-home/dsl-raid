(in-package #:dslraid)

(defun run-language-reference-smoke ()
  (let* ((ast (parse-fsm-form
               'reference-demo
               '((:state idle)
                 (:event start-requested)
                 (:transition bad
                  :from missing :to running :on unknown-event))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG005" "LANG006" "LANG007"))
    diagnostics))
