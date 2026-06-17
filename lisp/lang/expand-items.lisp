(in-package #:dslraid.lang)

(defun expand-state (args)
  (destructuring-bind
      (id &key (kind "atomic") initial terminal terminal-semantics defined-at tags)
      args
    (make-state :id (dslraid.ir::kebab-name id)
                :kind (authoring-enum-value kind)
                :initial-p initial
                :terminal-p terminal
                :terminal-semantics
                (when terminal-semantics
                  (authoring-enum-value terminal-semantics))
                :defined-at defined-at
                :tags tags)))

(defun expand-event (args)
  (destructuring-bind (id &key (kind "external")) args
    (make-event :id (dslraid.ir::kebab-name id)
                :kind (authoring-enum-value kind))))

(defun expand-transition (args)
  (destructuring-bind
      (id &key from to on guards actions requires defined-at tags)
      args
    (make-transition :id (dslraid.ir::kebab-name id)
                     :from (dslraid.ir::kebab-name from)
                     :to (dslraid.ir::kebab-name to)
                     :on (when on (dslraid.ir::kebab-name on))
                     :guards (mapcar #'dslraid.ir::kebab-name guards)
                     :actions (mapcar #'dslraid.ir::kebab-name actions)
                     :requires requires
                     :defined-at defined-at
                     :tags tags)))

(defun expand-defined-at (args)
  (destructuring-bind (&key uri start-line end-line) args
    (make-defined-at :uri uri
                     :start-line start-line
                     :end-line end-line)))
