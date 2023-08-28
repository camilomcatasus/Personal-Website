mod endpoints;
use std::cell::RefCell;

use endpoints::{getApiText, RequestObject};
use actix_files as fs;
use actix_web::http::header::ContentType;
use actix_web::{web, get, App, HttpServer, HttpRequest, HttpResponse };
use minijinja::value::Value;
use minijinja::{path_loader, Environment, context};

thread_local! {
    static CURRENT_REQUEST: RefCell<Option<HttpRequest>> = RefCell::default()
}

fn with_bound_req<F, R>(req: &HttpRequest, f: F) -> R
where 
    F: FnOnce() -> R, 
{
    CURRENT_REQUEST.with(|current_req| *current_req.borrow_mut() = Some(req.clone()));
    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    CURRENT_REQUEST.with(|current_req| current_req.borrow_mut().take());
    match rv {
        Ok(rv) => rv,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub struct AppState {
    env: minijinja::Environment<'static>,
}

impl AppState {
    pub fn render_template(&self, name: &str, req: &HttpRequest, ctx: Value) -> HttpResponse {
        with_bound_req(req, || {
            let tmpl = self.env.get_template(name).unwrap();
            let rv = tmpl.render(ctx).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType ::html())
                .body(rv)
        })
    }
}


#[get("/api/blurb/{blocking_start_x}/{blocking_start_y}/{blocking_end_x}/{blocking_end_y}/{screen_x}/{screen_y}")]
async fn blurb(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {

    let random_x = rand::random::<f64>();
    let random_y = rand::random::<f64>();
    let point_x: u32;
    let point_y: u32;
    let query_params_result: Result<(u32,u32, u32, u32, u32, u32), _> = req.match_info().load();
    match query_params_result {
        Ok(param_tuple) => {
            point_x = 0;
            point_y = 0;
        }
        Err(err) => {
            return app_state.render_template("error_blurb.html", &req, context! {
                inner_text => "An error occurred while making the API request",
                hidden_text => format!("Error Message: {}", err)
            });
        }
    }

    let random_response_result: Result<endpoints::ResponseObject, anyhow::Error> = endpoints::getApiText(None).await;
    match random_response_result {
        Ok(random_response) => {
            return app_state.render_template("blurb.html", &req, context! {
                inner_text => random_response.inner_text,
                url => random_response.url,
                hidden_text => random_response.help_text
            });
        },
        Err(err) => {
            return app_state.render_template("error_blurb.html", &req, context! {
                inner_text => "An error occurred while making the API request",
                hidden_text => format!("Error Message: {}", err)
            });
        }
    }
}

#[get("/{tail:.*}")]
async fn page(app_state: web::Data<AppState>, req:HttpRequest) -> HttpResponse {
    return app_state.render_template("base.html", &req, context! {});
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let air_quality = endpoints::getApiText(Some(&RequestObject::LeagueRankRequest)).await;
//let my_anime = getApiText(Some(&RequestObject::LastAnimeRequest)).await;
    println!("{}", serde_json::to_string(&air_quality.unwrap()).unwrap());
    //println!("{}", serde_json::to_string(&my_anime.unwrap()).unwrap());

    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));
    let state = web::Data::new(AppState { env });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(page)
   })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
