use actix_web::{get, web, HttpResponse, HttpRequest};
use models::AppState;
use minijinja::{context, Value};
use serde::{Serialize, Deserialize};
pub mod htmx_snake;


#[get("/fun-nonsense/about")]
pub async fn fun_nonsense_about(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return render_boosted(&app_state, 
                          &req,
                          "fun_nonsense/about", context! {  });
}

pub fn render_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {

    let boosted = req.headers().get("HX-Request").is_some();
    let routes: Vec<Route> = vec![
        Route { path: "/fun-nonsense/about", text:"About" },
        Route { path: "/fun-nonsense/htmx-snake", text:"Htmx Snake" },
    ];
    let correct_path = match req.headers().get("HX-Request") {
        Some(_) => format!("{}/body.html", dir_path),
        None => format!("{}/page.html", dir_path),
    };

    app_state.render_template(&correct_path, context! { ..ctx, ..context!{boosted => boosted, routes => routes} })
}

#[derive(Serialize, Deserialize)]
struct Route {
    path: &'static str,
    text: &'static str
}
