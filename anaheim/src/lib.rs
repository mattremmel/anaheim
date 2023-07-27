pub mod config;
pub mod http_server;
pub mod shutdown;
pub mod telemetry;
pub mod util;

// TODO: Better structured logging that tracing supports

// re-exports
pub use async_trait::async_trait;
pub use mockall::automock;
