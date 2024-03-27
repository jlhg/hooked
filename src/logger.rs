use anyhow::{Context, Result};
use std::path::Path;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub fn setup_logger(log_path: &str) -> Result<WorkerGuard> {
    let path = Path::new(log_path);
    let file_appender = tracing_appender::rolling::never(
        path.parent()
            .context("could not find parent directory of log path")?,
        path.file_name()
            .context("could not find name of log path")?,
    );
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = fmt::Subscriber::builder()
        .with_ansi(false)
        .with_max_level(Level::INFO)
        .finish()
        .with(
            fmt::Layer::default()
                .with_ansi(false)
                .with_writer(file_writer),
        );

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(guard)
}
