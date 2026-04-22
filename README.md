# labscope

[![CI](https://github.com/gotemzgrimze-art/labscope/actions/workflows/ci.yml/badge.svg)](https://github.com/gotemzgrimze-art/labscope/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-0f766e.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94%2B-102033.svg)](https://www.rust-lang.org/)

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

## Example Output

Text summary from the checked-in fixture:

```text
events: 8
unique_devices: 2
unique_event_types: 3
warnings_or_errors: 2
out_of_order_timestamps: 0
active_span_seconds: 160
first_seen: 2026-04-22T08:30:00+00:00
last_seen: 2026-04-22T08:32:40+00:00
devices:
  - esp32-lab-01: 5
  - esp32-lab-02: 3
events_by_type:
  - boot: 2
  - status: 2
  - wifi_scan: 4
events_by_severity:
  - error: 1
  - info: 4
  - notice: 2
  - warn: 1
events_by_channel:
  - 1: 1
  - 11: 1
  - 6: 2
  - n/a: 4
```

GitHub-friendly demo assets:

- [docs/sample-summary.txt](docs/sample-summary.txt)
- [docs/sample-report.html](docs/sample-report.html)

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

## GitHub Notes

- CI runs `fmt`, `clippy`, and `test` on every push and pull request.
- `Cargo.lock` is committed because this is an application crate.
- The sample fixture and report are checked in so the repo is explorable without setup.

## Safety

This repo is aimed at passive logging and defensive visibility on systems or
networks you own or are explicitly authorized to assess.
