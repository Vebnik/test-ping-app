mod types;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

use types::Args;

#[get("/ping")]
async fn ping_get() -> impl Responder {
    println!("Incoming request");
    HttpResponse::Ok().body("Hello body")
}

#[post("/ping")]
async fn ping_post(body: String) -> impl Responder {
    println!("Incoming request with body: {}", body);
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Try to listen: {} port", &args.port);

    HttpServer::new(|| {
        App::new()
            .service(ping_get)
            .service(ping_post)

    })
    .bind((args.addr, args.port))?
    .run()
    .await
}