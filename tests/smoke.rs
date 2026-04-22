use chrono::{TimeZone, Utc};
use labscope::model::{SessionEvent, Severity};
use labscope::report;
use labscope::store::{filter_events, summarize, Filter};
use std::collections::BTreeMap;

fn event(
    second: i64,
    device_id: &str,
    event: &str,
    severity: Severity,
    channel: Option<u16>,
    message: &str,
    tags: &[&str],
) -> SessionEvent {
    SessionEvent {
        ts: Utc.timestamp_opt(1_714_000_000 + second, 0).unwrap(),
        device_id: device_id.to_string(),
        event: event.to_string(),
        severity,
        channel,
        signal_dbm: Some(-48),
        message: message.to_string(),
        tags: tags.iter().map(|value| value.to_string()).collect(),
        meta: BTreeMap::from([("source".to_string(), "fixture".to_string())]),
    }
}

#[test]
fn summary_counts_devices_events_and_warnings() {
    let events = vec![
        event(
            0,
            "esp32-a",
            "wifi_scan",
            Severity::Info,
            Some(1),
            "normal scan",
            &["scan"],
        ),
        event(
            3,
            "esp32-a",
            "status",
            Severity::Warn,
            None,
            "battery low",
            &["battery"],
        ),
        event(
            8,
            "esp32-b",
            "wifi_scan",
            Severity::Error,
            Some(6),
            "radio reset",
            &["scan", "radio"],
        ),
    ];

    let summary = summarize(&events);

    assert_eq!(summary.total_events, 3);
    assert_eq!(summary.unique_devices, 2);
    assert_eq!(summary.unique_event_types, 2);
    assert_eq!(summary.warnings_or_errors, 2);
    assert_eq!(summary.by_device.get("esp32-a"), Some(&2));
    assert_eq!(summary.by_event.get("wifi_scan"), Some(&2));
    assert_eq!(summary.by_severity.get("error"), Some(&1));
    assert_eq!(summary.by_channel.get("n/a"), Some(&1));
}

#[test]
fn filter_matches_case_insensitive_fields_and_tags() {
    let events = vec![
        event(
            0,
            "esp32-a",
            "wifi_scan",
            Severity::Info,
            Some(1),
            "normal scan",
            &["scan"],
        ),
        event(
            4,
            "esp32-b",
            "status",
            Severity::Warn,
            None,
            "Battery Low",
            &["battery"],
        ),
    ];

    let filter = Filter {
        device: Some("ESP32-B".to_string()),
        event: Some("status".to_string()),
        severity: Some(Severity::Warn),
        channel: None,
        tag: Some("BATTERY".to_string()),
        contains: Some("low".to_string()),
    };

    let filtered = filter_events(&events, &filter);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].device_id, "esp32-b");
}

#[test]
fn summary_detects_out_of_order_timestamps() {
    let events = vec![
        event(
            10,
            "esp32-a",
            "wifi_scan",
            Severity::Info,
            Some(1),
            "late first",
            &[],
        ),
        event(
            5,
            "esp32-a",
            "wifi_scan",
            Severity::Info,
            Some(6),
            "out of order",
            &[],
        ),
    ];

    let summary = summarize(&events);

    assert_eq!(summary.out_of_order_timestamps, 1);
    assert_eq!(summary.active_span_seconds, Some(5));
}

#[test]
fn html_report_escapes_untrusted_content() {
    let events = vec![event(
        0,
        "esp32-<a>",
        "status",
        Severity::Info,
        None,
        "seen <script>alert(1)</script>",
        &["tag<script>"],
    )];

    let summary = summarize(&events);
    let html = report::render_html(&summary, &events);

    assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!html.contains("<script>alert(1)</script>"));
}
