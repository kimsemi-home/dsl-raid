# CI Strategy

DSLRaid should use the public repository advantage: GitHub-hosted Actions can
run a strict validation matrix without adding project infrastructure.

## Workflow Files

- `.github/workflows/ci.yml`: PR and main validation
- `.github/workflows/security.yml`: dependency audit and secret scanning
- `.github/workflows/golden.yml`: golden output checks
- `.github/workflows/release.yml`: tag release creation
- `.github/workflows/pages.yml`: public viewer demo deployment

## Permission Rules

Default to:

```yaml
permissions:
  contents: read
```

Only grant writes when the job needs them:

- Pages: `pages: write`, `id-token: write`
- Release: `contents: write`
- Security upload: `security-events: write`

Do not use:

- `pull_request_target`
- `write-all`
- `contents: write` in PR validation jobs
- secret-bearing jobs on untrusted PRs

## Required PR Gates

Once the Rust workspace and viewer exist, required checks should be:

- GitHub Actions workflow lint
- Rust fmt, clippy, test
- `dslraid validate`
- `dslraid compose`
- `dslraid project`
- `dslraid render`
- `dslraid trace import`
- `dslraid coverage build`
- `dslraid coverage check`
- `dslraid diff`
- `dslraid artifact verify`
- `dslraid compat check`
- `dslraid project --visibility public` leak fixture
- golden check
- viewer lint, test, build
- security audit
- secret scan

## Quality Command

The DSLRaid CLI should converge on one local and CI quality command:

```bash
cargo run -p dslraid-cli -- quality
```

That command should eventually run:

```text
validate
validate --format json
compose --check
project --check
render --check
trace import --check
coverage build --check
coverage check
diff --check
artifact verify
compat check
golden check
```

Validation reports should eventually match
`schemas/dslraid-validation.schema.json` and include proposition IDs such as
`V007`, assertion IDs such as `assertion:fsm.initial_exactly_one`, diagnostic
codes such as `FSM007`, evidence, and suggested fixes.

## Golden Tests

Recommended layout:

```text
tests/golden/
  validate/
    invalid-transition-target.input.json
    invalid-transition-target.expected.json
  assertion-registry/
    fsm.assertions.json
  validation-report/
    runscope.expected.json
  compose/
    runtime-agent.input.json
    runtime-agent.expected.json
  project/
    runtime-view.input.json
    runtime-view.expected.json
  render/
    runtime-svg.input.json
    runtime-svg.expected.svg
  trace/
    run-123.input.jsonl
    run-123.expected.json
  diff/
    base.input.json
    head.input.json
    summary.expected.md
  diagnostics/
    nondeterministic-transition.input.json
    nondeterministic-transition.expected.json
  mutation/
    missing-initial.input.json
    unknown-event.input.json
    private-visibility-leak.input.json
  artifacts/
    stale-generated-artifact.input.json
    stale-generated-artifact.expected.json
  compat/
    missing-provider-capability.input.json
    missing-provider-capability.expected.json
```

CI command:

```bash
cargo run -p dslraid-cli -- golden check tests/golden
```

Local update command:

```bash
cargo run -p dslraid-cli -- golden update tests/golden
```

## PR Review Summary

`dslraid diff --format markdown` can publish a deterministic PR summary:

```text
FSM changed:
+ 2 states
+ 3 transitions
- 1 policy edge
warning: 1 untested transition
warning: 1 stale generated artifact
warning: provider capability changed
```

The PR comment job must use minimal permissions and should not run on
`pull_request_target` without a separate security review.

## Benchmarks and Fuzzing

PR CI should stay fast and run light benchmarks only. Scheduled or nightly
workflows can run larger analyzer and composition benchmarks:

- 100 states
- 1,000 states
- 10,000 states
- lazy composition
- focus projection
- diagnostics-only composition

IR parser and validator fuzzing should cover invalid JSON, unknown refs, cyclic
hierarchies, huge compositions, unusual display text, and ID namespace edges.

## Bootstrap Behavior

The repository is currently in design bootstrap mode. Workflows therefore skip
Rust or viewer jobs when `Cargo.toml` or `apps/viewer/package-lock.json` do not
exist yet, while still enforcing JSON syntax and RunScope semantic sanity.
