use actix_cors::Cors;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web::middleware::Logger;
use http::Method;
use log::Level;


use mockserver::yts::data::MovieResponse;
use mockserver::yts::data::YtsResponse;
use mockserver::yts::mocks::mockYtsMovies;
use serde::{Deserialize, Serialize};


async fn ystMovies(name: web::Path<String>) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
        .header("Access-Control-Allow-Origin","*")
        .header("Strict-Transport-Security", "max-age=31556926; includeSubDomains; preload")
        .json(mockYtsMovies())
    )
}

async fn index(req: HttpRequest) -> &'static str {
    "Hello world"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST","OPTIONS","PUT","DELETE","PATCH"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .route("/ws/yts/torrents/{name}", web::get().to(ystMovies))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
