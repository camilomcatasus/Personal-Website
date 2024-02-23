use actix_web::{get, web, HttpResponse, HttpRequest};
use minijinja::context;
use models::AppState;
use serde::{Serialize, Deserialize};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
        .service(blurb)
        .service(chat);
}


#[get("/hello_world")]
async fn hello_world(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return crate::render_boosted(&app_state, &req, "hello_world", context! {});
}

#[derive(Serialize, Deserialize)]
struct Blurb {

}


//TODO: Implement redis and cache all blurbs
#[get("/hello_world/blurb")]
async fn blurb(_app_state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    return HttpResponse::Ok().json(Blurb {});
}

#[get("/hello_world/chat")]
async fn chat(_req: HttpRequest) -> HttpResponse {
    return HttpResponse::Ok().body("");
}
