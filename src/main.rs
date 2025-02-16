use std::env;

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Your IP is: {}",
        req.connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = env::var("HOST").unwrap_or("localhost".to_owned());
    let port: String = env::var("PORT").unwrap_or("8080".to_owned());

    println!("Server running at {host}:{port}");

    HttpServer::new(|| App::new().service(index))
        .bind(format!("{host}:{port}"))?
        .run()
        .await
}
