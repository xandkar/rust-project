use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

pub struct Guard {
    _tracing_appender_guards: Vec<WorkerGuard>,
}

pub fn init<P: AsRef<Path>>(debug: bool, dir: P) -> anyhow::Result<Guard> {
    let (writer_file_debug, guard_debug) = tracing_appender::non_blocking(
        tracing_appender::rolling::hourly(&dir, "debug.log"),
    );
    let (writer_file_trace, guard_trace) = tracing_appender::non_blocking(
        tracing_appender::rolling::hourly(&dir, "trace.log"),
    );
    let layer_stderr = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_file(debug)
        .with_line_number(debug)
        // FIXME fmt::time::LocalTime::rfc_3339 prints "<unknown time>".
        //       Because the feature was disabled in time crate due to safety
        //       impossibility under multiple threads. It maybe possible that
        //       tracing-subscriber will switch to chrono instead:
        //       https://github.com/tokio-rs/tracing/issues/2080
        //.with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_filter(EnvFilter::from_default_env().add_directive(if debug {
            tracing::Level::DEBUG.into()
        } else {
            tracing::Level::INFO.into()
        }));
    let layer_file_debug = fmt::Layer::new()
        .with_writer(writer_file_debug)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()),
        );
    let layer_file_trace = fmt::Layer::new()
        .with_writer(writer_file_trace)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        );
    let subscriber = tracing_subscriber::registry()
        .with(layer_stderr)
        .with(layer_file_debug)
        .with(layer_file_trace);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(Guard {
        _tracing_appender_guards: vec![guard_debug, guard_trace],
    })
}
