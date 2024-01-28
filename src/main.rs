use actix_web::{HttpServer, App, get, web, Responder, HttpResponse};
use serde::Deserialize;

pub static HTML: &'static str = include_str!("./index.html");
pub static JSON: &'static str = include_str!("./prov.json");

#[derive(Deserialize)]
struct Params {
    redirect: Option<String>,
    title: Option<String>,
    description: Option<String>,
    sitename: Option<String>,
    image: Option<String>,
    color: Option<String>,
    title_url: Option<String>,
    sitename_url: Option<String>,
}

#[get("/")]
async fn index(info: web::Query<Params>) -> impl Responder {
    let info = info.into_inner();

    HttpResponse::Ok().content_type("text/html").body(HTML
        .replace("{{REDIRECT}}", &info.redirect.unwrap_or("https://davidon.top".to_string()))
        .replace("{{TITLE}}", &info.title.unwrap_or_default())
        .replace("{{DESCRIPTION}}", &info.description.unwrap_or_default())
        .replace("{{PROV}}", &info.sitename.unwrap_or_default())
        .replace("{{IMAGE}}", &info.image.unwrap_or_default())
        .replace("{{COLOR}}", &info.color.unwrap_or_default())
        .replace("{{TITURL}}", &info.title_url.unwrap_or_default())
        .replace("{{PROVURL}}", &info.sitename_url.unwrap_or_default())
    )
}

#[derive(Deserialize)]
struct ParamsJson {
    title: Option<String>,
    title_url: Option<String>,
    provider: Option<String>,
    provider_url: Option<String>,
}

#[get("/json")]
async fn json(info: web::Query<ParamsJson>) -> impl Responder {
    let info = info.into_inner();

    HttpResponse::Ok().content_type("application/json").body(JSON
        .replace("{{TITLE}}", &info.title.unwrap_or_default())
        .replace("{{TITURL}}", &info.title_url.unwrap_or_default())
        .replace("{{PROV}}", &info.provider.unwrap_or_default())
        .replace("{{PROVURL}}", &info.provider_url.unwrap_or_default())
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(json)
    })
    .bind(("127.0.0.1", 3596))?
    .run()
    .await
}
