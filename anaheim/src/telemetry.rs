use std::time::Duration;

use anyhow::Result;
use opentelemetry::{
    global,
    sdk::{trace, trace::BatchConfig, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use tokio::task::JoinHandle;
use tonic::metadata::MetadataMap;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::util::hostname;

// TODO: Looks like by default otel.status_code = Ok is not being set. Only set on errors.

pub type TraceId = String;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub service_name: String,
    pub level: String,
    pub otlp: Option<OTLPConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OTLPConfig {
    pub enabled: bool,
    pub environment: String,
    pub collector_url: String,
    pub exporter_timeout_seconds: u64,
    pub api_key: Option<Secret<String>>,
}

// TODO: Consider using a builder style setup for telemetry
// Possibly more feature full telemetry setup: https://github.com/open-telemetry/opentelemetry-rust/issues/868
pub fn init_telemetry(config: Config) -> Result<()> {
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level)))
        .with(tracing_subscriber::fmt::layer());

    /*
     * Open Telemetry
     */
    if let Some(otlp) = config.otlp {
        /*
         * Exporter
         */
        let exporter = {
            let mut metadata = MetadataMap::with_capacity(1);
            if let Some(api_key) = &otlp.api_key {
                metadata.insert("api-key", api_key.expose_secret().parse()?);
            }

            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&otlp.collector_url)
                .with_timeout(Duration::from_secs(otlp.exporter_timeout_seconds))
                .with_metadata(metadata)
        };

        // TODO: Allow passing in additional attributes through config or programmatically
        let attributes = Resource::new(vec![
            KeyValue::new("service.name", config.service_name.clone()),
            KeyValue::new("environment", otlp.environment.clone()),
            KeyValue::new("host.name", hostname()),
        ]);

        /*
         * Tracing
         */

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(trace::config().with_resource(attributes))
            .with_batch_config(BatchConfig::default())
            .install_batch(opentelemetry::runtime::Tokio)?;

        // TODO: send logs and metrics as well

        /*
         * Init
         */

        subscriber
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();

        debug!("Initialized OpenTelemetry with config: {:?}", &otlp)
    } else {
        subscriber.init();
        debug!("Initialized telemetry logging")
    }

    Ok(())
}

pub async fn shutdown_telemetry() -> JoinHandle<()> {
    // TODO: Conditionally shutdown tracer only if one was initialized
    // This will hang if you call the blocking `shutdown_tracer_provider` in an
    // async context, because it accesses global state. The fix is to run the
    // shutdown on its own thread.
    tokio::task::spawn_blocking(global::shutdown_tracer_provider)
}

pub mod utils {
    use opentelemetry::trace::TraceContextExt;
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    pub fn current_trace_id() -> String {
        format!(
            "{:032x}",
            tracing::Span::current()
                .context()
                .span()
                .span_context()
                .trace_id()
        )
    }
}
