(in-package #:dslraid)

(defun run-language-source-value-smoke ()
  (let* ((ast (parse-fsm-form
               'source-value-demo
               '((:defined-at :uri 42 :start-line zero :end-line -1)
                 (:state idle :initial t))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics
                           '("LANG019" "LANG019" "LANG019"))
    diagnostics))

(defun run-language-source-required-smoke ()
  (let* ((ast (parse-fsm-form
               'source-required-demo
               '((:defined-at :start-line 1 :end-line 2)
                 (:state idle :initial t))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG019"))
    diagnostics))
