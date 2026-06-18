(in-package #:dslraid)

(defun run-language-identifier-smoke ()
  (let* ((ast (parse-fsm-form
               'identifier-demo
               '((:state :initial t) (:event)
                 (:transition :from idle :to idle))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG010" "LANG011" "LANG012"))
    diagnostics))
