(in-package #:dslraid.dsl)

(defmacro fsm (name &body forms)
  "Data-first surface syntax for a DSLRaid FSM.

The macro expands to a pure BUILD-FSM call. It does not write files, validate,
compose, call external commands, or mutate a global registry."
  `(build-fsm ',name ',forms))

(defmacro defdsl-fsm (name &body forms)
  `(defparameter ,name (fsm ,name ,@forms)))
