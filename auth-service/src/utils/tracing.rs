use std::sync::Arc;
use std::time::Duration;
use std::path::Path;
use std::fs::File;

use axum::{body::Body, extract::Request, response::Response};
use tracing::{Level, Span};
use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

// file log 
pub fn init_tracing(file_path: &str) -> Result<()> {
    let log_file = match Path::new(file_path).exists() {
        true => File::open(file_path)?,
        false => File::create(file_path)?
    };
    let file_writer = Arc::new(log_file);
    let make_writer = BoxMakeWriter::new(file_writer);

    let file_layer = fmt::Layer::default()
        .with_writer(make_writer) // Send logs to the file
        .with_target(false); // Optional: disable logging module paths

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer) // Add the filter layer to control log verbosity
        .with(file_layer) // Add the formatting layer for compact log output
        .with(ErrorLayer::default()) // Add the error layer to capture error contexts
        .init(); // Initialize the tracing subscriber

    Ok(())
}

// stdout for logging
// pub fn init_tracing() -> Result<()> {
//     // Create a formatting layer for tracing output with a compact format
//     let fmt_layer = fmt::layer().compact();
    
//     // Create a filter layer to control the verbosity of logs
//     // Try to get the filter configuration from the environment variables
//     // If it fails, default to the "info" log level
//     let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

//     // Build the tracing subscriber registry with the formatting layer,
//     // the filter layer, and the error layer for enhanced error reporting
//     tracing_subscriber::registry()
//         .with(filter_layer) // Add the filter layer to control log verbosity
//         .with(fmt_layer) // Add the formatting layer for compact log output
//         .with(ErrorLayer::default()) // Add the error layer to capture error contexts
//         .init(); // Initialize the tracing subscriber

//     Ok(())
// }

pub fn make_span_with_request_id(request: &Request<Body>) -> Span {
    let request_id = uuid::Uuid::new_v4();
    tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method = tracing::field::display(request.method()),
        uri = tracing::field::display(request.uri()),
        version = tracing::field::debug(request.version()),
        request_id = tracing::field::display(request_id),
    )
}

pub fn on_request(_request: &Request<Body>, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.as_u16();
    let status_code_class = status_code / 100;

    match status_code_class {
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
        _ => {
            tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
    };
}