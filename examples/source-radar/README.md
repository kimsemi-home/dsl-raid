# Research source radar

This internal-productivity fixture turns curated discovery into explicit,
revalidatable engineering decisions. An aggregator can surface an idea, but the
primary source and official provider constraints determine whether DSLRaid
records an adopt, adapt, or defer decision.

The initial radar follows GeekNews items on CLI authentication, Paisa,
Devflow Native, and loop engineering to their original sources. It deliberately
defers Google device flow for Shorts because the official allowed-scope list
does not include `youtube.upload`; the desktop installed-app flow remains the
supported choice.

```sh
go run ./cmd/researchradar
cargo run -p dslraid-cli -- validate examples/source-radar/source-radar.raid.json
cargo run -p dslraid-cli -- project examples/source-radar/source-radar.raid.json --projection view:source_radar
```

The manifest contains public technical URLs and decisions only. It stores no
credentials, account identifiers, financial values, local paths, or fetched
page contents, and the verifier performs no network request or external write.
