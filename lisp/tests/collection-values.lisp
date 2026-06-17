(in-package #:dslraid)

(defun run-language-collection-value-smoke ()
  (let* ((ast (parse-fsm-form
               'collection-value-demo
               '((:state idle :tags runtime)
                 (:state running)
                 (:transition start
                  :from idle :to running :guards can-start
                  :actions start-runtime :requires policy-allowed))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics
                           '("LANG017" "LANG017" "LANG017" "LANG017"))
    diagnostics))
