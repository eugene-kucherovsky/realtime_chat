use crate::websocket::WebSocket;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[get("/ws")]
async fn establish_connection(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WebSocket::new(), &req, stream)
}
