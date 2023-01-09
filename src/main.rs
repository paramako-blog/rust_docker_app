use actix_web::{App, HttpServer, HttpResponse, get};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}


