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

(defun run-language-guard-action-kind-smoke ()
  (let* ((ast (parse-fsm-form
               'guard-action-kind-demo
               '((:state idle)
                 (:guard can-start :kind maybe)
                 (:action start-runtime :kind teleport))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics '("LANG018" "LANG018"))
    diagnostics))

(defun run-language-guard-action-expansion-smoke ()
  (let* ((fsm (build-fsm
               'guard-action-expansion
               '((:state idle)
                 (:guard can-start :kind capability :input runtime
                  :tags (policy))
                 (:action emit-started :kind emit :emits (started)
                  :tags (runtime)))))
         (guard (first (dslraid.ir:fsm-guards fsm)))
         (action (first (dslraid.ir:fsm-actions fsm))))
    (assert (string= (dslraid.ir:guard-kind guard) "capability"))
    (assert (equal (dslraid.ir:guard-tags guard) '("policy")))
    (assert (string= (dslraid.ir:action-kind action) "emit"))
    (assert (equal (dslraid.ir:action-emits action) '("started")))
    fsm))

(defun run-language-guard-action-expression-smoke ()
  (let* ((fsm (build-fsm
               'guard-action-expression
               '((:state idle)
                 (:guard can-start :expression "runtime.ready")
                 (:action record :expression "runtime.record"))))
         (json (emit-fsm-json fsm)))
    (assert (search "\"expression\": {\"language\": \"lisp\"" json))
    json))
