(in-package #:dslraid.lang)

(defun expand-fsm-ast (ast)
  "Expand a language AST into a plain DSLRaid IR FSM object."
  (let ((states '())
        (events '())
        (transitions '())
        (guards '())
        (actions '())
        (defined-at nil)
        (tags '()))
    (dolist (form (fsm-ast-forms ast))
      (ecase (dsl-form-head form)
        (:state (push (expand-state (dsl-form-args form)) states))
        (:event (push (expand-event (dsl-form-args form)) events))
        (:transition
         (push (expand-transition (dsl-form-args form)) transitions))
        (:defined-at
         (setf defined-at (expand-defined-at (dsl-form-args form))))
        (:tags
         (setf tags (mapcar #'dslraid.ir::kebab-name
                            (dsl-form-args form))))
        (:guard (push (expand-guard (dsl-form-args form)) guards))
        (:action (push (expand-action (dsl-form-args form)) actions))))
    (make-fsm :id (semantic-id "fsm" (fsm-ast-name ast))
              :name (fsm-display-name (fsm-ast-name ast))
              :states (nreverse states)
              :events (nreverse events)
              :transitions (nreverse transitions)
              :guards (nreverse guards)
              :actions (nreverse actions)
              :defined-at defined-at
              :tags (nreverse tags))))
