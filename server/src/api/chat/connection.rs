use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use crate::websocket::WebSocket;

#[get("/ws")]
async fn establish_connection(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WebSocket::new(), &req, stream)
}
