(in-package #:dslraid.expansion)

(defun normalize-fsm (fsm)
  "Return FSM with deterministic authoring order for canonical IR emission."
  (make-fsm :id (fsm-id fsm)
            :name (fsm-name fsm)
            :states (copy-list (fsm-states fsm))
            :events (copy-list (fsm-events fsm))
            :transitions (copy-list (fsm-transitions fsm))
            :guards (copy-list (fsm-guards fsm))
            :actions (copy-list (fsm-actions fsm))
            :defined-at (fsm-defined-at fsm)
            :tags (copy-list (fsm-tags fsm))))
