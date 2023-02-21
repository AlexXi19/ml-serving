use actix_web::{get, HttpResponse, Responder};
use log::{info, LevelFilter};
use swerve::logger::SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

#[actix_web::main]
async fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    info!("Starting Swerve Web Server!");
    actix_web::HttpServer::new(|| actix_web::App::new().service(hello).service(ping))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
        .unwrap();
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Swerve!")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
