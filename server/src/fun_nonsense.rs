use actix_web::{error::ErrorInternalServerError, http::header::ContentType, web, HttpRequest, HttpResponse};
use models::AppState;
use minijinja::{context, Value};
use serde::{Serialize, Deserialize};
mod htmx_snake;

const ROUTES: [Route; 3] = [
    Route { 
        path: "fun-nonsense/about", 
        text:"About", 
        simple: true,
        title:"This is a meta page",
    },
    Route { 
        path: "/fun-nonsense/htmx-snake", 
        text:"Snake in HTMX", 
        simple: false,
        title: "SNEK",
    },
    Route { 
        path: "/fun-nonsense/fun-with-shaders", 
        text: "Fun with shaders", 
        simple: true,
        title: "Fun with shaders"
    },
];

pub fn config(cfg: &mut web::ServiceConfig) {
    let mut new_cfg = cfg.configure(htmx_snake::config);


    for route in ROUTES {
        if route.simple {
            new_cfg = new_cfg.route(route.path, web::get().to(move |app_state: web::Data<AppState>, req: HttpRequest| fun_nonsense_simple(app_state, req, route.clone())));
        }
    }
}

pub async fn fun_nonsense_simple(app_state: web::Data<AppState>, req: HttpRequest, route: Route) -> actix_web::Result<HttpResponse, actix_web::Error> {
    println!("Path: {}", req.path());
    let boosted = req.headers().get("HX-Request").is_some();

    let rendered_template: String;

    let context = context! { 
        fun_nonsense_route => route,
        blog_path => format!("{}/blog.html", route.path.trim_start_matches("/")),
        boosted => boosted,
        routes => ROUTES
    };

    if !boosted {
        let page_template = format!("
        {{% extends \"fun-nonsense/page.html\" %}}
        {{% block title %}}{title}{{% endblock %}}
        {{% block content %}}
        {{% if not fun_nonsense_route.path %}}
        {{% include \"{path}/body.html\" %}}
        {{% endif %}}
        {{% include \"{path}/blog.html\" %}}
        {{% endblock %}}
        ", title = route.title, path = route.path);

        rendered_template = app_state.env.render_str(&page_template, context).unwrap();
    }
    else {
        rendered_template = app_state.env
            .get_template(&format!("{}/blog.html", route.path))
            .unwrap().render(context).unwrap(); 
    }

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered_template));
}

pub fn render_fun_boosted(app_state: &web::Data<AppState>, req: &HttpRequest, dir_path: &str, ctx: Value) -> HttpResponse {
    crate::render_boosted(app_state, req, dir_path, context! { ..ctx, ..context!{ routes => ROUTES} })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Route {
    path: &'static str,
    text: &'static str,
    simple: bool,
    title: &'static str,
}
