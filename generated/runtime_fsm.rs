// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFSMState {
    Idle,
    Starting,
    Running,
    Completed,
    Failed,
}

pub fn runtime_transition(state: RuntimeFSMState, event: Option<&str>) -> Option<RuntimeFSMState> {
    match (state, event) {
        (RuntimeFSMState::Idle, Some("start_requested")) => Some(RuntimeFSMState::Starting),
        (RuntimeFSMState::Starting, None) => Some(RuntimeFSMState::Running),
        (RuntimeFSMState::Starting, Some("start_failed")) => Some(RuntimeFSMState::Failed),
        (RuntimeFSMState::Running, None) => Some(RuntimeFSMState::Completed),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFSMState {
    Idle,
    Planning,
    Acting,
    Waiting,
    Completed,
    Failed,
}

pub fn agent_transition(state: AgentFSMState, event: Option<&str>) -> Option<AgentFSMState> {
    match (state, event) {
        (AgentFSMState::Idle, Some("plan_requested")) => Some(AgentFSMState::Planning),
        (AgentFSMState::Planning, None) => Some(AgentFSMState::Acting),
        (AgentFSMState::Acting, Some("action_completed")) => Some(AgentFSMState::Waiting),
        (AgentFSMState::Waiting, None) => Some(AgentFSMState::Completed),
        (AgentFSMState::Acting, Some("action_failed")) => Some(AgentFSMState::Failed),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceFSMState {
    Clean,
    Dirty,
    Syncing,
    Synced,
    Conflict,
}

pub fn workspace_transition(state: WorkspaceFSMState, event: Option<&str>) -> Option<WorkspaceFSMState> {
    match (state, event) {
        (WorkspaceFSMState::Clean, Some("file_changed")) => Some(WorkspaceFSMState::Dirty),
        (WorkspaceFSMState::Dirty, Some("sync_requested")) => Some(WorkspaceFSMState::Syncing),
        (WorkspaceFSMState::Syncing, Some("sync_completed")) => Some(WorkspaceFSMState::Synced),
        (WorkspaceFSMState::Syncing, Some("sync_conflict")) => Some(WorkspaceFSMState::Conflict),
        _ => None,
    }
}

