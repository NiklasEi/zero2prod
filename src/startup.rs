use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health-check",
                web::get().to(crate::routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(crate::routes::subscriptions::subscribe),
            )
            .default_service(web::to(HttpResponse::NotFound))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
