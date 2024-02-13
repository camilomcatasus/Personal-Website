mod fun_nonsense;

use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use actix_files as fs;
use serde::{ Serialize, Deserialize };
use actix_web::{web::{ServiceConfig, self}, get, App, HttpServer, HttpRequest, HttpResponse, middleware::Logger};
use minijinja::{path_loader, Environment, context};
use std::collections::HashMap;
use models::{AppState, RequestObject, CacheObject};
use fun_nonsense::htmx_snake::{snake_step, snake_reset, snake_game};
use shuttle_actix_web::ShuttleActixWeb;

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

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));

    let request_cache: HashMap<RequestObject, CacheObject> = HashMap::new();
    let state = web::Data::new(AppState { env, request_cache: Mutex::new(request_cache) });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = move |cfg: & mut ServiceConfig| {
        cfg.service(web::scope("/").wrap(Logger::default())
            .app_data(state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fun_nonsense::fun_nonsense_about)
            .service(snake_game)
            .service(snake_reset)
            .service(snake_step));
            //.service(page)
    };

    return Ok(config.into());
}
