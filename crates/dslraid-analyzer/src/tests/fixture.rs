use dslraid_core::{CoreIr, Fsm, Project, State};

pub(super) fn missing_initial_ir() -> CoreIr {
    CoreIr {
        ir_version: "0.1.0".to_string(),
        project: project(),
        contexts: Vec::new(),
        requirements: Vec::new(),
        capabilities: Vec::new(),
        policies: Vec::new(),
        commands: Vec::new(),
        fsms: vec![runtime_fsm()],
        compositions: Vec::new(),
        projections: Vec::new(),
        derivations: Vec::new(),
        artifacts: Vec::new(),
        diagnostics: Vec::new(),
    }
}

fn project() -> Project {
    Project {
        id: "test".to_string(),
        name: "Test".to_string(),
        uid: None,
        visibility: None,
        tags: Vec::new(),
        metadata: None,
    }
}

fn runtime_fsm() -> Fsm {
    Fsm {
        id: "fsm:runtime".to_string(),
        name: "Runtime".to_string(),
        context: None,
        states: vec![idle_state()],
        events: Vec::new(),
        guards: Vec::new(),
        actions: Vec::new(),
        transitions: Vec::new(),
        defined_at: None,
        visibility: None,
        tags: Vec::new(),
        metadata: None,
    }
}

fn idle_state() -> State {
    State {
        id: "idle".to_string(),
        kind: "atomic".to_string(),
        initial: false,
        terminal: false,
        terminal_semantics: None,
        defined_at: None,
        visibility: None,
        tags: Vec::new(),
        metadata: None,
    }
}
