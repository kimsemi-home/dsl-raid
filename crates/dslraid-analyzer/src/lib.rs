use dslraid_core::{state_subject, transition_subject, CoreIr, Fsm};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const VALIDATION_VERSION: &str = "0.1.0";
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub validation_version: String,
    pub source: ValidationSource,
    pub run: ValidationRun,
    pub summary: Summary,
    pub propositions: Vec<PropositionResult>,
    pub assertions: Vec<AssertionResult>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSource {
    pub core_ir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRun {
    pub tool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub deny: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub status: String,
    pub propositions: CountSummary,
    pub assertions: CountSummary,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountSummary {
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
    pub skipped: usize,
    pub not_applicable: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropositionResult {
    pub id: String,
    pub layer: String,
    pub status: String,
    pub severity: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
    #[serde(default)]
    pub assertions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionResult {
    pub id: String,
    pub proposition: String,
    pub code: String,
    pub predicate: String,
    pub status: String,
    pub severity: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub evidence: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
}

pub type DiagnosticRef = String;

#[derive(Debug, Clone)]
pub struct ValidateOptions {
    pub source_path: String,
    pub ir_hash: Option<String>,
    pub mode: String,
    pub deny: Vec<String>,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            source_path: "<memory>".to_string(),
            ir_hash: None,
            mode: "validate".to_string(),
            deny: Vec::new(),
        }
    }
}

impl ValidationReport {
    pub fn has_blocking_errors(&self) -> bool {
        self.assertions
            .iter()
            .any(|assertion| assertion.status == "failed" && assertion.severity == "error")
    }

    pub fn has_denied_warnings(&self, denied: &[String]) -> bool {
        denied.iter().any(|severity| severity == "warning")
            && self
                .assertions
                .iter()
                .any(|assertion| assertion.status == "warning")
    }

    pub fn is_success(&self, denied: &[String]) -> bool {
        !self.has_blocking_errors() && !self.has_denied_warnings(denied)
    }
}

pub fn validate_core_ir(ir: &CoreIr, options: ValidateOptions) -> ValidationReport {
    let mut builder = ReportBuilder::new(options);

    check_unique_subjects(ir, &mut builder);
    check_reference_targets(ir, &mut builder);
    for fsm in &ir.fsms {
        check_fsm_state_count(fsm, &mut builder);
        check_initial_count(fsm, &mut builder);
        check_transition_refs(fsm, ir, &mut builder);
        check_terminal_outgoing(fsm, &mut builder);
        check_reachability(fsm, &mut builder);
        check_determinism(fsm, &mut builder);
        check_state_source_locations(fsm, &mut builder);
    }
    check_compositions(ir, &mut builder);
    check_projections(ir, &mut builder);
    check_derivations(ir, &mut builder);
    check_artifacts(ir, &mut builder);
    check_public_visibility(ir, &mut builder);

    builder.finish()
}

fn check_unique_subjects(ir: &CoreIr, builder: &mut ReportBuilder) {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();
    for subject in all_declared_subjects(ir) {
        if !seen.insert(subject.clone()) {
            duplicates.push(subject);
        }
    }
    builder.record(AssertionSpec {
        proposition: "V001",
        assertion: "assertion:ir.subject_ids_unique",
        code: "IRR001",
        layer: "ir_structure",
        predicate: "subject_ids_unique",
        severity: "error",
        status: if duplicates.is_empty() {
            "passed"
        } else {
            "failed"
        },
        subjects: duplicates.clone(),
        evidence: json!({ "duplicates": duplicates }),
        message: if duplicates.is_empty() {
            Some("All semantic subject IDs are unique.".to_string())
        } else {
            Some("Duplicate semantic subject IDs were found.".to_string())
        },
        suggestion: Some("Rename the duplicate object or assign a stable unique ID.".to_string()),
    });
}

fn check_reference_targets(ir: &CoreIr, builder: &mut ReportBuilder) {
    let subjects = ir.semantic_subjects();
    let mut missing = Vec::new();

    for context in &ir.contexts {
        for owned in &context.owns {
            if !subjects.contains(owned) {
                missing.push(json!({ "source": context.id, "reference": owned }));
            }
        }
    }
    for requirement in &ir.requirements {
        for subject in &requirement.satisfied_by {
            if !subjects.contains(subject) {
                missing.push(json!({ "source": requirement.id, "reference": subject }));
            }
        }
    }
    for capability in &ir.capabilities {
        for subject in capability.provides.iter().chain(capability.requires.iter()) {
            if !subjects.contains(subject) {
                missing.push(json!({ "source": capability.id, "reference": subject }));
            }
        }
    }
    for policy in &ir.policies {
        for subject in &policy.applies_to {
            if !subjects.contains(subject) {
                missing.push(json!({ "source": policy.id, "reference": subject }));
            }
        }
    }
    for command in &ir.commands {
        if let Some(capability) = &command.capability {
            if !subjects.contains(capability) {
                missing.push(json!({ "source": command.id, "reference": capability }));
            }
        }
    }

    let missing_subjects = missing
        .iter()
        .filter_map(|item| item.get("source").and_then(Value::as_str))
        .map(str::to_string)
        .collect();

    builder.record(AssertionSpec {
        proposition: "V002",
        assertion: "assertion:ir.reference_targets_exist",
        code: "IRR002",
        layer: "ir_structure",
        predicate: "reference_targets_exist",
        severity: "error",
        status: if missing.is_empty() {
            "passed"
        } else {
            "failed"
        },
        subjects: missing_subjects,
        evidence: json!({ "missing": missing }),
        message: if missing.is_empty() {
            Some("All semantic references resolve.".to_string())
        } else {
            Some("Some semantic references do not resolve.".to_string())
        },
        suggestion: Some(
            "Add the missing subject or update the reference to a stable existing ID.".to_string(),
        ),
    });
}

fn check_fsm_state_count(fsm: &Fsm, builder: &mut ReportBuilder) {
    builder.record(AssertionSpec {
        proposition: "V006",
        assertion: "assertion:fsm.has_state",
        code: "FSM006",
        layer: "fsm",
        predicate: "fsm_has_state",
        severity: "error",
        status: if fsm.states.is_empty() {
            "failed"
        } else {
            "passed"
        },
        subjects: vec![fsm.id.clone()],
        evidence: json!({ "fsm": fsm.id, "state_count": fsm.states.len() }),
        message: Some(format!("{} has {} states.", fsm.name, fsm.states.len())),
        suggestion: Some("Define at least one state for every complete FSM.".to_string()),
    });
}

fn check_initial_count(fsm: &Fsm, builder: &mut ReportBuilder) {
    let initial_states: Vec<String> = fsm
        .states
        .iter()
        .filter(|state| state.initial)
        .map(|state| state.id.clone())
        .collect();
    let status = if initial_states.len() == 1 {
        "passed"
    } else {
        "failed"
    };
    let mut subjects = vec![fsm.id.clone()];
    subjects.extend(
        initial_states
            .iter()
            .map(|state| state_subject(&fsm.id, state)),
    );
    builder.record(AssertionSpec {
        proposition: "V007",
        assertion: "assertion:fsm.initial_exactly_one",
        code: "FSM007",
        layer: "fsm",
        predicate: "exactly_one_initial_state",
        severity: "error",
        status,
        subjects,
        evidence: json!({
            "fsm": fsm.id,
            "initial_count": initial_states.len(),
            "expected": 1,
            "initial_states": initial_states
        }),
        message: Some(format!(
            "{} has {} initial states.",
            fsm.name,
            initial_states.len()
        )),
        suggestion: Some("Mark exactly one state as initial.".to_string()),
    });
}

fn check_transition_refs(fsm: &Fsm, ir: &CoreIr, builder: &mut ReportBuilder) {
    let states: BTreeSet<_> = fsm.states.iter().map(|state| state.id.as_str()).collect();
    let events: BTreeSet<_> = fsm.events.iter().map(|event| event.id.as_str()).collect();
    let guards: BTreeSet<_> = fsm.guards.iter().map(|guard| guard.id.as_str()).collect();
    let actions: BTreeSet<_> = fsm
        .actions
        .iter()
        .map(|action| action.id.as_str())
        .collect();
    let subjects = ir.semantic_subjects();

    let mut unknown_from = Vec::new();
    let mut unknown_to = Vec::new();
    let mut unknown_events = Vec::new();
    let mut unknown_guard_action = Vec::new();
    let mut unknown_requires = Vec::new();

    for transition in &fsm.transitions {
        let subject = transition_subject(&fsm.id, &transition.id);
        if !states.contains(transition.from.as_str()) {
            unknown_from.push(json!({ "transition": subject, "from": transition.from }));
        }
        if !states.contains(transition.to.as_str()) {
            unknown_to.push(json!({ "transition": subject, "to": transition.to }));
        }
        if let Some(event) = &transition.on {
            if !events.contains(event.as_str()) {
                unknown_events.push(json!({ "transition": subject, "event": event }));
            }
        }
        for guard in &transition.guards {
            if !guards.contains(guard.as_str()) {
                unknown_guard_action.push(json!({ "transition": subject, "guard": guard }));
            }
        }
        for action in &transition.actions {
            if !actions.contains(action.as_str()) {
                unknown_guard_action.push(json!({ "transition": subject, "action": action }));
            }
        }
        for required in &transition.requires {
            if !subjects.contains(required) {
                unknown_requires.push(json!({ "transition": subject, "requires": required }));
            }
        }
    }

    record_collection_check(
        builder,
        "V008",
        "assertion:fsm.transition_from_exists",
        "FSM008",
        "fsm",
        "transition_from_state_exists",
        "error",
        &unknown_from,
        "All transition.from states resolve inside their FSM.",
        "Some transition.from states do not resolve inside their FSM.",
        "Use a state ID defined in the same FSM.",
    );
    record_collection_check(
        builder,
        "V009",
        "assertion:fsm.transition_target_exists",
        "FSM009",
        "fsm",
        "transition_to_state_exists",
        "error",
        &unknown_to,
        "All transition.to states resolve inside their FSM.",
        "Some transition.to states do not resolve inside their FSM.",
        "Use a state ID defined in the same FSM.",
    );
    record_collection_check(
        builder,
        "V010",
        "assertion:fsm.transition_event_exists",
        "FSM010",
        "fsm",
        "transition_event_exists",
        "error",
        &unknown_events,
        "All transition events resolve inside their FSM.",
        "Some transition events do not resolve inside their FSM.",
        "Declare the event or remove the transition event reference.",
    );
    record_collection_check(
        builder,
        "V017",
        "assertion:guard.references_existing_capability",
        "GUA017",
        "guard_action",
        "guard_references_existing_capability",
        "error",
        &unknown_guard_action,
        "All guard/action references resolve inside their FSM.",
        "Some guard/action references do not resolve inside their FSM.",
        "Declare the guard or action before referencing it.",
    );
    record_collection_check(
        builder,
        "V018",
        "assertion:action.uses_allowed_capability",
        "ACT018",
        "guard_action",
        "action_uses_allowed_capability",
        "error",
        &unknown_requires,
        "All transition requirements resolve to semantic subjects.",
        "Some transition requirements do not resolve.",
        "Reference an existing policy, capability, constraint, or semantic subject.",
    );
}

fn check_terminal_outgoing(fsm: &Fsm, builder: &mut ReportBuilder) {
    let terminal: BTreeSet<_> = fsm
        .states
        .iter()
        .filter(|state| state.terminal)
        .map(|state| state.id.as_str())
        .collect();
    let outgoing: Vec<Value> = fsm
        .transitions
        .iter()
        .filter(|transition| terminal.contains(transition.from.as_str()))
        .map(|transition| {
            json!({
                "transition": transition_subject(&fsm.id, &transition.id),
                "from": transition.from,
                "to": transition.to
            })
        })
        .collect();
    record_collection_check(
        builder,
        "V011",
        "assertion:fsm.terminal_has_no_outgoing",
        "FSM011",
        "fsm",
        "terminal_has_no_outgoing",
        "error",
        &outgoing,
        "Terminal states do not have outgoing transitions.",
        "A terminal state has outgoing transitions.",
        "Move the outgoing transition to a non-terminal state or remove terminal=true.",
    );
}

fn check_reachability(fsm: &Fsm, builder: &mut ReportBuilder) {
    let initial = fsm
        .states
        .iter()
        .find(|state| state.initial)
        .map(|state| state.id.clone());
    let mut reachable = BTreeSet::new();
    if let Some(initial) = initial {
        let mut queue = VecDeque::from([initial]);
        while let Some(state) = queue.pop_front() {
            if !reachable.insert(state.clone()) {
                continue;
            }
            for transition in fsm
                .transitions
                .iter()
                .filter(|transition| transition.from == state)
            {
                queue.push_back(transition.to.clone());
            }
        }
    }
    let unreachable: Vec<String> = fsm
        .states
        .iter()
        .filter(|state| !reachable.contains(&state.id))
        .map(|state| state_subject(&fsm.id, &state.id))
        .collect();
    builder.record(AssertionSpec {
        proposition: "V012",
        assertion: "assertion:fsm.states_reachable",
        code: "FSM012",
        layer: "fsm",
        predicate: "states_reachable_from_initial",
        severity: "warning",
        status: if unreachable.is_empty() { "passed" } else { "warning" },
        subjects: unreachable.clone(),
        evidence: json!({ "fsm": fsm.id, "unreachable_states": unreachable }),
        message: Some(if unreachable.is_empty() {
            "All states are reachable from the initial state.".to_string()
        } else {
            "Some states are not reachable from the initial state.".to_string()
        }),
        suggestion: Some("Connect the state from the initial path or hide it through an explicit projection policy.".to_string()),
    });
}

fn check_determinism(fsm: &Fsm, builder: &mut ReportBuilder) {
    let mut by_key: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for transition in &fsm.transitions {
        let event = transition
            .on
            .clone()
            .unwrap_or_else(|| "epsilon".to_string());
        by_key
            .entry((transition.from.clone(), event))
            .or_default()
            .push(transition.id.clone());
    }
    let conflicts: Vec<Value> = by_key
        .into_iter()
        .filter(|(_, transitions)| transitions.len() > 1)
        .map(|((state, event), transitions)| {
            json!({
                "fsm": fsm.id,
                "state": state,
                "event": event,
                "transitions": transitions.iter().map(|transition| transition_subject(&fsm.id, transition)).collect::<Vec<_>>()
            })
        })
        .collect();
    record_collection_check(
        builder,
        "V015",
        "assertion:fsm.event_handling_deterministic",
        "FSM015",
        "fsm",
        "event_handling_deterministic",
        "error",
        &conflicts,
        "Transitions are deterministic by state and event.",
        "Multiple transitions handle the same event from the same state.",
        "Add mutually exclusive guards or merge the transitions.",
    );
}

fn check_state_source_locations(fsm: &Fsm, builder: &mut ReportBuilder) {
    let missing: Vec<String> = fsm
        .states
        .iter()
        .filter(|state| state.defined_at.is_none() && fsm.defined_at.is_none())
        .map(|state| state_subject(&fsm.id, &state.id))
        .collect();
    builder.record(AssertionSpec {
        proposition: "V033",
        assertion: "assertion:traceability.state_source_location_exists",
        code: "TRC033",
        layer: "traceability",
        predicate: "state_source_location_exists",
        severity: "warning",
        status: if missing.is_empty() {
            "passed"
        } else {
            "warning"
        },
        subjects: missing.clone(),
        evidence: json!({ "fsm": fsm.id, "states_without_source": missing }),
        message: Some(if missing.is_empty() {
            "Every state has a direct or FSM-level source location.".to_string()
        } else {
            "Some states do not have a source location.".to_string()
        }),
        suggestion: Some("Attach defined_at to each state or to the parent FSM.".to_string()),
    });
}

fn check_compositions(ir: &CoreIr, builder: &mut ReportBuilder) {
    let fsm_ids: BTreeSet<_> = ir.fsms.iter().map(|fsm| fsm.id.as_str()).collect();
    let mut bad_inputs = Vec::new();
    let mut missing_policy = Vec::new();
    for composition in &ir.compositions {
        for input in &composition.inputs {
            if !fsm_ids.contains(input.as_str()) {
                bad_inputs.push(json!({ "composition": composition.id, "input": input }));
            }
        }
        if composition.conflict_policy.is_none() {
            missing_policy.push(json!({ "composition": composition.id }));
        }
    }
    let status = if ir.compositions.is_empty() {
        "not_applicable"
    } else if bad_inputs.is_empty() {
        "passed"
    } else {
        "failed"
    };
    builder.record(AssertionSpec {
        proposition: "V021",
        assertion: "assertion:composition.inputs_are_fsms",
        code: "CMP021",
        layer: "composition",
        predicate: "composition_inputs_are_fsms",
        severity: "error",
        status,
        subjects: bad_inputs
            .iter()
            .filter_map(|item| item.get("composition").and_then(Value::as_str))
            .map(str::to_string)
            .collect(),
        evidence: json!({ "invalid_inputs": bad_inputs }),
        message: Some("Composition inputs must resolve to FSM objects.".to_string()),
        suggestion: Some("Reference fsm:* IDs in composition.inputs.".to_string()),
    });
    let status = if ir.compositions.is_empty() {
        "not_applicable"
    } else if missing_policy.is_empty() {
        "passed"
    } else {
        "failed"
    };
    builder.record(AssertionSpec {
        proposition: "V027",
        assertion: "assertion:composition.policy_explicit",
        code: "CMP027",
        layer: "composition",
        predicate: "composition_policy_explicit",
        severity: "error",
        status,
        subjects: missing_policy
            .iter()
            .filter_map(|item| item.get("composition").and_then(Value::as_str))
            .map(str::to_string)
            .collect(),
        evidence: json!({ "missing_conflict_policy": missing_policy }),
        message: Some("Composition conflict policy must be explicit.".to_string()),
        suggestion: Some(
            "Add conflict_policy with deterministic nondeterminism handling.".to_string(),
        ),
    });
}

fn check_projections(ir: &CoreIr, builder: &mut ReportBuilder) {
    let mut allowed: BTreeSet<String> = ir.fsms.iter().map(|fsm| fsm.id.clone()).collect();
    allowed.extend(
        ir.compositions
            .iter()
            .map(|composition| composition.id.clone()),
    );
    allowed.extend(ir.contexts.iter().map(|context| context.id.clone()));
    let missing: Vec<Value> = ir
        .projections
        .iter()
        .filter(|projection| !allowed.contains(&projection.source))
        .map(|projection| json!({ "projection": projection.id, "source": projection.source }))
        .collect();
    record_collection_check(
        builder,
        "V029",
        "assertion:projection.root_exists",
        "PRJ029",
        "projection",
        "projection_root_exists",
        "error",
        &missing,
        "Projection roots resolve.",
        "A projection root does not resolve.",
        "Point projection.source at an existing fsm, composition, or context.",
    );
}

fn check_derivations(ir: &CoreIr, builder: &mut ReportBuilder) {
    let known_sources: BTreeSet<String> = ir
        .fsms
        .iter()
        .map(|fsm| fsm.id.clone())
        .chain(
            ir.compositions
                .iter()
                .map(|composition| composition.id.clone()),
        )
        .chain(ir.contexts.iter().map(|context| context.id.clone()))
        .collect();
    let artifacts: BTreeSet<_> = ir
        .artifacts
        .iter()
        .map(|artifact| artifact.id.as_str())
        .collect();
    let mut broken = Vec::new();
    for derivation in &ir.derivations {
        if !known_sources.contains(&derivation.source) {
            broken.push(json!({ "derivation": derivation.id, "source": derivation.source }));
        }
        for target in &derivation.targets {
            if !artifacts.contains(target.artifact.as_str()) {
                broken.push(json!({ "derivation": derivation.id, "artifact": target.artifact }));
            }
        }
    }
    record_collection_check(
        builder,
        "V034",
        "assertion:traceability.generated_artifact_traced",
        "TRC034",
        "traceability",
        "generated_artifact_traced",
        "error",
        &broken,
        "Generated artifacts trace back to derivations.",
        "A derivation references a missing source or artifact.",
        "Add the missing source/artifact or update the derivation target.",
    );
}

fn check_artifacts(ir: &CoreIr, builder: &mut ReportBuilder) {
    let derivations: BTreeSet<_> = ir
        .derivations
        .iter()
        .map(|derivation| derivation.id.as_str())
        .collect();
    let orphan_generated: Vec<Value> = ir
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == "generated")
        .filter(|artifact| match artifact.generated_by.as_deref() {
            Some(id) => !derivations.contains(id),
            None => true,
        })
        .map(|artifact| json!({ "artifact": artifact.id, "generated_by": artifact.generated_by }))
        .collect();
    record_collection_check(
        builder,
        "V036",
        "assertion:artifact.generated_by_exists",
        "ART036",
        "artifact",
        "generated_artifact_has_derivation",
        "warning",
        &orphan_generated,
        "Generated artifacts have derivation provenance.",
        "Some generated artifacts do not trace to a derivation.",
        "Set generated_by to an existing derivation ID.",
    );
}

fn check_public_visibility(ir: &CoreIr, builder: &mut ReportBuilder) {
    let leaked: Vec<Value> = ir
        .artifacts
        .iter()
        .filter(|artifact| artifact.visibility.as_deref() == Some("secret"))
        .map(|artifact| json!({ "artifact": artifact.id, "path": artifact.path }))
        .collect();
    record_collection_check(
        builder,
        "V046",
        "assertion:security.public_projection_no_secret",
        "SEC046",
        "visibility_security",
        "public_projection_no_secret",
        "error",
        &leaked,
        "Public IR fixture has no secret-bearing artifacts.",
        "Secret-bearing artifacts are visible in a public projection candidate.",
        "Remove the secret artifact or mark it private and exclude it from public projections.",
    );
}

#[allow(clippy::too_many_arguments)]
fn record_collection_check(
    builder: &mut ReportBuilder,
    proposition: &'static str,
    assertion: &'static str,
    code: &'static str,
    layer: &'static str,
    predicate: &'static str,
    severity: &'static str,
    failures: &[Value],
    pass_message: &'static str,
    fail_message: &'static str,
    suggestion: &'static str,
) {
    builder.record(AssertionSpec {
        proposition,
        assertion,
        code,
        layer,
        predicate,
        severity,
        status: if failures.is_empty() {
            "passed"
        } else if severity == "warning" {
            "warning"
        } else {
            "failed"
        },
        subjects: failures
            .iter()
            .filter_map(|item| {
                item.get("transition")
                    .or_else(|| item.get("projection"))
                    .or_else(|| item.get("derivation"))
                    .or_else(|| item.get("artifact"))
                    .and_then(Value::as_str)
            })
            .map(str::to_string)
            .collect(),
        evidence: json!({ "failures": failures }),
        message: Some(
            if failures.is_empty() {
                pass_message
            } else {
                fail_message
            }
            .to_string(),
        ),
        suggestion: Some(suggestion.to_string()),
    });
}

fn all_declared_subjects(ir: &CoreIr) -> Vec<String> {
    let mut subjects = Vec::new();
    subjects.push(format!("project:{}", ir.project.id));
    subjects.extend(ir.contexts.iter().map(|item| item.id.clone()));
    subjects.extend(ir.requirements.iter().map(|item| item.id.clone()));
    subjects.extend(ir.capabilities.iter().map(|item| item.id.clone()));
    subjects.extend(ir.policies.iter().map(|item| item.id.clone()));
    subjects.extend(ir.commands.iter().map(|item| item.id.clone()));
    for fsm in &ir.fsms {
        subjects.push(fsm.id.clone());
        subjects.extend(
            fsm.states
                .iter()
                .map(|state| state_subject(&fsm.id, &state.id)),
        );
        subjects.extend(
            fsm.events
                .iter()
                .map(|event| dslraid_core::event_subject(&fsm.id, &event.id)),
        );
        subjects.extend(
            fsm.guards
                .iter()
                .map(|guard| dslraid_core::guard_subject(&fsm.id, &guard.id)),
        );
        subjects.extend(
            fsm.actions
                .iter()
                .map(|action| dslraid_core::action_subject(&fsm.id, &action.id)),
        );
        subjects.extend(
            fsm.transitions
                .iter()
                .map(|transition| transition_subject(&fsm.id, &transition.id)),
        );
    }
    subjects.extend(ir.compositions.iter().map(|item| item.id.clone()));
    subjects.extend(ir.projections.iter().map(|item| item.id.clone()));
    subjects.extend(ir.derivations.iter().map(|item| item.id.clone()));
    subjects.extend(ir.artifacts.iter().map(|item| item.id.clone()));
    subjects
}

#[derive(Debug)]
struct AssertionSpec {
    proposition: &'static str,
    assertion: &'static str,
    code: &'static str,
    layer: &'static str,
    predicate: &'static str,
    severity: &'static str,
    status: &'static str,
    subjects: Vec<String>,
    evidence: Value,
    message: Option<String>,
    suggestion: Option<String>,
}

struct ReportBuilder {
    options: ValidateOptions,
    propositions: Vec<PropositionResult>,
    assertions: Vec<AssertionResult>,
    diagnostics: Vec<DiagnosticRef>,
}

impl ReportBuilder {
    fn new(options: ValidateOptions) -> Self {
        Self {
            options,
            propositions: Vec::new(),
            assertions: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn record(&mut self, spec: AssertionSpec) {
        let diagnostic = if spec.status == "failed" || spec.status == "warning" {
            Some(format!(
                "diagnostic:{}",
                spec.assertion
                    .strip_prefix("assertion:")
                    .unwrap_or(spec.assertion)
                    .replace('_', "-")
            ))
        } else {
            None
        };
        let diagnostics = diagnostic.iter().cloned().collect::<Vec<_>>();
        self.diagnostics.extend(diagnostics.clone());
        self.propositions.push(PropositionResult {
            id: spec.proposition.to_string(),
            layer: spec.layer.to_string(),
            status: spec.status.to_string(),
            severity: spec.severity.to_string(),
            subjects: spec.subjects.clone(),
            diagnostics: diagnostics.clone(),
            assertions: vec![spec.assertion.to_string()],
            message: spec.message.clone(),
        });
        self.assertions.push(AssertionResult {
            id: spec.assertion.to_string(),
            proposition: spec.proposition.to_string(),
            code: spec.code.to_string(),
            predicate: spec.predicate.to_string(),
            status: spec.status.to_string(),
            severity: spec.severity.to_string(),
            subjects: spec.subjects,
            evidence: spec.evidence,
            message: spec.message,
            suggestion: spec.suggestion,
            diagnostics,
        });
    }

    fn finish(self) -> ValidationReport {
        let proposition_counts = count_propositions(&self.propositions);
        let assertion_counts = count_assertions(&self.assertions);
        let failed = self
            .assertions
            .iter()
            .any(|assertion| assertion.status == "failed" && assertion.severity == "error");
        ValidationReport {
            validation_version: VALIDATION_VERSION.to_string(),
            source: ValidationSource {
                core_ir: self.options.source_path,
                ir_hash: self.options.ir_hash,
                lock: None,
                assertions: None,
                projection: None,
            },
            run: ValidationRun {
                tool: "dslraid-cli".to_string(),
                version: Some(TOOL_VERSION.to_string()),
                mode: self.options.mode,
                deny: self.options.deny,
            },
            summary: Summary {
                status: if failed { "failed" } else { "passed" }.to_string(),
                propositions: proposition_counts,
                assertions: assertion_counts,
            },
            propositions: self.propositions,
            assertions: self.assertions,
            diagnostics: self.diagnostics,
        }
    }
}

fn count_propositions(results: &[PropositionResult]) -> CountSummary {
    let mut summary = CountSummary::default();
    for result in results {
        count_status(&mut summary, &result.status);
    }
    summary
}

fn count_assertions(results: &[AssertionResult]) -> CountSummary {
    let mut summary = CountSummary::default();
    for result in results {
        count_status(&mut summary, &result.status);
    }
    summary
}

fn count_status(summary: &mut CountSummary, status: &str) {
    match status {
        "passed" => summary.passed += 1,
        "failed" => summary.failed += 1,
        "warning" => summary.warnings += 1,
        "skipped" => summary.skipped += 1,
        "not_applicable" => summary.not_applicable += 1,
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dslraid_core::{Project, State};

    #[test]
    fn missing_initial_is_blocking() {
        let ir = CoreIr {
            ir_version: "0.1.0".to_string(),
            project: Project {
                id: "test".to_string(),
                name: "Test".to_string(),
                uid: None,
                visibility: None,
                tags: Vec::new(),
                metadata: None,
            },
            contexts: Vec::new(),
            requirements: Vec::new(),
            capabilities: Vec::new(),
            policies: Vec::new(),
            commands: Vec::new(),
            fsms: vec![Fsm {
                id: "fsm:runtime".to_string(),
                name: "Runtime".to_string(),
                context: None,
                states: vec![State {
                    id: "idle".to_string(),
                    kind: "atomic".to_string(),
                    initial: false,
                    terminal: false,
                    terminal_semantics: None,
                    defined_at: None,
                    visibility: None,
                    tags: Vec::new(),
                    metadata: None,
                }],
                events: Vec::new(),
                guards: Vec::new(),
                actions: Vec::new(),
                transitions: Vec::new(),
                defined_at: None,
                visibility: None,
                tags: Vec::new(),
                metadata: None,
            }],
            compositions: Vec::new(),
            projections: Vec::new(),
            derivations: Vec::new(),
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };
        let report = validate_core_ir(&ir, ValidateOptions::default());
        assert!(report.has_blocking_errors());
    }
}
