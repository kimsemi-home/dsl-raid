(in-package #:dslraid.lang)

(defstruct (dsl-form (:constructor make-dsl-form
                      (&key head args ordinal)))
  head
  args
  ordinal)

(defstruct (fsm-ast (:constructor make-fsm-ast
                     (&key name forms)))
  name
  forms)

(defun parse-fsm-form (name forms)
  "Parse surface FSM forms into an authoring AST.

This is intentionally before IR creation so language conformance can point at
Lisp authoring forms instead of only Canonical IR objects."
  (make-fsm-ast
   :name name
   :forms (loop for form in forms
                for ordinal from 1
                collect (parse-dsl-form form ordinal))))

(defun parse-dsl-form (form ordinal)
  (if (consp form)
      (destructuring-bind (head &rest args) form
        (make-dsl-form :head head
                       :args args
                       :ordinal ordinal))
      (make-dsl-form :head :malformed
                     :args (list form)
                     :ordinal ordinal)))
