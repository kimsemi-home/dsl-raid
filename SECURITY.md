# Security Policy

DSLRaid may process source paths, artifact paths, policy names, capability
names, runtime traces, and generated outputs. Treat these as potentially
sensitive.

## Reporting

Until the project has a published security contact, open a private advisory or
contact the repository owner directly. Do not file public issues for suspected
secret leaks or unsafe generated output.

## Scope

Security reports may include:

- private or secret data leaking through public projection
- generated artifacts containing token-like values
- unsafe importer behavior
- unsafe path handling
- dependency vulnerabilities
- CI workflows that expose secrets to untrusted pull requests

## Expected Controls

- public projection must redact or reject private and secret data
- CI must not use `pull_request_target` for untrusted code
- secret-bearing artifacts must not be committed
- generated outputs should be checked with `dslraid artifact verify`
- importers should record provenance and trust classification
