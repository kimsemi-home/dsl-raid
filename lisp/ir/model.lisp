(in-package #:dslraid.ir)

(defstruct defined-at
  uri
  start-line
  end-line)

(defstruct (state (:constructor make-state
                   (&key id (kind "atomic") initial-p terminal-p terminal-semantics defined-at tags)))
  id
  kind
  initial-p
  terminal-p
  terminal-semantics
  defined-at
  tags)

(defstruct (event (:constructor make-event (&key id (kind "external"))))
  id
  kind)

(defstruct (transition (:constructor make-transition
                        (&key id from to on guards actions requires defined-at tags)))
  id
  from
  to
  on
  guards
  actions
  requires
  defined-at
  tags)

(defstruct (fsm (:constructor make-fsm
                 (&key id name states events transitions guards actions defined-at tags)))
  id
  name
  states
  events
  transitions
  guards
  actions
  defined-at
  tags)
