use std::sync::OnceLock;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::info;

static GLOBAL_SHUTDOWN_TOKEN: OnceLock<CancellationToken> = OnceLock::new();

pub async fn wait_for_termination_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

pub fn shutdown_token() -> CancellationToken {
    GLOBAL_SHUTDOWN_TOKEN
        .get_or_init(CancellationToken::new)
        .clone()
}

pub async fn shutdown_applications_on_termination_signal() {
    wait_for_termination_signal().await;
    shutdown_token().cancel();
}
