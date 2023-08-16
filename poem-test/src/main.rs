use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

#[tokio::main]
async fn main() {
    let api_service1 = OpenApiService::new((Api1, Api2), "Api1", "1.0");
    // let api_service2 = OpenApiService::new(Api2, "Api2", "1.0");
    let app = Route::new().nest("/api", api_service1);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .unwrap();
}

struct Api1;

#[OpenApi]
impl Api1 {
    #[oai(path = "/hello1", method = "get")]
    async fn hello1(&self) -> PlainText<&'static str> {
        PlainText("hello1")
    }
}

struct Api2;

#[OpenApi]
impl Api2 {
    #[oai(path = "/hello2", method = "get")]
    async fn hello1(&self) -> PlainText<&'static str> {
        PlainText("hello2")
    }
}
