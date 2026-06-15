(in-package #:dslraid.passes)

(defun normalize-fsm (fsm)
  "Return FSM with deterministic local ordering for stable codegen and JSON."
  (make-fsm :id (fsm-id fsm)
            :name (fsm-name fsm)
            :states (stable-sort (copy-list (fsm-states fsm)) #'string< :key #'state-id)
            :events (stable-sort (copy-list (fsm-events fsm)) #'string< :key #'event-id)
            :transitions (stable-sort (copy-list (fsm-transitions fsm)) #'string< :key #'transition-id)
            :guards (copy-list (fsm-guards fsm))
            :actions (copy-list (fsm-actions fsm))
            :defined-at (fsm-defined-at fsm)
            :tags (copy-list (fsm-tags fsm))))
