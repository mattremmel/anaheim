use anaheim_derive::controller;

struct Api;

#[controller]
impl Api {
    #[handler(path = "/hello", method = "get")]
    async fn hello(&self) -> PlainText<&'static str> {
        PlainText("hello world")
    }
}
