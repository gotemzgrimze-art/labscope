use crate::cli::FilterArgs;
use crate::model::SessionEvent;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub device: Option<String>,
    pub event: Option<String>,
    pub severity: Option<crate::model::Severity>,
    pub channel: Option<u16>,
    pub tag: Option<String>,
    pub contains: Option<String>,
}

impl From<&FilterArgs> for Filter {
    fn from(value: &FilterArgs) -> Self {
        Self {
            device: value.device.clone(),
            event: value.event.clone(),
            severity: value.severity,
            channel: value.channel,
            tag: value.tag.clone(),
            contains: value.contains.clone(),
        }
    }
}

impl Filter {
    pub fn matches(&self, event: &SessionEvent) -> bool {
        if let Some(device) = &self.device {
            if !event.device_id.eq_ignore_ascii_case(device) {
                return false;
            }
        }

        if let Some(event_name) = &self.event {
            if !event.event.eq_ignore_ascii_case(event_name) {
                return false;
            }
        }

        if let Some(severity) = self.severity {
            if event.severity != severity {
                return false;
            }
        }

        if let Some(channel) = self.channel {
            if event.channel != Some(channel) {
                return false;
            }
        }

        if let Some(tag) = &self.tag {
            if !event.tags.iter().any(|item| item.eq_ignore_ascii_case(tag)) {
                return false;
            }
        }

        if let Some(needle) = &self.contains {
            let needle = needle.to_ascii_lowercase();
            if !searchable_blob(event).contains(&needle) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionSummary {
    pub total_events: usize,
    pub unique_devices: usize,
    pub unique_event_types: usize,
    pub warnings_or_errors: usize,
    pub out_of_order_timestamps: usize,
    pub active_span_seconds: Option<i64>,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
    pub by_device: BTreeMap<String, usize>,
    pub by_event: BTreeMap<String, usize>,
    pub by_severity: BTreeMap<String, usize>,
    pub by_channel: BTreeMap<String, usize>,
}

pub fn load_events(path: &Path) -> Result<Vec<SessionEvent>> {
    if path == Path::new("-") {
        let stdin = io::stdin();
        return parse_events(stdin.lock());
    }

    let file = File::open(path)
        .with_context(|| format!("failed to read input file: {}", path.display()))?;
    parse_events(BufReader::new(file))
}

pub fn filter_events(events: &[SessionEvent], filter: &Filter) -> Vec<SessionEvent> {
    events
        .iter()
        .filter(|event| filter.matches(event))
        .cloned()
        .collect()
}

pub fn summarize(events: &[SessionEvent]) -> SessionSummary {
    let mut by_device = BTreeMap::new();
    let mut by_event = BTreeMap::new();
    let mut by_severity = BTreeMap::new();
    let mut by_channel = BTreeMap::new();
    let mut warnings_or_errors = 0usize;
    let mut out_of_order_timestamps = 0usize;
    let mut first_ts: Option<DateTime<Utc>> = None;
    let mut last_ts: Option<DateTime<Utc>> = None;
    let mut previous_ts: Option<DateTime<Utc>> = None;

    for event in events {
        let ts = event.ts;

        *by_device.entry(event.device_id.clone()).or_insert(0) += 1;
        *by_event.entry(event.event.clone()).or_insert(0) += 1;
        *by_severity
            .entry(event.severity.label().to_string())
            .or_insert(0) += 1;
        *by_channel
            .entry(
                event
                    .channel
                    .map(|channel| channel.to_string())
                    .unwrap_or_else(|| "n/a".to_string()),
            )
            .or_insert(0) += 1;

        if matches!(
            event.severity,
            crate::model::Severity::Warn | crate::model::Severity::Error
        ) {
            warnings_or_errors += 1;
        }

        if let Some(previous) = previous_ts.as_ref() {
            if ts < *previous {
                out_of_order_timestamps += 1;
            }
        }
        previous_ts = Some(ts);

        first_ts = Some(match first_ts {
            Some(current) => current.min(ts),
            None => ts,
        });
        last_ts = Some(match last_ts {
            Some(current) => current.max(ts),
            None => ts,
        });
    }

    let active_span_seconds = match (first_ts, last_ts) {
        (Some(start), Some(end)) => Some((end - start).num_seconds()),
        _ => None,
    };

    SessionSummary {
        total_events: events.len(),
        unique_devices: by_device.len(),
        unique_event_types: by_event.len(),
        warnings_or_errors,
        out_of_order_timestamps,
        active_span_seconds,
        first_seen: first_ts.map(|ts| ts.to_rfc3339()),
        last_seen: last_ts.map(|ts| ts.to_rfc3339()),
        by_device,
        by_event,
        by_severity,
        by_channel,
    }
}

fn parse_events<R: BufRead>(reader: R) -> Result<Vec<SessionEvent>> {
    reader
        .lines()
        .enumerate()
        .filter_map(|(index, line)| match line {
            Ok(content) if content.trim().is_empty() => None,
            Ok(content) => Some((index, Ok(content))),
            Err(error) => Some((index, Err(error))),
        })
        .map(|(index, line)| {
            let line = line.with_context(|| format!("failed to read line {}", index + 1))?;
            serde_json::from_str::<SessionEvent>(&line)
                .with_context(|| format!("invalid event on line {}", index + 1))
        })
        .collect()
}

fn searchable_blob(event: &SessionEvent) -> String {
    let mut parts = vec![
        event.device_id.to_ascii_lowercase(),
        event.event.to_ascii_lowercase(),
        event.message.to_ascii_lowercase(),
    ];

    parts.extend(event.tags.iter().map(|tag| tag.to_ascii_lowercase()));
    parts.extend(
        event
            .meta
            .iter()
            .flat_map(|(key, value)| [key.to_ascii_lowercase(), value.to_ascii_lowercase()]),
    );

    parts.join(" ")
}
