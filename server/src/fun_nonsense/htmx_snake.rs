use actix_web::{web, get, post, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use minijinja::context;
use models::AppState;

const GRID_SIZE: usize = 15;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Row {
    cells: Vec<Cell>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cell {
    cell_type: Option<String>,
    snake_index: usize,
    pos: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SnakeResponse {
    snake: Vec<String>,
    direction: String,
    apple: String 
}


#[get("/fun-nonsense/htmx-snake")]
pub async fn snake_game(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return app_state.render_template("htmx_snake.html", &req, context! { grid => get_new_grid() });
}

#[post("/fun-nonsense/htmx-snake/step")]
pub async fn snake_step(app_state: web::Data<AppState>, req_data: web::Json<SnakeResponse>, req: HttpRequest) -> HttpResponse {
    let mut cells: Vec<Cell> = Vec::new();



    return app_state.render_template("htmx_snake_gameboard.html", &req, context! { grid => grid });
}

fn get_new_grid() -> Vec<Row> {
    let mut grid: Vec<Row> = Vec::new();
    let snake_start = 5;
    let apple_start_x = 7;
    let apple_start_y = 7;
    for y in 0..GRID_SIZE {

        let mut row: Row = Row {
            cells: Vec::new(),
        };

        for x in 0..GRID_SIZE {
            let mut cell: Cell = Cell {
                cell_type: None,
                snake_index: 0,
                pos: format!("{},{}", x, y)
            };
            if y == snake_start && (x == snake_start || x == snake_start - 1) {
                cell.cell_type = Some("snake".to_string());
                if x == snake_start { 
                    cell.snake_index = 0;
                };
                if x == snake_start - 1 { 
                    cell.snake_index = 1;
                }
            }
            else if y == apple_start_y && x == apple_start_x
            {
                cell.cell_type = Some("apple".to_string());
            }

            row.cells.push(cell);
        }

        grid.push(row);
    }
    println!("{:?}", grid);
    
    return grid;
}

#[get("/fun-nonsense/htmx-snake/reset")]
pub async fn snake_reset(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return app_state.render_template("htmx_snake_gameboard.html", &req, context! { grid => get_new_grid() });
}
