use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Trace,
    Info,
    Notice,
    Warn,
    Error,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Info
    }
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Trace => "trace",
            Severity::Info => "info",
            Severity::Notice => "notice",
            Severity::Warn => "warn",
            Severity::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub ts: DateTime<Utc>,
    pub device_id: String,
    pub event: String,
    #[serde(default)]
    pub severity: Severity,
    #[serde(default)]
    pub channel: Option<u16>,
    #[serde(default)]
    pub signal_dbm: Option<i16>,
    pub message: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: BTreeMap<String, String>,
}

