mod endpoints;
mod fun_nonsense;
mod blurb;

use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use actix_files as fs;
use serde::{ Serialize, Deserialize };
use actix_web::{web, get, App, HttpServer, HttpRequest, HttpResponse, middleware::Logger};
use minijinja::{path_loader, Environment, context};
use std::collections::HashMap;
use models::{AppState, RequestObject, CacheObject};
use fun_nonsense::htmx_snake::{snake_step, snake_reset, snake_game};


#[derive(Serialize, Deserialize)]
struct Project {
    pub image_url: String,
    pub title: String,
    pub information: String,
}

#[get("/{tail:.*}")]
async fn page(app_state: web::Data<AppState>, req:HttpRequest) -> HttpResponse {

    let projects_file = File::open("./data/projects.json").unwrap();
    let reader = BufReader::new(projects_file);
   
    let serious_projects: Vec<Project> = serde_json::from_reader(reader).unwrap();
    return app_state.render_template("base.html", &req, context! { serious_projects => serious_projects, variant => "chat" });
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
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(blurb::config)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fun_nonsense::fun_nonsense_about)
            .service(snake_game)
            .service(snake_reset)
            .service(snake_step)
            .service(page)
   })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
