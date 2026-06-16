(in-package #:dslraid)

(defun run-language-smoke ()
  (let* ((ast (parse-fsm-form
               'duplicate-demo
               '((:state idle)
                 (:state idle)
                 (:unknown-form value))))
         (diagnostics (validate-fsm-ast ast)))
    (assert (equal (mapcar (lambda (diag) (getf diag :code)) diagnostics)
                   '("LANG001" "LANG004")))
    diagnostics))

(defun run-golden-smoke ()
  (let ((fsms (runscope-fsms)))
    (run-language-smoke)
    (dolist (fsm fsms)
      (assert (null (validate-fsm fsm))))
    (emit-project-json "runscope" "RunScope" fsms)))
