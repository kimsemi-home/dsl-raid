# Local finance readiness and indirect evidence contract

This public-safe example first models hash-sealed connection plans for Ledger,
Portfolio, and Revenue, then verification of four fixture or plan-only receipts
used by the MyHome repositories. The plan path enforces day 2 → day 3 → day 5
ordering and fails closed before receipt verification. It contains no runtime
values, local paths, credentials, account identifiers, merchant data, holdings,
channel IDs, or revenue figures.

~~~sh
cargo run -p dslraid-cli -- validate examples/local-finance/local-finance.raid.json
cargo run -p dslraid-cli -- project examples/local-finance/local-finance.raid.json --projection view:runtime
cargo run -p dslraid-cli -- export mermaid examples/local-finance/local-finance.raid.json
~~~

DSLRaid verifies the public contract and schedule ordering only. It does not
read credentials, activate authentication, collect data, install a scheduler,
make financial decisions, write a channel, or perform any external action.
