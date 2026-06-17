(in-package #:dslraid)

(defun run-language-guard-action-identifier-smoke ()
  (let* ((ast (parse-fsm-form
               'guard-action-identifier-demo
               '((:state idle)
                 (:guard :kind predicate)
                 (:action :kind command))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG021" "LANG022"))
    diagnostics))

(defun run-language-guard-action-duplicate-smoke ()
  (let* ((ast (parse-fsm-form
               'guard-action-duplicate-demo
               '((:state idle)
                 (:guard can-start)
                 (:guard can-start)
                 (:action start-runtime)
                 (:action start-runtime))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG023" "LANG024"))
    diagnostics))
