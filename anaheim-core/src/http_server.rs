use crate::shutdown::shutdown_token;
use anyhow::{anyhow, Result};
use axum::{Router, Server};
use derive_new::new;
use serde::Deserialize;
use std::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(new)]
pub struct HttpServerBuilder {
    config: Config,
    #[new(value = "Router::new()")]
    router: Router,
}

impl HttpServerBuilder {
    pub fn merge(mut self, router: Router) -> Self {
        self.router = router.merge(self.router);
        self
    }

    // TODO: Function to take a Trait object that implements service and contains route
    // TODO: Trait extension with `controller` function that add a controller

    pub fn build(self) -> Result<HttpServer> {
        let addr = format!("{}:{}", &self.config.host, &self.config.port)
            .to_socket_addrs()?
            .next()
            .ok_or(anyhow!("Error parsing bind host/port"))?;

        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer {
            host: self.config.host,
            port: listener.local_addr()?.port(),
            listener,
            router: self.router,
        })
    }
}

pub struct HttpServer {
    host: String,
    port: u16,
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    pub fn builder(config: Config) -> HttpServerBuilder {
        HttpServerBuilder::new(config)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // TODO: Should be able to choose between spawn and run_until_stopped
    pub fn spawn(self) -> JoinHandle<Result<()>> {
        info!("Starting http server on {}:{}", self.host(), self.port());

        tokio::spawn(async move {
            Server::from_tcp(self.listener)?
                .serve(self.router.into_make_service())
                .with_graceful_shutdown(shutdown_token().cancelled())
                .await
                .map_err(|e| {
                    error!("Failed to start http server: {:#}", e);
                    e
                })?;

            info!("Stopping http server");
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {
        let _ = HttpServer::builder(Config {
            host: "".to_string(),
            port: 0,
        })
        .build();
    }
}
