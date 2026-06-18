// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.

package generated

func dslraidNext[S comparable](state S, event string, transitions map[S]map[string]S) (S, bool) {
	next, ok := transitions[state][event]
	if !ok {
		return state, false
	}
	return next, true
}

type RuntimeFSMState string

const (
	RuntimeFSMStateIdle      RuntimeFSMState = "idle"
	RuntimeFSMStateStarting  RuntimeFSMState = "starting"
	RuntimeFSMStateRunning   RuntimeFSMState = "running"
	RuntimeFSMStateCompleted RuntimeFSMState = "completed"
	RuntimeFSMStateFailed    RuntimeFSMState = "failed"
)

var runtimeFSMTransitions = map[RuntimeFSMState]map[string]RuntimeFSMState{
	RuntimeFSMStateIdle:     {"start_requested": RuntimeFSMStateStarting},
	RuntimeFSMStateStarting: {"": RuntimeFSMStateRunning, "start_failed": RuntimeFSMStateFailed},
	RuntimeFSMStateRunning:  {"": RuntimeFSMStateCompleted},
}

func RuntimeFSMTransition(state RuntimeFSMState, event string) (RuntimeFSMState, bool) {
	return dslraidNext(state, event, runtimeFSMTransitions)
}

type AgentFSMState string

const (
	AgentFSMStateIdle      AgentFSMState = "idle"
	AgentFSMStatePlanning  AgentFSMState = "planning"
	AgentFSMStateActing    AgentFSMState = "acting"
	AgentFSMStateWaiting   AgentFSMState = "waiting"
	AgentFSMStateCompleted AgentFSMState = "completed"
	AgentFSMStateFailed    AgentFSMState = "failed"
)

var agentFSMTransitions = map[AgentFSMState]map[string]AgentFSMState{
	AgentFSMStateIdle:     {"plan_requested": AgentFSMStatePlanning},
	AgentFSMStatePlanning: {"": AgentFSMStateActing},
	AgentFSMStateActing:   {"action_completed": AgentFSMStateWaiting, "action_failed": AgentFSMStateFailed},
	AgentFSMStateWaiting:  {"": AgentFSMStateCompleted},
}

func AgentFSMTransition(state AgentFSMState, event string) (AgentFSMState, bool) {
	return dslraidNext(state, event, agentFSMTransitions)
}

type WorkspaceFSMState string

const (
	WorkspaceFSMStateClean    WorkspaceFSMState = "clean"
	WorkspaceFSMStateDirty    WorkspaceFSMState = "dirty"
	WorkspaceFSMStateSyncing  WorkspaceFSMState = "syncing"
	WorkspaceFSMStateSynced   WorkspaceFSMState = "synced"
	WorkspaceFSMStateConflict WorkspaceFSMState = "conflict"
)

var workspaceFSMTransitions = map[WorkspaceFSMState]map[string]WorkspaceFSMState{
	WorkspaceFSMStateClean:   {"file_changed": WorkspaceFSMStateDirty},
	WorkspaceFSMStateDirty:   {"sync_requested": WorkspaceFSMStateSyncing},
	WorkspaceFSMStateSyncing: {"sync_completed": WorkspaceFSMStateSynced, "sync_conflict": WorkspaceFSMStateConflict},
}

func WorkspaceFSMTransition(state WorkspaceFSMState, event string) (WorkspaceFSMState, bool) {
	return dslraidNext(state, event, workspaceFSMTransitions)
}
