use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Configuring logging...");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_port: u16 = 8080;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(echo)
            .service(health)
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}
