use actix_cors::Cors;
use actix_web::{ App, HttpServer, http};

mod api_error;
mod generation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();


    HttpServer::new(|| {

        let cors = Cors::default()
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    
        App::new()
        .wrap(cors)
            .configure(generation::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}