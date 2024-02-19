use actix_web::{get, web, HttpResponse, HttpRequest};
use minijinja::context;
use models::AppState;



pub fn config(cfg: &mut web::ServiceConfig) {
    
}


#[get("/hello_world")]
pub async fn hello_world(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return crate::render_boosted(&app_state, &req, "hellow_world", context! {});
}
