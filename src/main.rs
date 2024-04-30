use actix_web::{
    body::MessageBody, http::header::TryIntoHeaderValue, web, HttpResponse, HttpServer,
};
use askama::Template;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(
                web::scope("/src")
                    .service(
                        web::resource("/htmx.min.js")
                            .get(|| async { http_response_js(include_str!("files/htmx.min.js")) }),
                    )
                    .service(web::resource("/tailwindcss.3.4.3.js").get(|| async {
                        http_response_js(include_str!("files/tailwindcss.3.4.3.js"))
                    }))
                    .service(web::resource("/qr-scanner.min.js").get(|| async {
                        http_response_js(include_str!("files/qr-scanner.min.js"))
                    }))
                    .service(web::resource("/qr-scanner-worker.min.js").get(|| async {
                        http_response_js(include_str!("files/qr-scanner-worker.min.js"))
                    })),
            )
            .service(web::resource("/").to(|| async { Index {} }))
            .service(web::resource("/camera.html").get(|| async { CameraX {} }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "camera.x.html")]
pub struct CameraX {}

fn http_response_file<B, V>(body: B, media: V) -> HttpResponse
where
    B: MessageBody + 'static,
    V: TryIntoHeaderValue,
{
    HttpResponse::Ok().content_type(media).body(body)
}

const MEDIA_TYPE_JS: &'static str = "text/javascript";
fn http_response_js<B>(body: B) -> HttpResponse
where
    B: MessageBody + 'static,
{
    http_response_file(body, MEDIA_TYPE_JS)
}

const MEDIA_TYPE_CSS: &'static str = "text/css";
fn http_response_css<B>(body: B) -> HttpResponse
where
    B: MessageBody + 'static,
{
    http_response_file(body, MEDIA_TYPE_CSS)
}
