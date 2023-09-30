use actix_web::{web, get, post, App, HttpServer, HttpRequest, HttpResponse};


struct Row {
    cells: Vec<Cell>
}
struct Cell {
    type: String,
    snake_index: usize,
    pos: String
}

#[post("/fun-nonsense/htmx-snake/step")]
pub async fn snake_step(req: HttpRequest) -> HttpResponse {
    let grid: Vec<Row> = Vec::new();
    return app_state.render_template("htmx_snake_gameboard.html", &req, context! { grid => grid });
}

#[get("/fun-nonsense/htmx-snake/reset")]
pub async fn snake_reset(req: HttpRequest) -> HttpResponse {
    let grid: Vec<Row> = Vec::new();
    return app_state.render_tempalte("htmx_snake_gameboard.html", &req, context! { grid => grid });
}
