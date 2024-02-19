use actix_web::{web, get, post, HttpRequest, HttpResponse};
use rand::Rng;
use serde::{Serialize, Deserialize};
use minijinja::context;
use models::AppState;

use crate::fun_nonsense::render_boosted;

const GRID_SIZE: usize = 15;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(snake_game)
        .service(snake_step)
        .service(snake_reset);
}

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
pub struct SnakeResponse {
    snake: Vec<String>,
    direction: String,
    apple: String 
}

fn generate_empty_grid() -> Vec<Row> {
    let mut grid: Vec<Row> = Vec::new();
    for y in 0..GRID_SIZE {

        let mut row: Row = Row {
            cells: Vec::new(),
        };

        for x in 0..GRID_SIZE {
            let cell: Cell = Cell {
                cell_type: None,
                snake_index: 0,
                pos: format!("{},{}", x, y)
            };
            row.cells.push(cell);
        }
        grid.push(row);
    }

    return grid;
}

#[get("/fun-nonsense/htmx-snake")]
pub async fn snake_game(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return render_boosted(&app_state, &req, "fun_nonsense/htmx_snake", context! { grid => get_new_grid() });
}

#[post("/fun-nonsense/htmx-snake/step")]
pub async fn snake_step(app_state: web::Data<AppState>, req_data: web::Json<SnakeResponse>) -> HttpResponse {
    let mut grid = generate_empty_grid();

    let (mut apple_x, mut apple_y) = match parse_apple_str(&req_data.apple) {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("No Direction")
    };

    let mut snake_cells: Vec<SnakeCell> = Vec::new();

    for snake_item_str in &req_data.snake {
        let parse_result = parse_snake_str(snake_item_str);
        match parse_result {
            Ok(snake_cell) => {
                snake_cells.push(snake_cell);
            },
            Err(_) => {
                return HttpResponse::BadRequest().finish();
            }
        }
    }
    let snake_len = snake_cells.len();
    snake_cells.sort_by_key(|cell| cell.index);

    let mut gen_apple_flag = false;
    let mut head_x = 0;
    let mut head_y = 0;
    for current_snake_cell in &snake_cells {
        if current_snake_cell.index == 0 { 
            head_x = current_snake_cell.x;
            head_y = current_snake_cell.y;

            match req_data.direction.as_str() {
                "1" => {
                    if head_x == 0 {
                        head_x = GRID_SIZE - 1;
                    }
                    else {
                        head_x -= 1;
                    }
                },
                "2" => {
                    if head_y == 0 {
                        head_y = GRID_SIZE - 1;
                    }
                    else {
                        head_y -= 1;
                    }
                },
                "3" => {
                    head_x.increment(GRID_SIZE);
                },
                "4" => {
                    head_y.increment(GRID_SIZE);
                },
                &_ => { return HttpResponse::BadRequest().body("No Direction"); }

            }

            if head_x == apple_x && head_y == apple_y {
                if snake_len + 1 == usize::from(GRID_SIZE * GRID_SIZE) {
                    return app_state.render_template("fun_nonsense/htmx_snake/htmx_snake_win.html", context! { grid => grid });
                }

                gen_apple_flag = true;
                let old_tail = snake_cells.last().unwrap();
                println!("{:?}", old_tail);
                let new_tail = &mut grid[old_tail.y].cells[old_tail.x];
                new_tail.cell_type = Some("snake".to_string());
                new_tail.snake_index = snake_len;
            }
            
            let new_head = &mut grid[head_y].cells[head_x];
            new_head.cell_type = Some("snake".to_string());
            new_head.snake_index = 0;


            continue; 
        }

        let old_snake_cell = &snake_cells[current_snake_cell.index - 1];
        let selected_grid_cell = &mut grid[old_snake_cell.y].cells[old_snake_cell.x];
        selected_grid_cell.snake_index = current_snake_cell.index;
        selected_grid_cell.cell_type = Some("snake".to_string());
    }

    if  grid[head_y].cells[head_x].snake_index != 0 {
        return app_state.render_template("fun_nonsense/htmx_snake/htmx_snake_loss.html", context! {});
    }

    // Semi random apple generation
    if gen_apple_flag {
        let mut rng = rand::thread_rng();
        let mut new_apple_x: usize = rng.gen_range(0..GRID_SIZE);
        let mut new_apple_y: usize = rng.gen_range(0..GRID_SIZE);

        let mut loop_condition: bool = true;
        while loop_condition {
            if grid[new_apple_y].cells[new_apple_x].cell_type == None {
                loop_condition = false;
            }
            else {
                if new_apple_x == GRID_SIZE - 1 {
                    new_apple_y.increment(GRID_SIZE);
                }
                new_apple_x.increment(GRID_SIZE);
            }
        }

        apple_x = new_apple_x;
        apple_y = new_apple_y;
    }

    grid[apple_y].cells[apple_x].cell_type = Some("apple".to_string());
    return app_state.render_template("fun_nonsense/htmx_snake/htmx_snake_gameboard.html", context! { grid => grid });
}

trait UsizeExtensions {
    fn increment(&mut self, limit: usize);
}

impl UsizeExtensions for usize {
    fn increment(&mut self, limit: usize) {
        if *self == limit - 1 {
            *self = 0;
        }
        else  {
            *self += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct SnakeCell {
    pub index: usize,
    pub x: usize,
    pub y: usize,
}

fn parse_snake_str(snake_str: &String) -> anyhow::Result<SnakeCell> {
    let parsed_snake_item = snake_str.split(',');
    let snake_item_list: Vec<&str> = parsed_snake_item.collect();
    return Ok(SnakeCell {
        x: snake_item_list[0].trim().parse()?,
        y: snake_item_list[1].trim().parse()?,
        index: snake_item_list[2].trim().parse()?
    });
}

fn parse_apple_str(apple_str: &String) -> anyhow::Result<(usize, usize)> {
    let parsed_apple_str = apple_str.split(',');
    let apple_list: Vec<&str> = parsed_apple_str.collect();
    let apple_x: usize = apple_list[0].trim().parse()?;
    let apple_y: usize = apple_list[1].trim().parse()?;
    return Ok((apple_x, apple_y));
}

fn get_new_grid() -> Vec<Row> {
    let mut grid: Vec<Row> = generate_empty_grid();
    let snake_start = 5;
    let apple_start_x = 7;
    let apple_start_y = 7;

    grid[snake_start].cells[snake_start].cell_type = Some("snake".to_string());
    grid[snake_start].cells[snake_start].snake_index = 0;

    grid[snake_start - 1].cells[snake_start].cell_type = Some("snake".to_string());
    grid[snake_start - 1].cells[snake_start].snake_index = 1;

    grid[apple_start_y].cells[apple_start_x].cell_type = Some("apple".to_string());
    
    return grid;
}

#[get("/fun-nonsense/htmx-snake/reset")]
pub async fn snake_reset(app_state: web::Data<AppState>) -> HttpResponse {
    return app_state.render_template("fun_nonsense/htmx_snake/htmx_snake_gameboard.html", context! { grid => get_new_grid() });
}
