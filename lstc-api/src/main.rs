use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};

#[post("/api/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Echo: {}", req_body))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

fn configure_app(config: &mut ServiceConfig) {
    config
        .service(echo)
        .service(health)
        .default_service(web::to(not_found));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Configuring logging...");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_port: u16 = 8080;

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(configure_app))
        .bind(("0.0.0.0", app_port))?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{body, http::header::ContentType, test};

    #[test]
    async fn health_endpoint() {
        let app = App::new().configure(configure_app);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        let status = resp.status();
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert!(status.is_success());
        assert_eq!(bytes.unwrap(), web::Bytes::from_static(b"OK"));
    }

    #[test]
    async fn echo_endpoint() {
        let app = App::new().configure(configure_app);
        let app = test::init_service(app).await;

        let req = test::TestRequest::post()
            .set_payload("Hello")
            .uri("/api/echo")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;

        let status = resp.status();
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert!(status.is_success());
        assert_eq!(bytes.unwrap(), web::Bytes::from_static(b"Echo: Hello"));
    }

    #[test]
    async fn not_found_endpoint() {
        let app = App::new().configure(configure_app);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/nonsense/path").to_request();
        let resp = test::call_service(&app, req).await;

        let status = resp.status();
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert!(!status.is_success());
        assert_eq!(bytes.unwrap(), web::Bytes::from_static(b"Not Found"));
    }
}
