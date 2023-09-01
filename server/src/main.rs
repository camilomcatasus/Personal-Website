mod endpoints;
use std::cell::RefCell;
use endpoints::{getApiText, RequestObject, CacheObject, ResponseObject};
use actix_files as fs;
use actix_web::http::header::ContentType;
use serde::{ Serialize, Deserialize };
use actix_web::{web, get, post, App, HttpServer, HttpRequest, HttpResponse};
use minijinja::value::Value;
use minijinja::{path_loader, Environment, context};
use rand::seq::SliceRandom;
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
    request_cache: HashMap<RequestObject, CacheObject>,
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

#[derive(Serialize, Deserialize)]
struct BlurbRequestData {
    screen_w: usize,
    screen_h: usize,
    displayed_rects: Vec<RectPos>
}

#[derive(Serialize, Deserialize)]
struct RectPos {
    col: usize,
    row: usize,
    #[serde(default)]
    deletion_flag: bool
}

const BLURB_WIDTH: usize = 250;
const BLURB_HEIGHT: usize = 100;

#[post("/api/blurb")]
async fn blurb(app_state: web::Data<AppState>, req_data: web::Json<BlurbRequestData>, req: HttpRequest) -> HttpResponse {
    
    let col_total: usize = req_data.screen_w / BLURB_WIDTH;
    let row_total: usize = req_data.screen_h / BLURB_HEIGHT;

    let mut positions: Vec<RectPos> = Vec::new();
    
    for x in 0..col_total {
        for y in 0..row_total {
           positions.push(RectPos {
               col: x,
               row: y,
               deletion_flag: false
           });
        }
    }

    req_data.displayed_rects.iter().for_each( |rect_pos| {
        match positions.get_mut(rect_pos.col + rect_pos.row * col_total) {
            Some(rect_ref) => {
                rect_ref.deletion_flag = true;
            },
            None => {} 
        }
    });

    let positions_length = positions.len();

    for i in positions_length..0 {
        if positions.get(i).unwrap().deletion_flag {
            positions.remove(i);
        }
    }

    let mut rng = rand::thread_rng();
    let choice = positions.choose(&mut rng).unwrap();
    println!("RECT SELECTED | X: {}, Y{}", choice.col, choice.row);

    let random_response_result: Result<ResponseObject, anyhow::Error> = getApiText(None, &app_state.request_cache).await;
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


    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));
    let request_cache: HashMap<RequestObject, CacheObject> = HashMap::new();
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
