# Examples

Examples should demonstrate DSLRaid as an executable architecture browser, not
as a diagram renderer.

Planned fixture projects:

- `simple-fsm/`: smallest valid atomic FSM
- `agent-runtime/`: provider, protocol, runtime, and capability modeling
- `ci-pipeline/`: GitHub Actions-style CI FSM import
- `policy-redaction/`: public/private projection and secret redaction
- `composition-conflict/`: lazy composition and conflict diagnostics
- `runtime-trace-overlay/`: trace import, coverage overlay, and source maps
- `runscope/`: bootstrap fixture for source, artifact, trace, and lock examples
- `local-finance/`: plan-only monthly readiness and fixture evidence convergence with a fail-closed,
  no-external-write boundary for the MyHome finance workflow

Each fixture should eventually include:

- canonical `*.dslraid.json`
- optional `*.dslraid.assertions.json`
- optional `*.dslraid.lock.json`
- optional `*.dslraid.annotations.json`
- optional `*.dslraid.validation.json`
- expected diagnostics
- expected projection
- expected semantic diff
- expected generated artifact hashes
