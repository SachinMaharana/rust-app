use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde_derive::Serialize;
extern crate hostname;

#[derive(Debug, Serialize)]
struct Health {
    healthy: bool,
}
#[derive(Debug, Serialize)]
struct NotFound {
    message: String,
}

#[derive(Debug, Serialize)]
struct Info {
    host: String,
}

fn index(_req: HttpRequest) -> HttpResponse {
    let host_machine;
    match hostname::get_hostname() {
        Some(host) => host_machine = host,
        None => host_machine = "".to_owned(),
    }
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your have hit {}", host_machine))
}

fn healthz(_req: HttpRequest) -> HttpResponse {
    let healthy = Health { healthy: true };
    HttpResponse::Ok().json(healthy)
}

fn not_found(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(NotFound {
        message: "Not Found".to_owned(),
    })
}

fn info(req: HttpRequest) -> HttpResponse {
    let conn_info = req.connection_info();
    let host = conn_info.host().to_owned();
    let info = Info { host: host };
    HttpResponse::Ok().json(info)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // .service(web::resource("/index.html").to(|| "Hello world!"))
            .service(web::resource("/").to(index))
            .service(web::resource("/healthz").to(healthz))
            .service(web::resource("/info").to(info))
            .default_service(web::resource("*").to(not_found))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
