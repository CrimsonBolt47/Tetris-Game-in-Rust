use macroquad::prelude::*;


const BLOCK_SIZE: f32 = 30.0;
const BORDER_THICKNESS: f32 = 1.0; 
const BORDER_COLOR: Color = BLACK;

#[derive(Debug,Clone)]
enum TetriminoKind {
    I, J, L, O, S, T, Z
}

#[derive(Debug,Clone)]
struct Tetrimino {
    kind: TetriminoKind,
    color: Color,
    shape: [[u8; 4]; 4]
}
impl Tetrimino {
    const fn new(kind: TetriminoKind) ->Self {
        match kind
        {
            TetriminoKind::I => Tetrimino {kind: TetriminoKind::I,
                        color: Color::from_rgba(100, 20, 40, 255),
                            shape: [[0, 0, 1, 0],
                                    [0, 0, 1, 0],
                                    [0, 0, 1, 0],
                                    [0, 0, 1, 0]]},
            TetriminoKind::J => Tetrimino {kind: TetriminoKind::J,
                        color: Color::from_rgba(10, 200, 40, 255),
                            shape: [[1, 0, 0, 0],
                                    [1, 1, 1, 0],
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
            TetriminoKind::L => Tetrimino {kind: TetriminoKind::L,
                        color: Color::from_rgba(10, 20, 240, 255),
                            shape: [[0, 0, 1, 0],
                                    [1, 1, 1, 0],
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
            TetriminoKind::O => Tetrimino {kind: TetriminoKind::O,
                        color: Color::from_rgba(200, 200, 40, 255),
                            shape: [[1, 1, 0, 0],
                                    [1, 1, 0, 0],
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
            TetriminoKind::S => Tetrimino {kind: TetriminoKind::S,
                        color: Color::from_rgba(200, 20, 200, 255),
                            shape: [[0, 1, 1, 0],
                                    [1, 1, 0, 0],                                
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
            TetriminoKind::T => Tetrimino {kind: TetriminoKind::T,
                        color: Color::from_rgba(20, 200, 200, 255),
                            shape: [[0, 1, 0, 0],
                                    [1, 1, 1, 0],
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
            TetriminoKind::Z => Tetrimino {kind: TetriminoKind::Z,
                        color: Color::from_rgba(200, 100, 20, 255),
                            shape: [[1, 1, 0, 0         ],
                                    [0, 1, 1, 0],
                                    [0, 0, 0, 0],
                                    [0, 0, 0, 0]]},
        }
    }
}

#[derive(Debug,Clone)]
struct GameState {
    game_over:bool,
    grid: [[Option<Color>; 10]; 24],
    points: u8,
    curr_ttmo: Tetrimino,
    next_ttmo: Tetrimino,
    // FIX: Using i32 for position allows pieces to reach the left wall correctly
    ttmo_row: i32, 
    ttmo_col: i32,
}
fn pick_rand_tetrimino() -> Tetrimino {
    let rand_num = rand::gen_range(0,7);
    match rand_num {
        0 => Tetrimino::new(TetriminoKind::I),
        1 => Tetrimino::new(TetriminoKind::J),
        2 => Tetrimino::new(TetriminoKind::L),
        3 => Tetrimino::new(TetriminoKind::O),
        4 => Tetrimino::new(TetriminoKind::S),
        5 => Tetrimino::new(TetriminoKind::T),
        6 => Tetrimino::new(TetriminoKind::Z),
        _ => Tetrimino::new(TetriminoKind::I),
    }
}
#[macroquad::main("Tetris")]
async fn main() {
    let delay: f64 = 0.5;
    let mut time = get_time();
    let mut gamestate = GameState {game_over : false, points : 0,curr_ttmo:pick_rand_tetrimino(),next_ttmo:pick_rand_tetrimino(),ttmo_col: 3, ttmo_row: 0, grid: [[None; 10]; 24]};

    loop {
        clear_background(BLACK);

        draw_rectangle(0.,0.,300.,720.,GRAY);
        render_grid(&gamestate.grid);

        if is_key_pressed(KeyCode::Left)
        {   
            let new_col = gamestate.ttmo_col - 1; 
            let new_row = gamestate.ttmo_row;
            if !is_hit(&gamestate.grid, &gamestate.curr_ttmo, new_row, new_col) 
            {
                gamestate.ttmo_col = new_col;
            }
        } 
        else if is_key_pressed(KeyCode::Right) {
            let new_col = gamestate.ttmo_col + 1;
            let new_row = gamestate.ttmo_row; 
            if !is_hit(&gamestate.grid, &gamestate.curr_ttmo, new_row, new_col) 
            {
                gamestate.ttmo_col = new_col;
            }
        }  
        else if is_key_pressed(KeyCode::Down) {
            let new_col = gamestate.ttmo_col;
            let new_row = gamestate.ttmo_row + 1;
            if !is_hit(&gamestate.grid, &gamestate.curr_ttmo, new_row, new_col) 
            {
                gamestate.ttmo_row = new_row;
            }
        }
        else if is_key_pressed(KeyCode::Up) {
            try_rotate_tetrimino(&mut gamestate);
        }
        
        render_curr_tetrimino(&gamestate.curr_ttmo, gamestate.ttmo_row, gamestate.ttmo_col);

        render_next_ttmo(&gamestate.next_ttmo);

        render_points(gamestate.points);

        if gamestate.game_over {
            draw_text("Game Over!", 80., 300., 50., RED);
            break;
        }
        
        if get_time() - time > delay {
            time = get_time();
            
            let next_row = gamestate.ttmo_row + 1;
            let curr_col = gamestate.ttmo_col;
            
            if is_hit(&gamestate.grid, &gamestate.curr_ttmo, next_row, curr_col) 
            {
               reset_grid(&mut gamestate);
            }
            else {
                gamestate.ttmo_row = next_row;
            }
        }
        next_frame().await;
    }
}

// FIX: Added border logic
fn render_curr_tetrimino(curr_ttmo: &Tetrimino, ttmo_row: i32, ttmo_col: i32) {
    for r in 0..4 {
        for c in 0..4 {
            if curr_ttmo.shape[r][c] == 1 {
                let x = (ttmo_col + c as i32) as f32 * BLOCK_SIZE;
                let y = (ttmo_row + r as i32) as f32 * BLOCK_SIZE;

                if x >= 0.0 && y >= 0.0 {
                    // 1. Draw the border rectangle
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BORDER_COLOR);
                    
                    // 2. Draw the inner block
                    draw_rectangle(x + BORDER_THICKNESS, 
                                   y + BORDER_THICKNESS, 
                                   BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                                   BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                                   curr_ttmo.color);
                }
            }
        }
    }
}

fn is_hit(grid: &[[Option<Color>; 10]; 24], curr_ttmo: &Tetrimino, ttmo_row: i32, ttmo_col: i32) -> bool {
    for r in 0..4 {
        for c in 0..4 {
            if curr_ttmo.shape[r][c] == 1 {
                
                let check_col = ttmo_col + c as i32;
                let check_row = ttmo_row + r as i32;
                
                // Check all wall boundaries
                if check_col < 0 || check_col >= 10 || check_row >= 24 {
                    return true;
                }
                
                // Check for existing blocks (only if within vertical bounds)
                if check_row >= 0 { 
                    let grid_row = check_row as usize;
                    let grid_col = check_col as usize;

                    if grid[grid_row][grid_col].is_some() {
                        return true;
                    }
                }
            }
        }
    }
    false
}  

fn fill_grid_with_tetrimino(grid: &mut [[Option<Color>; 10]; 24], curr_ttmo: &Tetrimino, ttmo_row: i32, ttmo_col: i32) {
    for r in 0..4 {
        for c in 0..4 {
            if curr_ttmo.shape[r][c] == 1 {
                let global_row = ttmo_row + r as i32;
                let global_col = ttmo_col + c as i32;
                
                if global_row >= 0 && global_row < 24 && global_col >= 0 && global_col < 10 {
                    grid[global_row as usize][global_col as usize] = Some(curr_ttmo.color);
                }
            }
        }
    }
}

fn try_rotate_tetrimino(gamestate: &mut GameState) {
    let original_col = gamestate.ttmo_col;
    let original_shape = gamestate.curr_ttmo.shape;

    rotate_tetrimino(&mut gamestate.curr_ttmo);
    let kick_tests: [i32; 5] = [0, 1, -1, 2, -2]; 
    for kick in kick_tests.iter() {
        let new_col = original_col + kick;
        if !is_hit(&gamestate.grid, &gamestate.curr_ttmo, gamestate.ttmo_row, new_col) {
            gamestate.ttmo_col = new_col;
            return;
        }
    }

    gamestate.curr_ttmo.shape = original_shape;
}

fn reset_grid(gamestate: &mut GameState) {
    fill_grid_with_tetrimino(&mut gamestate.grid, &gamestate.curr_ttmo, gamestate.ttmo_row, gamestate.ttmo_col);
    let earned_points = row_cleared(&mut gamestate.grid);
    gamestate.game_over = is_game_over(&gamestate.grid);
    gamestate.points += earned_points*10 + 1;
    gamestate.curr_ttmo = gamestate.next_ttmo.clone();
    gamestate.next_ttmo = pick_rand_tetrimino();
    gamestate.ttmo_row = 0;
    gamestate.ttmo_col = 3;
}

fn render_grid(grid: &[[Option<Color>; 10]; 24],) {
    for r in 0..24 {
        for c in 0..10 {
            if let Some(color) = grid[r][c] {
                let x = c as f32 * BLOCK_SIZE;
                let y = r as f32 * BLOCK_SIZE;
                
                draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BORDER_COLOR);
                
                draw_rectangle(x + BORDER_THICKNESS, 
                               y + BORDER_THICKNESS, 
                               BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                               BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                               color);
            }
        }
    }
}

fn rotate_tetrimino(ttmo: &mut Tetrimino) {
    let mut new_shape = [[0u8;4];4];
    for r in 0..4 {
        for c in 0..4 {
            new_shape[c][3 - r] = ttmo.shape[r][c];
        }
    }
    ttmo.shape = new_shape;
}

fn row_cleared(grid: &mut [[Option<Color>; 10]; 24]) -> u8 {
    let mut cleared_rows = 0;
    for r in (0..24).rev() {
        if grid[r].iter().all(|&cell| cell.is_some()) {
            for row_to_move in (1..=r).rev() {
                grid[row_to_move] = grid[row_to_move - 1];
            }
            grid[0] = [None; 10];
            cleared_rows += 1;
        }
    }
    cleared_rows
}

fn render_next_ttmo(next_ttmo: &Tetrimino) {
    let offset_x = 320.;
    let offset_y = 50.;
    draw_text("Next:", offset_x, offset_y - 10., 20., WHITE);
    for r in 0..4 {
        for c in 0..4 {
            if next_ttmo.shape[r][c] == 1 {
                let draw_x = offset_x + c as f32 * 30.;
                let draw_y = offset_y + r as f32 * 30.;
                draw_rectangle(draw_x, draw_y, BLOCK_SIZE, BLOCK_SIZE, BORDER_COLOR);
                draw_rectangle(draw_x + BORDER_THICKNESS, 
                               draw_y + BORDER_THICKNESS, 
                               BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                               BLOCK_SIZE - 2.0 * BORDER_THICKNESS, 
                               next_ttmo.color);
            }
        }
    }
}

fn render_points(points: u8) {
    let offset_x = 320.;
    let offset_y = 250.;
    draw_text(&format!("Points: {}", points), offset_x, offset_y, 20., WHITE);
}

fn is_game_over(grid: &[[Option<Color>; 10]; 24]) -> bool {
    for c in 0..10 {
        if grid[0][c].is_some() {
            return true;
        }
    }
    false
}