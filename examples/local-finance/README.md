# Local finance indirect evidence contract

This public-safe example models verification of four fixture or plan-only
receipts used by the MyHome repositories. It contains no runtime values, local
paths, credentials, account identifiers, merchant data, holdings, channel IDs,
or revenue figures.

~~~sh
cargo run -p dslraid-cli -- validate examples/local-finance/local-finance.raid.json
cargo run -p dslraid-cli -- project examples/local-finance/local-finance.raid.json --projection view:runtime
cargo run -p dslraid-cli -- export mermaid examples/local-finance/local-finance.raid.json
~~~

DSLRaid verifies the public contract only. It does not own authentication,
collection, scheduling, financial decisions, channel writes, or any external
action.
