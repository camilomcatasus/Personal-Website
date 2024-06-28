mod fun_nonsense;
mod serious_series;
mod hello_world;

use std::sync::Mutex;
use actix_files as fs;
use serde::{ Serialize, Deserialize };
use actix_web::{get, web::{self, ServiceConfig}, App, HttpRequest, HttpResponse, HttpServer};
use minijinja::{path_loader, Environment, context, Value};
use std::collections::HashMap;
use models::{AppState, RequestObject, CacheObject};

#[derive(Serialize, Deserialize)]
struct Project {
    pub image_url: String,
    pub title: String,
    pub information: String,
}

#[get("/{tail:.*}")]
async fn page() -> HttpResponse {
    return HttpResponse::TemporaryRedirect().insert_header(("Location", "/fun-nonsense/about")).finish();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));

    let request_cache: HashMap<RequestObject, CacheObject> = HashMap::new();
    let state = web::Data::new(AppState { env, request_cache: Mutex::new(request_cache) });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .configure(fun_nonsense::config)
            .configure(hello_world::config)
            .configure(serious_series::config)
            .service(page)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

pub fn render_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {
    let boosted = req.headers().get("HX-Request").is_some();
    let correct_path = match req.headers().get("HX-Request") {
        Some(_) => format!("{}/body.html", dir_path),
        None => format!("{}/page.html", dir_path),
    };

    app_state.render_template(&correct_path, context! { ..ctx, ..context!{boosted => boosted} })
}
