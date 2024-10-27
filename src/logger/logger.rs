use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() -> (WorkerGuard, WorkerGuard) {
    let time_format = time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]"
    );
    let timer = fmt::time::UtcTime::new(time_format);
    let (file_none_blocking, file_guard) = set_file_logger();
    let (stdout_none_blocking, stdout_guard) = set_stdout_logger();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_timer(timer.clone())
                .with_writer(file_none_blocking)
                .with_ansi(false),
        )
        .with(
            fmt::layer()
                .with_writer(stdout_none_blocking)
                .with_ansi(true)
                .with_timer(timer),
        )
        .init();
    (file_guard, stdout_guard)
}

fn set_file_logger() -> (NonBlocking, WorkerGuard) {
    let file_appender = tracing_appender::rolling::daily("logs", "axum-log");
    let (non_blocking, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
        .lossy(false)
        .buffered_lines_limit(5_000)
        .finish(file_appender);
    (non_blocking, guard)
}

fn set_stdout_logger() -> (NonBlocking, WorkerGuard) {
    let (non_blocking, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
        .lossy(true)
        .buffered_lines_limit(5_000)
        .finish(std::io::stdout());
    (non_blocking, guard)
}

pub fn drop_guard(file_guard: WorkerGuard, std_out_guard: WorkerGuard) {
    drop(file_guard);
    drop(std_out_guard);
}
