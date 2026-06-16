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

(defun run-language-reference-smoke ()
  (let* ((ast (parse-fsm-form
               'reference-demo
               '((:state idle)
                 (:event start-requested)
                 (:transition bad
                  :from missing
                  :to running
                  :on unknown-event))))
         (diagnostics (validate-fsm-ast ast)))
    (assert (equal (mapcar (lambda (diag) (getf diag :code)) diagnostics)
                   '("LANG005" "LANG006" "LANG007")))
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

(defun run-golden-smoke ()
  (let ((fsms (runscope-fsms)))
    (run-language-smoke)
    (run-language-reference-smoke)
    (run-build-fsm-conformance-smoke)
    (dolist (fsm fsms)
      (assert (null (validate-fsm fsm))))
    (emit-project-json "runscope" "RunScope" fsms)))
