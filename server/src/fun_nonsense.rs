use actix_web::{get, web, HttpResponse, HttpRequest};
use models::AppState;
use minijinja::{context, Value};
use serde::{Serialize, Deserialize};
mod htmx_snake;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(fun_nonsense_about)
        .configure(htmx_snake::config);
}

#[get("/fun-nonsense/about")]
pub async fn fun_nonsense_about(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return render_fun_boosted(&app_state, 
                          &req,
                          "fun_nonsense/about", context! {  });
}

pub fn render_fun_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {

    let routes: Vec<Route> = vec![
        Route { path: "/fun-nonsense/about", text:"About" },
        Route { path: "/fun-nonsense/htmx-snake", text:"Htmx Snake" },
    ];

    crate::render_boosted(app_state, req, dir_path, context! { ..ctx, ..context!{ routes => routes} })
}

#[derive(Serialize, Deserialize)]
struct Route {
    path: &'static str,
    text: &'static str
}
