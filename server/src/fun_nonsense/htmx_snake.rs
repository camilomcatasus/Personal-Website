use actix_web::{web, get, post, App, HttpServer, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Row {
    cells: Vec<Cell>
}

#[derive(Serialize, Deserialize, Clone)]
struct Cell {
    type: Option<String>,
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
    let snake_start = 5;
    let apple_start_x = 7;
    let apple_start_y = 7;
    for y in 0..10 {

        let row: Row = Row::new();
        let cells: Vec<Cell> = Vec::new();
        for x in 0..10 {
            let cell: Cell = Cell::new();
            if y == snake_start && (x == snake_start || x == snake_start - 1) {
                cell.type = Some("snake");
                if x == snake_start { 
                    cell.snake_index = 0;
                };
                if x == snake_start - 1 { 
                    snake_index = 1;
                }
            }
            else if y == apple_start_y && x == apple_start_x
            {
                cell.type = Some("apple");
            }
            else 
            {
                cell.type = None;
            }
            cell.pos = format!("{},{}", x, y);
        }
    }

    return app_state.render_tempalte("htmx_snake_gameboard.html", &req, context! { grid => grid });
}
