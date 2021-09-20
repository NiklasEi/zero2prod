use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
            .default_service(web::to(HttpResponse::NotFound))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
