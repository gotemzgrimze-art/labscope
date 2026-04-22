# labscope

`labscope` is a Rust CLI for passive device telemetry, scan-session parsing, and
operator-friendly reporting.

It fits the rest of your GitHub better than another toy CLI because it bridges:

- terminal-first tooling
- ESP32 lab work
- defensive recon and telemetry workflows

The current version is intentionally local, passive, and easy to demo.

## Features

- Ingest newline-delimited JSON session logs from a file or stdin
- Filter by device, event, severity, channel, tag, or free-text match
- Print summaries in plain text or JSON
- Tail recent matching events for quick inspection
- Export a styled static HTML report
- Keep the event model flexible enough for future ESP32 serial ingestion

## Commands

```bash
cargo run -- ingest examples/session.jsonl
cargo run -- ingest examples/session.jsonl --json
cargo run -- summary examples/session.jsonl --device esp32-lab-01
cargo run -- summary examples/session.jsonl --severity warn --json
cargo run -- tail examples/session.jsonl --lines 5 --contains battery
cargo run -- export-html examples/session.jsonl report.html
cat examples/session.jsonl | cargo run -- ingest -
```

## Event Format

`labscope` expects one JSON object per line.

```json
{"ts":"2026-04-22T08:30:00Z","device_id":"esp32-lab-01","event":"wifi_scan","severity":"info","channel":1,"signal_dbm":-49,"message":"Observed 3 nearby AP beacons","tags":["scan","wifi"],"meta":{"firmware":"0.1.0","source":"serial"}}
{"ts":"2026-04-22T08:31:12Z","device_id":"esp32-lab-01","event":"status","severity":"warn","message":"Battery dipped below target threshold","tags":["battery"],"meta":{"battery_v":"3.72","source":"serial"}}
```

Fields:

- `ts`: RFC3339 UTC timestamp
- `device_id`: stable logical source id
- `event`: event name such as `wifi_scan`, `status`, or `boot`
- `severity`: `trace`, `info`, `notice`, `warn`, or `error`
- `channel`: optional radio channel
- `signal_dbm`: optional signal reading
- `message`: human-readable operator note
- `tags`: optional labels for filtering
- `meta`: optional string map for device-specific details

## Why It’s A Good First Rust Repo

- It gives you a clean, typed data model instead of loose Python dicts.
- It looks like systems tooling, not a novelty project.
- It leaves obvious room for a stronger v2:
  serial ingestion, SQLite, `ratatui`, release automation, and richer fixtures.

## Next Steps

1. Add serial-port ingestion behind a `serial` feature.
2. Add SQLite persistence and saved sessions.
3. Add a live terminal dashboard with `ratatui`.
4. Add GitHub release binaries for Linux/macOS.

## Safety

This repo is aimed at passive logging and defensive visibility on systems or
networks you own or are explicitly authorized to assess.

