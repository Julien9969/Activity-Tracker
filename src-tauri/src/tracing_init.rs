use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use crate::shared::paths;
use tracing_appender::non_blocking::WorkerGuard;

pub fn init_tracing() -> WorkerGuard {
    let log_dir = paths::get_exe_dir(None).join("logs");
    let file_appender = tracing_appender::rolling::daily(log_dir.clone(), "activity-watch.log");

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    // Console layer
    let console_layer = fmt::layer()
        .pretty()
        .with_target(false);

    // File layer
    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true);

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(
        "Tracing initialized for console and file. Log path: {}",
        log_dir.display()
    );

    guard
}