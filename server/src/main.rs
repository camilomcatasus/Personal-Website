mod endpoints;
use std::cell::RefCell;
use std::time::{Instant, Duration};

use endpoints::{getApiText, RequestObject};
use actix_files as fs;
use actix_web::http::header::ContentType;
use actix_web::{web, get, App, HttpServer, HttpRequest, HttpResponse };
use minijinja::value::Value;
use minijinja::{path_loader, Environment, context};
use rand::Rng;
use std::collections::HashMap;

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
    request_cache: HashMap<endpoints::RequestObject, endpoints::CacheObject>,
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

fn get_area(rect: (i32, i32, i32, i32)) -> f64 {
    return f64::from((rect.2 - rect.0) * (rect.3 - rect.1));
}

#[get("/api/blurb/{blocking_start_x}/{blocking_start_y}/{blocking_end_x}/{blocking_end_y}/{screen_x}/{screen_y}")]
async fn blurb(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    
    let point_x: i32;
    let point_y: i32;

    let query_params_result: Result<(i32,i32, i32, i32, i32, i32), _> = req.match_info().load();
    match query_params_result {
        Ok(param_tuple) => {
            let random_rect = rand::random::<f64>();

            let (blocking_start_x, blocking_start_y, blocking_end_x, blocking_end_y, screen_x, screen_y) = param_tuple;

            let left_rect: (i32, i32, i32, i32) = (0, 0, blocking_start_x, screen_y);
            let right_rect: (i32, i32, i32, i32) = (blocking_end_x, 0, screen_x, screen_y);
            let bot_rect: (i32, i32, i32, i32) = (blocking_start_x, 0, blocking_end_x, blocking_start_y);
            let top_rect: (i32, i32, i32, i32) = (blocking_start_x, blocking_end_y, blocking_end_x, screen_y);
            
            let total_area: f64 = get_area(left_rect) + get_area(right_rect) + get_area(bot_rect) + get_area(top_rect);
            let selected_rect: (i32, i32, i32, i32);

            if random_rect <= get_area(left_rect) / total_area {
                selected_rect = left_rect;
            }
            else if random_rect <= get_area(right_rect) / total_area {
                selected_rect = right_rect;
            }
            else if random_rect <= get_area(bot_rect) / total_area {
                selected_rect = bot_rect;
            }
            else {
                selected_rect = top_rect;
            }
            point_x = rand::thread_rng().gen_range(selected_rect.0..selected_rect.2);
            point_y = rand::thread_rng().gen_range(selected_rect.1..selected_rect.3);
        }
        Err(err) => {
            return app_state.render_template("error_blurb.html", &req, context! {
                x => 0,
                y => 0,
                inner_text => "An error occurred while making the API request",
                hidden_text => format!("Error Message: {}", err)
            });
        }
    }

    let random_response_result: Result<endpoints::ResponseObject, anyhow::Error> = endpoints::getApiText(None, &app_state.request_cache).await;
    match random_response_result {
        Ok(random_response) => {
            return app_state.render_template("blurb.html", &req, context! {
                x => point_x,
                y => point_y,
                inner_text => random_response.inner_text,
                url => random_response.url,
                hidden_text => random_response.help_text
            });
        },
        Err(err) => {
            return app_state.render_template("error_blurb.html", &req, context! {
                x => point_x,
                y => point_y,
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


    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));
    let mut request_cache: HashMap<endpoints::RequestObject, (Instant, Duration)> = HashMap::new();
    let state = web::Data::new(AppState { env, request_cache });

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
