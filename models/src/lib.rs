use std::sync::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;
use minijinja::value::Value;
use enum_macros::EnumArray;
use std::time::{Duration, Instant};
use actix_web::{HttpRequest, HttpResponse, http::header::ContentType};
use serde::{Serialize, Deserialize};

pub struct AppState {
    pub env: minijinja::Environment<'static>,
    pub request_cache: Mutex<HashMap<RequestObject, CacheObject>>,
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

#[derive(EnumArray, PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum RequestObject {
    AirQualityRequest,
    LastAnimeRequest,
    LeagueRankRequest,
    HistoricalEventRequest,
    BreakingNewsRequest,
}

#[derive(Clone)]
pub struct CacheObject {
    pub last_get_time: Instant,
    pub keep_alive: Duration,
    pub last_response: ResponseObject
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseObject {
    pub inner_text: String,
    pub help_text: Option<String>,
    pub url: String
}

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
