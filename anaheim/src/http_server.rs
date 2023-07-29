use std::net::ToSocketAddrs;

use anyhow::{anyhow, Result};
use derive_new::new;
use poem::endpoint::BoxEndpoint;
use poem::{
    listener::{Acceptor, Listener, TcpAcceptor, TcpListener},
    web::LocalAddr,
    IntoEndpoint, Route, Server,
};
use serde::Deserialize;
use tokio::time::Duration;
use tracing::info;

use crate::shutdown::shutdown_token;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(new)]
pub struct HttpServerBuilder {
    config: Config,
    controllers: Vec<HttpController>,
}

impl HttpServerBuilder {
    pub fn add(mut self, controller: HttpController) -> Self {
        self.controllers.push(controller);
        self
    }

    // TODO: Function to take a Trait object that implements service and contains route
    // TODO: Trait extension with `controller` function that add a controller

    pub async fn build(self) -> Result<HttpServer> {
        let addr = format!("{}:{}", &self.config.host, &self.config.port)
            .to_socket_addrs()?
            .next()
            .ok_or(anyhow!("Error parsing bind host/port"))?;

        let acceptor = TcpListener::bind(addr).into_acceptor().await?;

        Ok(HttpServer {
            config: (),
            host: self.config.host,
            acceptor,
            route: self.route,
        })
    }
}

pub struct HttpController {
    path: String,
    service: Box<dyn IntoEndpoint<Endpoint = BoxEndpoint<'static>>>,
}

pub struct HttpServer {
    config: (),
    host: String,
    acceptor: TcpAcceptor,
    route: Route,
}

impl HttpServer {
    pub fn builder(config: Config) -> HttpServerBuilder {
        HttpServerBuilder::new(config)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn local_addr(&self) -> Vec<LocalAddr> {
        self.acceptor.local_addr()
    }

    pub async fn run(self) -> Result<()> {
        // TODO: Do a nicer print
        info!("Starting http server on {:?}", self.local_addr());

        Server::new_with_acceptor(self.acceptor)
            // TODO: Make timeout configurable; Optional timeout sets number of seconds to wait before murder open connections
            .run_with_graceful_shutdown(
                self.route,
                shutdown_token().cancelled(),
                Some(Duration::from_secs(30)),
            )
            .await?;

        info!("Stopping http server");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use poem_openapi::payload::PlainText;
    use poem_openapi::OpenApi;

    use super::*;

    struct UserController;

    #[OpenApi]
    impl UserController {
        #[oai(path = "/hello", method = "get")]
        async fn hello(&self) -> PlainText<&str> {
            PlainText("hello")
        }
    }

    // TODO: Impl trait that adds path directly to controller

    #[test]
    fn compile() {
        let _ = HttpServer::builder(Config {
            host: "127.0.0.1".to_string(),
            port: 0,
        })
        .add(UserController)
        .build();
    }
}
