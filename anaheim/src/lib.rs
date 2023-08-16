// re-exports
pub use async_trait::async_trait;
pub use derive_new::new;
pub use mockall::automock;

#[forbid(unsafe_code)]
pub mod config;
pub mod http_server;
pub mod shutdown;
pub mod telemetry;
pub mod util;

// TODO: Better structured logging that tracing supports
