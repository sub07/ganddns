# ganddns

Dynamic DNS updater for domains managed by [Gandi](https://www.gandi.net/).

Updates DNS A records on Gandi's LiveDNS API to point to the machine's current public IP address. Useful for keeping a domain pointed at a home server with a dynamic IP.

## How it works

Runs in a continuous loop:
1. Fetches the machine's public IPv4 (via ipify, with fallback to ifconfig.me)
2. Compares against a cached IP — if unchanged, skips all Gandi API calls
3. If the IP changed, for each configured DNS record: fetches the current A record and updates it if different
4. Sleeps for the configured `fetch_rate` duration

## Build & run

Requires [Rust](https://rustup.rs/).

```sh
cargo build
cargo run
```

## Configuration

Create a `config.yaml` file (see `config.example.yaml`):

```yaml
api_key: "your-gandi-pat"
fetch_rate: "5m"
records:
  - domain: "example.com"
    name: "home"
  - domain: "example.com"
    name: "@"
```

| Field | Description |
|-------|-------------|
| `api_key` | Gandi Personal Access Token (PAT) |
| `fetch_rate` | How often to check the public IP (e.g. `"5m"`, `"1h30m"`) |
| `records` | List of A records to update — `domain` is the zone, `name` is the record name (`"@"` for apex) |
