(in-package #:dslraid)

(defun assert-language-codes (diagnostics expected)
  (assert (equal (mapcar (lambda (diag) (getf diag :code)) diagnostics)
                 expected)))

(defun run-language-smoke ()
  (let* ((ast (parse-fsm-form
               'duplicate-demo
               '((:state idle) (:state idle) (:unknown-form value))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG001" "LANG004"))
    diagnostics))

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

(defun run-language-required-smoke ()
  (let* ((ast (parse-fsm-form
               'required-demo
               '((:state idle) (:transition broken))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG008" "LANG009"))
    diagnostics))

(defun run-language-identifier-smoke ()
  (let* ((ast (parse-fsm-form
               'identifier-demo
               '((:state :initial t) (:event)
                 (:transition :from idle :to idle))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG010" "LANG011" "LANG012"))
    diagnostics))

(defun run-language-malformed-smoke ()
  (let* ((ast (parse-fsm-form 'malformed-demo '(idle ())))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG013" "LANG013"))
    diagnostics))

(defun run-language-keyword-smoke ()
  (let* ((ast (parse-fsm-form
               'keyword-demo
               '((:state idle :bogus t)
                 (:event happened :kind))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG014" "LANG015"))
    diagnostics))

(defun run-language-value-smoke ()
  (let* ((ast (parse-fsm-form
               'value-demo
               '((:state idle :initial yes)
                 (:state done :terminal "true"))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG016" "LANG016"))
    diagnostics))

(defun run-build-fsm-conformance-smoke ()
  (let ((blocked nil))
    (handler-case
        (build-fsm 'bad-demo
                   '((:state idle)
                     (:transition broken :from idle :to missing)))
      (error ()
        (setf blocked t)))
    (assert blocked)))
