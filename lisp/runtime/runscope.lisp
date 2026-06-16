(in-package #:dslraid)

(defparameter *runscope-runtime-fsm*
  (fsm runtime
    (:defined-at :uri "lisp/runtime/runscope.lisp" :start-line 4 :end-line 16)
    (:state idle :initial t)
    (:state starting)
    (:state running)
    (:state completed :terminal t :terminal-semantics "success")
    (:state failed :terminal t :terminal-semantics "failed")
    (:event start-requested :kind "external")
    (:event start-failed :kind "error")
    (:transition idle->starting :from idle :to starting :on start-requested)
    (:transition starting->running :from starting :to running)
    (:transition starting->failed :from starting :to failed :on start-failed)
    (:transition running->completed :from running :to completed)))

(defparameter *runscope-agent-fsm*
  (fsm agent
    (:defined-at :uri "lisp/runtime/runscope.lisp" :start-line 19 :end-line 34)
    (:state idle :initial t)
    (:state planning)
    (:state acting)
    (:state waiting)
    (:state completed :terminal t :terminal-semantics "success")
    (:state failed :terminal t :terminal-semantics "failed")
    (:event plan-requested :kind "internal")
    (:event action-completed :kind "internal")
    (:event action-failed :kind "error")
    (:transition idle->planning :from idle :to planning :on plan-requested)
    (:transition planning->acting :from planning :to acting)
    (:transition acting->waiting :from acting :to waiting :on action-completed)
    (:transition waiting->completed :from waiting :to completed)
    (:transition acting->failed :from acting :to failed :on action-failed)))

(defparameter *runscope-workspace-fsm*
  (fsm workspace
    (:defined-at :uri "lisp/runtime/runscope.lisp" :start-line 37 :end-line 51)
    (:state clean :initial t)
    (:state dirty)
    (:state syncing)
    (:state synced :terminal t :terminal-semantics "success")
    (:state conflict :terminal t :terminal-semantics "failed")
    (:event file-changed :kind "external")
    (:event sync-requested :kind "internal")
    (:event sync-completed :kind "internal")
    (:event sync-conflict :kind "error")
    (:transition clean->dirty :from clean :to dirty :on file-changed)
    (:transition dirty->syncing :from dirty :to syncing :on sync-requested)
    (:transition syncing->synced :from syncing :to synced :on sync-completed)
    (:transition syncing->conflict :from syncing :to conflict :on sync-conflict)))

(defun runscope-fsms ()
  (mapcar #'normalize-fsm
          (list *runscope-runtime-fsm*
                *runscope-agent-fsm*
                *runscope-workspace-fsm*)))
