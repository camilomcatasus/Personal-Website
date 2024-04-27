use actix_web::{web, HttpResponse, HttpRequest};
use models::AppState;
use minijinja::{context, Value};
use serde::{Serialize, Deserialize};
mod htmx_snake;

const ROUTES: [Route; 3] = [
    Route { path: "/fun-nonsense/about", text:"About", simple: true},
    Route { path: "/fun-nonsense/htmx-snake", text:"Htmx Snake", simple: false},
    Route { path: "/fun-nonsense/shader-balls", text: "Shader Balls", simple: true},
];

pub fn config(cfg: &mut web::ServiceConfig) {
    let mut new_cfg = cfg.configure(htmx_snake::config);

    for route in ROUTES {
        if route.simple {
            new_cfg = new_cfg.route(route.path, web::get().to(fun_nonsense_simple));
        }
    }
}

pub async fn fun_nonsense_simple(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    println!("Path: {}", req.path());
    return render_fun_boosted(&app_state, 
                          &req,
                          &req.path().trim_start_matches("/"), context! {  });
}

pub fn render_fun_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {
    crate::render_boosted(app_state, req, dir_path, context! { ..ctx, ..context!{ routes => ROUTES} })
}

#[derive(Serialize, Deserialize)]
struct Route {
    path: &'static str,
    text: &'static str,
    simple: bool,
}
