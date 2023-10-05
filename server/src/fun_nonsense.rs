use actix_web::{get, web, HttpResponse, HttpRequest};
use models::AppState;
use minijinja::context;
pub mod htmx_snake;


#[get("/fun-nonsense/about")]
pub async fn fun_nonsense_about(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return app_state.render_template("fun_nonsense/fun_nonsense_about.html", &req, context! {  });
}
