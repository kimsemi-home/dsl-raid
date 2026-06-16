(in-package #:dslraid.dsl)

(defun build-fsm (name forms)
  "Convert human-friendly DSL forms into a plain DSLRaid IR FSM object.

This function performs parsing only. Conformance, composition, IO, projection,
and backend codegen belong to explicit steps outside macro expansion."
  (let ((states '())
        (events '())
        (transitions '())
        (guards '())
        (actions '())
        (defined-at nil)
        (tags '()))
    (dolist (form forms)
      (destructuring-bind (head &rest args) form
        (ecase head
          (:state
           (destructuring-bind (id &key (kind "atomic") initial terminal terminal-semantics defined-at tags) args
             (push (make-state :id (dslraid.ir::kebab-name id)
                               :kind kind
                               :initial-p initial
                               :terminal-p terminal
                               :terminal-semantics terminal-semantics
                               :defined-at defined-at
                               :tags tags)
                   states)))
          (:event
           (destructuring-bind (id &key (kind "external")) args
             (push (make-event :id (dslraid.ir::kebab-name id) :kind kind) events)))
          (:transition
           (destructuring-bind (id &key from to on guards actions requires defined-at tags) args
             (push (make-transition :id (dslraid.ir::kebab-name id)
                                    :from (dslraid.ir::kebab-name from)
                                    :to (dslraid.ir::kebab-name to)
                                    :on (when on (dslraid.ir::kebab-name on))
                                    :guards (mapcar #'dslraid.ir::kebab-name guards)
                                    :actions (mapcar #'dslraid.ir::kebab-name actions)
                                    :requires requires
                                    :defined-at defined-at
                                    :tags tags)
                   transitions)))
          (:defined-at
           (destructuring-bind (&key uri start-line end-line) args
             (setf defined-at (make-defined-at :uri uri :start-line start-line :end-line end-line))))
          (:tags
           (setf tags (mapcar #'dslraid.ir::kebab-name args)))
          (:guard
           (push args guards))
          (:action
           (push args actions)))))
    (make-fsm :id (semantic-id "fsm" name)
              :name (symbol-name name)
              :states (nreverse states)
              :events (nreverse events)
              :transitions (nreverse transitions)
              :guards (nreverse guards)
              :actions (nreverse actions)
              :defined-at defined-at
              :tags (nreverse tags))))
