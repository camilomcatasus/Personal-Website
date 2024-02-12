use actix_web::{get, web, HttpResponse, HttpRequest};
use models::AppState;
use minijinja::{context, Value};
pub mod htmx_snake;


#[get("/fun-nonsense/about")]
pub async fn fun_nonsense_about(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return render_boosted(&app_state, 
                          &req,
                          "fun_nonsense/about", context! {  });
}

pub fn render_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {
    let boosted = req.headers().get("HX-Request").is_some();
    let correct_path = match req.headers().get("HX-Request") {
        Some(_) => format!("{}/body.html", dir_path),
        None => format!("{}/page.html", dir_path),
    };
    return app_state.render_template(&correct_path, context! { ..ctx, ..context!{boosted => boosted} });
}
