# ganddns

Dynamic DNS updater for domains managed by [Gandi](https://www.gandi.net/).

## Project overview

A CLI tool that updates DNS records on Gandi's LiveDNS API to point to the machine's current public IP address. Useful for keeping a domain pointed at a home server with a dynamic IP.

## Tech stack

- **Language:** Rust (edition 2024)
- **Build:** Cargo

## Dependencies

- `anyhow` — error propagation
- `duration-string` — human-readable duration parsing for config (e.g. `"5m"`, `"1h"`)
- `serde` / `serde_yaml` — YAML config deserialization
- `ureq` — blocking HTTP client (for Gandi LiveDNS API calls and public IP lookup)

## Build & run

```sh
cargo build
cargo run
cargo test
```

## Project structure

```
src/
  main.rs    # Entry point, config loading, main loop
  config.rs  # Config structs and YAML deserialization
  ip.rs      # Public IP fetching (ipify + fallback)
  gandi.rs   # Gandi LiveDNS API client (get/update A records)
```

## Behavior

Runs in a continuous loop:
1. Fetches the machine's public IPv4 (via ipify, with fallback to ifconfig.me)
2. Compares against a cached IP — if unchanged, skips all Gandi API calls
3. If the IP changed, for each configured DNS record: fetches the current Gandi A record and updates it if different
4. Sleeps for the configured `fetch_rate` duration

## Configuration

YAML config file with the following structure:

- **Gandi API** — API key (PAT) and base URL for LiveDNS
- **Records** — list of A records to update (domain + record name)
- **Fetch rate** — how often to check the public IP (parsed via `duration-string`, e.g. `"5m"`, `"1h30m"`)

Example:

```yaml
api_key: "your-gandi-pat"
fetch_rate: "5m"
records:
  - domain: "example.com"
    name: "home"
  - domain: "example.com"
    name: "@"
```

## Conventions

- Follow standard Rust idioms and `clippy` lints
- Use `anyhow` for error propagation — never panic, never `unwrap()`, never `expect()`
- Keep dependencies minimal
- Always update CLAUDE.md when adding dependencies or new behavior
