use actix_web::{web, web::Path, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(web::resource("/files/{name}").to(files))
            .service(web::resource("/").to(|| async { IndexTemplate {} }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn files(path: Path<String>) -> HttpResponse {
    match path.as_str() {
        "camera.js" => HttpResponse::Ok()
            .content_type("text/javascript")
            .body(include_str!("camera.js"))
            .into(),
        _ => HttpResponse::NotFound().into(),
    }
}

#[derive(askama_actix::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
