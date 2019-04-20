extern crate hostname;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use clap::{crate_authors, crate_description, crate_name, crate_version};
use dotenv::dotenv;
use serde_derive::Serialize;
use std::env;

#[derive(Debug, Serialize)]
struct Health {
    healthy: bool,
}
#[derive(Debug, Serialize)]
struct NotFound {
    message: String,
}
#[derive(Debug, Serialize)]
struct Env {
    host: String,
    remote_ip: String,
}

#[derive(Debug, Serialize)]
struct Info {
    authors: String,
    description: String,
    name: String,
    version: String,
}

fn index(req: HttpRequest) -> HttpResponse {
    let host_machine = match hostname::get_hostname() {
        Some(host) => host,
        None => "".to_owned(),
    };
    let env = Env {
        host: host_machine,
        remote_ip: req.connection_info().host().to_owned(),
    };
    HttpResponse::Ok().json(env)
}

fn healthz(_req: HttpRequest) -> HttpResponse {
    let healthy = Health { healthy: true };
    HttpResponse::Ok().json(healthy)
}

fn not_found(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(NotFound {
        message: "Resource Not Found".to_owned(),
    })
}

// host: req.connection_info().host().to_owned(),

fn info(_req: HttpRequest) -> HttpResponse {
    let info = Info {
        authors: crate_authors!().to_owned(),
        description: crate_description!().to_owned(),
        name: crate_name!().to_owned(),
        version: crate_version!().to_owned(),
    };
    HttpResponse::Ok().json(info)
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let addr: String = env::var("ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()
        .expect("Can't Parse ADDRESS variable.");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/healthz").to(healthz))
            .service(web::resource("/info").to(info))
            .default_service(web::resource("*").to(not_found))
    })
    .bind(addr)?
    .run()
}
