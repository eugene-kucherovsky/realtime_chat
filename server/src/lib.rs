pub mod api;
pub mod config;
pub mod model;
pub mod websocket;

use actix_web::{
    http::header, middleware as actix_web_middleware, App, HttpServer,
};

use actix_cors::Cors;
use dotenv::dotenv;

use config::config::Settings;
use api::chat::connection::establish_connection;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    log::info!("starting HTTP server at http://0.0.0.0:8080");

    let config = Settings::init();

    let app_host = &config.app_host.to_owned();
    let app_port = &config.app_port.to_owned();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(actix_web_middleware::Logger::default())
            .service(establish_connection)
            .wrap(cors)
    })
    .bind(format!("{}:{}", app_host, app_port))?
    .workers(2)
    .run()
    .await
}
