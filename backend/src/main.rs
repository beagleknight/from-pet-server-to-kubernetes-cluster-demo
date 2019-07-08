use actix_cors::Cors;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use hostname::get_hostname;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {} from {}!", &name, get_hostname().unwrap())
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(
                web::resource("/{name}")
                    .route(web::get().to(greet))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
}
