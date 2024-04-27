use std::iter::repeat;

use actix_web::{get, web, HttpRequest, HttpResponse};
use minijinja::context;
use models::AppState;
use serde::{Serialize, Deserialize};

use crate::render_boosted;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(main);
}

#[derive(Clone, Serialize, Deserialize)]
struct Project <'a> {
    source: &'a str,
}

#[get("/serious-series")]
async fn main(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let projects: Vec<Project> = repeat(Project{ source: "https://picsum.photos/200"}).take(8).collect();
    render_boosted(&app_state, &req, "serious-series", context! {projects => projects})
}
