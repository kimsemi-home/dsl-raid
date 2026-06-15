# Validation Proposition Catalog

DSLRaid validation is not "JSON is valid." A useful validator checks structure,
meaning, composition, derivations, security, traceability, determinism, and
runtime evidence.

Validation propositions are stable product requirements. They are separate from
diagnostic codes:

- Proposition ID: stable validation rule, such as `V007`.
- Assertion ID: executable assertion definition, such as
  `assertion:fsm.initial_exactly_one`.
- Diagnostic code: emitted problem family, such as `FSM001`.
- Diagnostic instance: one concrete finding tied to subjects.

One proposition can emit multiple diagnostic instances. One diagnostic code can
be used by several related propositions only if the registry explicitly allows
it.

## Validation Flow

Use this flow everywhere:

```text
proposition
  -> assertion
  -> diagnostic
  -> design feedback
```

Definitions:

- Proposition: what must be true.
- Assertion: how to check it, with scope, predicate, severity, evidence, and
  fix template.
- Diagnostic: what failed for concrete subjects.
- Design feedback: what the viewer, CLI, PR summary, or editor shows back to
  the user.

Core IR carries meaning. Assertions verify meaning. Diagnostics explain failed
assertions. The viewer turns diagnostics and evidence into design feedback.

## Assertion Registry

Assertions are data, not hidden code. The registry should match
[dslraid-assertion.schema.json](../schemas/dslraid-assertion.schema.json).

Example:

```json
{
  "id": "assertion:fsm.initial_exactly_one",
  "proposition": "V007",
  "code": "FSM007",
  "name": "exactly-one-initial-state",
  "layer": "fsm",
  "scope": "fsm",
  "predicate": "exactly_one_initial_state",
  "severity": "error",
  "subjects": ["fsm", "state"],
  "given": ["fsm.states"],
  "check": "count(state.initial == true) == 1",
  "fail_if": "initial state count is zero or greater than one",
  "message_template": "{fsm} has {initial_count} initial states: {initial_states}.",
  "fix_template": "Mark only one state as initial."
}
```

The assertion registry can derive:

- Rust validator stubs
- CLI docs
- diagnostic code tables
- fixture templates
- viewer copy and suggestions generated from fix templates

## Validation Layers

```text
JSON Schema
  -> syntax, required fields, primitive shape, enum values, ID namespaces
Analyzer
  -> executable IR semantics and diagnostics
Composer
  -> composition validity, determinism, bounded materialization
Projector
  -> projection safety, reference integrity, stable ordering
Artifact verifier
  -> lock hashes, stale generated outputs, deterministic derivations
Security verifier
  -> public/private visibility, path and token leakage
Trace verifier
  -> runtime event mapping and FSM conformance
Determinism verifier
  -> stable order, stable hashes, stable generated output
Golden tests
  -> behavior contract
```

## Severity Defaults

Default severity should be stable unless changed through an ADR:

```text
error    = merge block
warning  = review needed
info     = contextual information
hint     = improvement suggestion
```

CI policy can then choose gates:

```bash
dslraid validate --deny warning
dslraid validate --allow info
```

## Proposition Catalog V001-V050

| ID | Layer | Proposition | Default |
| --- | --- | --- | --- |
| V001 | IR structure | Every canonical object subject ID and derived index atom ID is unique within its scope. | error |
| V002 | IR structure | Every reference target exists. | error |
| V003 | IR structure | Containment hierarchies that forbid cycles are acyclic. | error |
| V004 | IR structure | `kind` and payload shape are compatible. | error |
| V005 | IR structure | Every semantic object has a stable ID. | error |
| V006 | FSM | Every FSM has at least one state. | error |
| V007 | FSM | Every complete FSM has exactly one initial state. | error |
| V008 | FSM | `transition.from` exists inside the same FSM or allowed region. | error |
| V009 | FSM | `transition.to` exists inside the same FSM or allowed region. | error |
| V010 | FSM | `transition.on` references a defined event unless explicitly epsilon. | error |
| V011 | FSM | Terminal states have no outgoing transitions. | error |
| V012 | FSM | Every state is reachable from the initial state unless explicitly hidden by projection policy. | warning |
| V013 | FSM | Dead states are diagnostic subjects. | warning |
| V014 | FSM | Eventless transitions are explicitly marked epsilon. | error |
| V015 | FSM | Transitions from the same state handling the same event are deterministic. | error |
| V016 | Guard / Action | Guards return boolean-compatible results. | error |
| V017 | Guard / Action | Guard-referenced capabilities exist. | error |
| V018 | Guard / Action | Actions use only allowed capabilities. | error |
| V019 | Guard / Action | Action dependency graphs are acyclic. | error |
| V020 | Guard / Action | Guard evaluation order is deterministic. | error |
| V021 | Composition | Composition inputs are FSMs. | error |
| V022 | Composition | Composition targets and referenced outputs exist. | error |
| V023 | Composition | Product composition output is deterministic. | error |
| V024 | Composition | Composed state tuples are valid combinations of source FSM states. | error |
| V025 | Composition | Unreachable composed states become diagnostic subjects. | warning |
| V026 | Composition | Composition state explosion beyond threshold emits a warning or error according to policy. | warning |
| V027 | Composition | Composition policy is explicit. | error |
| V028 | Composition | Conflict resolution policy is deterministic. | error |
| V029 | Projection | Projection root exists. | error |
| V030 | Projection | Projection does not mutate source Core IR. | error |
| V031 | Projection | Projection output uses stable ordering. | error |
| V032 | Projection | Projection filters do not break reference integrity. | error |
| V033 | Traceability | Every state has a source location or an explicit generated/imported provenance exception. | warning |
| V034 | Traceability | Generated artifacts can be traced back to source subjects. | error |
| V035 | Traceability | Tests reference at least one semantic subject. | warning |
| V036 | Traceability | Orphan artifacts are diagnostic subjects. | warning |
| V037 | Traceability | Orphan tests are diagnostic subjects. | warning |
| V038 | Artifact | Artifact hash matches the current IR and derivation input hash. | error |
| V039 | Artifact | Stale artifacts are diagnostic subjects. | error |
| V040 | Artifact | Generated artifacts are deterministic. | error |
| V041 | Artifact | Artifact paths do not violate visibility policy. | error |
| V042 | Diagnostics | Registered diagnostic codes are unique. | error |
| V043 | Diagnostics | Diagnostic severity is one of `hint`, `info`, `warning`, or `error`. | error |
| V044 | Diagnostics | Diagnostic subjects exist. | error |
| V045 | Diagnostics | `error` severity causes CI failure unless explicitly downgraded by policy. | error |
| V046 | Visibility / Security | Public projections do not include secret-bearing artifacts. | error |
| V047 | Visibility / Security | Private paths are not exposed in public projections. | error |
| V048 | Visibility / Security | Token-like strings are not present in public artifacts. | error |
| V049 | Runtime Trace | Runtime trace events map to designed states, transitions, events, or artifacts. | error |
| V050 | Runtime Trace | Executed transitions do not contradict the FSM definition. | error |

## Validation Report

`dslraid validate --format json` should eventually emit a report matching
[dslraid-validation.schema.json](../schemas/dslraid-validation.schema.json).

The report should include:

- validation run metadata
- source IR path and hash
- proposition results
- assertion results with evidence and suggested fix
- related diagnostic IDs
- separate proposition and assertion summary counts by status

Status values:

- `passed`
- `failed`
- `warning`
- `skipped`
- `not_applicable`

Summary counts must state whether they are counting propositions or assertions.
Do not mix those two totals in a single flat count.

Failed assertion shape:

```json
{
  "id": "assertion:fsm.initial_exactly_one",
  "proposition": "V007",
  "code": "FSM007",
  "predicate": "exactly_one_initial_state",
  "status": "failed",
  "severity": "error",
  "subjects": ["fsm:runtime", "state:runtime.idle", "state:runtime.booting"],
  "evidence": {
    "fsm": "runtime",
    "initial_count": 2,
    "expected": 1,
    "initial_states": ["idle", "booting"]
  },
  "message": "RuntimeFSM has 2 initial states: idle, booting.",
  "suggestion": "Mark only one state as initial.",
  "diagnostics": ["diagnostic:runtime.initial_state_count"]
}
```

Viewer feedback should show severity, diagnostic code, message, subjects,
evidence, and fix suggestion. Clicking a subject should highlight it in the
Canvas and open its inspector row.

## Golden Strategy

Each proposition should have at least one fixture over time. Early fixtures can
group related propositions, but the expected report must list which proposition
was exercised.

Recommended fixture groups:

```text
tests/golden/validate/
  structure/
  fsm/
  guards-actions/
  composition/
  projection/
  traceability/
  artifacts/
  diagnostics/
  security/
  runtime-trace/
```

## Growth Path

For real agent runtimes such as Riido or `myhome-jarvis`, the catalog should
grow naturally to 150-300 propositions.

Future proposition groups:

- Coverage validation
- Capability validation
- Provider compatibility validation
- Protocol validation
- Performance validation
- Layout validation
- Governance validation
- ADR validation

Keep the numbering stable. Add new ranges rather than renumbering existing
propositions.
