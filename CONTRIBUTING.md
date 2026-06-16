# Contributing to DSLRaid

Thanks for helping build DSLRaid. The project is intentionally executable
IR-first, so the best contributions usually strengthen the typed model before
expanding the surface area around it.

## Good First Areas

- improve examples
- add typed IR fixtures
- add schema validation cases
- improve documentation
- write small analyzer checks
- add renderer interaction tests once the viewer exists

## Design Rule

If a feature changes typed IR objects, FSM semantics, composition, projection,
diagnostics, derivations, artifacts, or code generation, start with a short
design issue or RFC before implementing it.

See [ADR 0001](docs/adr/0001-layer-boundaries.md) for the layer boundaries and
[ADR 0002](docs/adr/0002-product-scope-and-risk-boundaries.md) for MVP scope,
validation, security, and storage boundaries. Use the
[Refactoring Guide](docs/refactoring.md) as the audit order once implementation
code exists.

If a feature only improves docs, examples, tests, internal implementation, or
developer experience without changing public behavior, a normal pull request is
fine.

## ADR-Required Changes

- Core IR field changes
- ID stability rule changes
- FSM composition semantic changes
- transition/guard/action semantic changes
- diagnostic code changes
- diagnostic severity policy changes
- public/private visibility changes
- provenance and trust-boundary changes
- lock file, artifact freshness, or compatibility contract changes
- generated code contract changes

## Safe Autonomous Refactoring

- UI layout
- color, icon, and panel composition
- zoom and pan UX
- minimap
- search UX
- inspector panel layout
- layout cache implementation
- SVG export implementation
- Canvas/WebGL optimization
- keyboard shortcuts
- example projects
- internal function extraction
- test fixture additions
- CLI output formatting improvements
- friendlier error messages

Short rule: renderer/UI work can move freely; Core IR, composition, validation
meaning, visibility rules, diagnostic codes, stable IDs, and codegen contracts
need an ADR.

## Development Expectations

- Keep the canonical executable IR deterministic.
- Add fixtures for new typed object kinds or composition modes.
- Add golden tests for generated output.
- Run `scripts/install-hooks.sh` once after cloning so pre-commit checks are enforced locally.
- Keep renderers, graph indexes, and code generators as consumers of IR, not
  alternate sources of truth.
- Preserve source, derivation, and artifact traceability whenever possible.

## Pull Request Checklist

- The change has a narrow purpose.
- Layer ownership is preserved: Core IR, analyzer/projection, ViewModel/layout,
  and renderer/UI remain separate.
- Public behavior is covered by tests or fixtures.
- IR changes are documented.
- Generated output is deterministic.
- Documentation examples still match the implementation.

## Current Status

The project is in design-first bootstrap mode. The highest value early
contributions are:

1. initial core/index/view schemas
2. RunScope example fixture
3. `dslraid-core` crate
4. analyzer diagnostics
5. simple interactive viewer
