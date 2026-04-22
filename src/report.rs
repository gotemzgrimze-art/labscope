use crate::model::SessionEvent;
use crate::store::SessionSummary;
use std::collections::BTreeMap;

pub fn render_summary_text(summary: &SessionSummary) -> String {
    let mut output = String::new();

    output.push_str(&format!("events: {}\n", summary.total_events));
    output.push_str(&format!("unique_devices: {}\n", summary.unique_devices));
    output.push_str(&format!(
        "unique_event_types: {}\n",
        summary.unique_event_types
    ));
    output.push_str(&format!(
        "warnings_or_errors: {}\n",
        summary.warnings_or_errors
    ));
    output.push_str(&format!(
        "out_of_order_timestamps: {}\n",
        summary.out_of_order_timestamps
    ));
    output.push_str(&format!(
        "active_span_seconds: {}\n",
        summary
            .active_span_seconds
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string())
    ));
    output.push_str(&format!(
        "first_seen: {}\n",
        summary.first_seen.as_deref().unwrap_or("n/a")
    ));
    output.push_str(&format!(
        "last_seen: {}\n",
        summary.last_seen.as_deref().unwrap_or("n/a")
    ));

    render_counts("devices", &summary.by_device, &mut output);
    render_counts("events_by_type", &summary.by_event, &mut output);
    render_counts("events_by_severity", &summary.by_severity, &mut output);
    render_counts("events_by_channel", &summary.by_channel, &mut output);

    output
}

pub fn render_tail_text(events: &[SessionEvent]) -> String {
    if events.is_empty() {
        return "no events matched\n".to_string();
    }

    let mut output = String::new();
    for event in events {
        let channel = event
            .channel
            .map(|value| format!(" ch={value}"))
            .unwrap_or_default();
        let signal = event
            .signal_dbm
            .map(|value| format!(" signal={value}dBm"))
            .unwrap_or_default();
        let tags = if event.tags.is_empty() {
            String::new()
        } else {
            format!(" tags={}", event.tags.join(","))
        };

        output.push_str(&format!(
            "[{}] {} {} {}{}{}{} - {}\n",
            event.ts.to_rfc3339(),
            event.severity.label(),
            event.device_id,
            event.event,
            channel,
            signal,
            tags,
            event.message
        ));
    }

    output
}

pub fn render_html(summary: &SessionSummary, events: &[SessionEvent]) -> String {
    let rows = events
        .iter()
        .map(|event| {
            let tags = if event.tags.is_empty() {
                "-".to_string()
            } else {
                html_escape(&event.tags.join(", "))
            };
            let meta = if event.meta.is_empty() {
                "-".to_string()
            } else {
                event.meta
                    .iter()
                    .map(|(key, value)| {
                        format!("<span class=\"meta-chip\">{}={}</span>", html_escape(key), html_escape(value))
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                event.ts.to_rfc3339(),
                html_escape(event.severity.label()),
                html_escape(&event.device_id),
                html_escape(&event.event),
                event
                    .channel
                    .map(|channel| channel.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                event
                    .signal_dbm
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                html_escape(&event.message),
                tags,
                meta
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <title>labscope report</title>
  <style>
    :root {{
      --bg: #f3f7fb;
      --panel: #ffffff;
      --panel-alt: #eef4fb;
      --border: #d8e0ea;
      --ink: #102033;
      --muted: #4c6177;
      --accent: #0f766e;
      --accent-soft: #d6f5ef;
      --warn: #b45309;
      --error: #991b1b;
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      padding: 2rem;
      background:
        radial-gradient(circle at top right, rgba(15, 118, 110, 0.12), transparent 26rem),
        linear-gradient(180deg, #f8fbfe 0%, var(--bg) 100%);
      color: var(--ink);
      font-family: 'Iosevka', 'IBM Plex Mono', ui-monospace, monospace;
    }}
    main {{
      max-width: 1200px;
      margin: 0 auto;
    }}
    h1, h2 {{
      margin: 0 0 0.5rem 0;
    }}
    p {{
      margin: 0;
      color: var(--muted);
    }}
    .hero {{
      margin-bottom: 1rem;
      padding: 1.5rem;
      border: 1px solid var(--border);
      border-radius: 18px;
      background: linear-gradient(135deg, rgba(15, 118, 110, 0.1), rgba(255, 255, 255, 0.95));
    }}
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
      gap: 1rem;
      margin-bottom: 1rem;
    }}
    .card {{
      border: 1px solid var(--border);
      border-radius: 18px;
      background: var(--panel);
      padding: 1rem 1.1rem;
      box-shadow: 0 10px 30px rgba(16, 32, 51, 0.05);
    }}
    .stat {{
      font-size: 1.8rem;
      font-weight: 700;
      color: var(--accent);
    }}
    .list {{
      display: grid;
      gap: 0.45rem;
      padding-left: 1rem;
      margin: 0;
    }}
    table {{
      width: 100%;
      border-collapse: collapse;
      background: var(--panel);
      border: 1px solid var(--border);
      border-radius: 18px;
      overflow: hidden;
    }}
    th, td {{
      padding: 0.8rem;
      border-bottom: 1px solid var(--border);
      text-align: left;
      vertical-align: top;
      font-size: 0.95rem;
    }}
    th {{
      background: var(--panel-alt);
    }}
    code {{
      padding: 0.15rem 0.35rem;
      border-radius: 8px;
      background: var(--accent-soft);
    }}
    .meta-chip {{
      display: inline-block;
      margin: 0 0.35rem 0.35rem 0;
      padding: 0.15rem 0.45rem;
      border-radius: 999px;
      background: var(--panel-alt);
      border: 1px solid var(--border);
    }}
    .empty {{
      padding: 1rem 0;
      color: var(--muted);
    }}
  </style>
</head>
<body>
  <main>
    <section class=\"hero\">
      <h1>labscope report</h1>
      <p>Passive session inventory for device telemetry and defensive lab workflows.</p>
    </section>

    <section class=\"grid\">
      <div class=\"card\"><div class=\"stat\">{}</div><p>Total events</p></div>
      <div class=\"card\"><div class=\"stat\">{}</div><p>Devices</p></div>
      <div class=\"card\"><div class=\"stat\">{}</div><p>Event types</p></div>
      <div class=\"card\"><div class=\"stat\">{}</div><p>Warn/Error events</p></div>
    </section>

    <section class=\"grid\">
      <div class=\"card\">
        <h2>Window</h2>
        <p>first: {}</p>
        <p>last: {}</p>
        <p>active span: {} seconds</p>
        <p>out-of-order timestamps: {}</p>
      </div>
      <div class=\"card\">
        <h2>Devices</h2>
        {}
      </div>
      <div class=\"card\">
        <h2>Event Types</h2>
        {}
      </div>
      <div class=\"card\">
        <h2>Severity</h2>
        {}
      </div>
    </section>

    <section class=\"card\">
      <h2>Channels</h2>
      {}
    </section>

    <section class=\"card\">
      <h2>Raw Events</h2>
      <table>
        <thead>
          <tr>
            <th>Timestamp</th>
            <th>Severity</th>
            <th>Device</th>
            <th>Event</th>
            <th>Channel</th>
            <th>Signal</th>
            <th>Message</th>
            <th>Tags</th>
            <th>Metadata</th>
          </tr>
        </thead>
        <tbody>
          {}
        </tbody>
      </table>
    </section>
  </main>
</body>
</html>",
        summary.total_events,
        summary.unique_devices,
        summary.unique_event_types,
        summary.warnings_or_errors,
        summary.first_seen.as_deref().unwrap_or("n/a"),
        summary.last_seen.as_deref().unwrap_or("n/a"),
        summary
            .active_span_seconds
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        summary.out_of_order_timestamps,
        render_list(&summary.by_device),
        render_list(&summary.by_event),
        render_list(&summary.by_severity),
        render_list(&summary.by_channel),
        rows
    )
}

fn render_counts(title: &str, counts: &BTreeMap<String, usize>, output: &mut String) {
    output.push_str(&format!("{title}:\n"));
    if counts.is_empty() {
        output.push_str("  - none\n");
        return;
    }

    for (key, value) in counts {
        output.push_str(&format!("  - {key}: {value}\n"));
    }
}

fn render_list(items: &BTreeMap<String, usize>) -> String {
    if items.is_empty() {
        return "<div class=\"empty\">No matching events.</div>".to_string();
    }

    let list = items
        .iter()
        .map(|(key, value)| format!("<li><code>{}</code>: {}</li>", html_escape(key), value))
        .collect::<Vec<_>>()
        .join("");

    format!("<ul class=\"list\">{list}</ul>")
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
