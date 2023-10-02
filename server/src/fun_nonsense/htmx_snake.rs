use actix_web::{web, get, post, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use minijinja::context;
use models::AppState;

const GRID_SIZE: u16 = 15;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Row {
    cells: Vec<Cell>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cell {
    cell_type: Option<String>,
    snake_index: u16,
    pos: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SnakeResponse {
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
            let mut cell: Cell = Cell {
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
    return app_state.render_template("htmx_snake.html", &req, context! { grid => get_new_grid() });
}

#[post("/fun-nonsense/htmx-snake/step")]
pub async fn snake_step(app_state: web::Data<AppState>, req_data: web::Json<SnakeResponse>, req: HttpRequest) -> HttpResponse {
    let mut grid = generate_empty_grid();

    

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

    for current_snake_cell in &snake_cells {
        if current_snake_cell.index == 0 { 
            let mut head_x = current_snake_cell.x;
            let mut head_y = current_snake_cell.y;

            match req_data.direction.as_str() {
                "1" => {
                    if head_x == 0 {
                        head_x = (GRID_SIZE - 1).try_into().unwrap();
                    }
                    else {
                        head_x -= 1;
                    }
                },
                "2" => {
                    if head_y == 0 {
                        head_y = (GRID_SIZE - 1).try_into().unwrap();
                    }
                    else {
                        head_y -= 1;
                    }
                },
                "3" => {
                    if head_x == GRID_SIZE - 1 {
                        head_x = 0;
                    }
                    else {
                        head_x += 1;
                    }
                },
                "4" => {
                    if head_y == GRID_SIZE - 1 {
                        head_y = 0;
                    }
                    else {
                        head_y += 1;
                    }
                },
                &_ => { return HttpResponse::BadRequest().finish(); }

            }

            let temp_new_head = grid[usize::from(head_y)].cells[usize::from(head_x)].clone();
            if temp_new_head.cell_type == Some("apple".to_string()) {
                let old_tail = snake_cells.last().unwrap();
                let new_tail = &mut grid[usize::from(old_tail.y)].cells[usize::from(old_tail.y)];
                new_tail.cell_type = Some("snake".to_string());
                new_tail.snake_index = u16::try_from(snake_len).unwrap();
            }
            else if temp_new_head.cell_type == Some("snake".to_string()) {
                return app_state.render_template("htmx_snake_loss.html", &req, context! { grid => grid });
            }
            let new_head = &mut grid[usize::from(head_y)].cells[usize::from(head_x)];
            new_head.cell_type = Some("snake".to_string());
            new_head.snake_index = 0;

            continue; 
        }

        let old_snake_cell = &snake_cells[usize::from(current_snake_cell.index - 1)];
        let selected_grid_cell = &mut grid[usize::from(old_snake_cell.y)].cells[usize::from(old_snake_cell.x)];
        selected_grid_cell.snake_index = current_snake_cell.index;
        selected_grid_cell.cell_type = Some("snake".to_string());
    }

    return app_state.render_template("htmx_snake_gameboard.html", &req, context! { grid => grid });
}

#[derive(Clone)]
struct SnakeCell {
    pub index: u16,
    pub x: u16,
    pub y: u16,
}

fn parse_snake_str(snake_str: &String) -> anyhow::Result<SnakeCell> {
    let parsed_snake_item = snake_str.split(',');
    let snake_item_list: Vec<&str> = parsed_snake_item.collect();
    return Ok(SnakeCell {
        x: snake_item_list[0].parse()?,
        y: snake_item_list[1].parse()?,
        index: snake_item_list[2].parse()?
    });
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
    println!("{:?}", grid);
    
    return grid;
}

#[get("/fun-nonsense/htmx-snake/reset")]
pub async fn snake_reset(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    return app_state.render_template("htmx_snake_gameboard.html", &req, context! { grid => get_new_grid() });
}
