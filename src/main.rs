use actix_web::{
    body::MessageBody, http::header::TryIntoHeaderValue, web, HttpResponse, HttpServer,
};
use askama::Template;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(
                web::resource("/src/htmx.min.js")
                    .to(|| async { http_response_js(include_str!("files/htmx.min.js")) }),
            )
            .service(
                web::resource("/src/tailwindcss.3.4.3.js")
                    .to(|| async { http_response_js(include_str!("files/tailwindcss.3.4.3.js")) }),
            )
            .service(
                web::resource("/src/main.css")
                    .get(|| async { http_response_css(include_str!("files/main.css")) }),
            )
            .service(
                web::resource("/src/qr-scanner.min.js")
                    .get(|| async { http_response_js(include_str!("files/qr-scanner.min.js")) }),
            )
            .service(
                web::resource("/src/qr-scanner-worker.min.js").get(|| async {
                    http_response_js(include_str!("files/qr-scanner-worker.min.js"))
                }),
            )
            .service(web::resource("/").to(|| async { Index {} }))
            .service(web::resource("/camera.html").to(|| async { CameraX {} }))
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
