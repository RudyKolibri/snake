use std::time::SystemTime;
use std::process::Command;
use macroquad::{prelude::*, rand::{gen_range, srand}};
// inputs : 
// 1 = right
// 2 = left
// 3 = up
// 4 = down
// 5 = nothing

const MAX_APPLE_COUNT: i8 = 1;
const TILES_PER_ROW: i32 = 17;

struct Appel {
    position : [i32; 2],
    collision : bool //if the snake hit the apple this goes to true
}
impl Copy for Appel { }
impl Clone for Appel {
    fn clone(&self) -> Appel {
        *self
    }
}
struct Snake {
    position : [i32; 2],
}
impl Copy for Snake {}
impl Clone for Snake {
    fn clone(&self) -> Snake {
        *self
    }
}

#[macroquad::main("Snake")]

async fn main() {
    //random seed
    srand((get_time() * 100000.0) as u64);
    
    let mut apple_list : Vec<Appel> = Vec::new();
    let mut snake_list : Vec<Snake> = Vec::new();
    let mut score = snake_list.len() as i64 - 1; 
    let mut new_apple= Appel {
        position : [0, 0],
        collision : false
    };
    let mut new_snake: Snake = Snake {
        position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
    };
    let mut previuse_input: i32 = 1;
    let mut previuse_snake = new_snake;
    let mut apple_count: i8 = 0;

    let mut last_input = get_input(previuse_input);

    previuse_input = last_input;
    //setup for first run

    snake_list.push(new_snake);

    let mut sys_time = SystemTime::now();
    let time_wanted = 180.;
    loop { //main game loop

        let new_sys_time = SystemTime::now();
        if new_sys_time.duration_since(sys_time).unwrap().as_millis() as f64 > time_wanted {
            sys_time = SystemTime::now();
            //setup this frame
            last_input = get_input(previuse_input);
            //code of this frame
            new_snake = previuse_snake;

            //snake movement
            if last_input == 1 {
                new_snake.position[0] = new_snake.position[0] + 1;
                snake_list.push(new_snake)
            };
            if last_input == 2 {
                new_snake.position[0] = new_snake.position[0] - 1;
                snake_list.push(new_snake)
            };
            if last_input == 3 {
                new_snake.position[1] = new_snake.position[1] - 1;
                snake_list.push(new_snake)
            };
            if last_input == 4 {
                new_snake.position[1] = new_snake.position[1] + 1;
                snake_list.push(new_snake)
            }

            //end this frame
            for i in 0..snake_list.len() {
                let index = i as usize;
                let list_snake = snake_list.clone();
                let this_snake = list_snake[index as usize];
                for j in 0..snake_list.len() {
                    let index_check = j as usize;
                    let this_snake_check = list_snake[index_check as usize];

                    if this_snake.position == this_snake_check.position {
                        if index != index_check {
                            snake_list = Vec::new();
                            apple_list = Vec::new();
                            new_snake = Snake {
                                position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
                            };
                            snake_list.push(new_snake);
                            apple_count = 0;
                            last_input = 1;
                        } 
                    }
                }
            }

            for i in 0..snake_list.len() {
                let index = i as usize;
                let list_snake = snake_list.clone();
                let this_snake = list_snake[index as usize];

                if this_snake.position[0] < 0 {
                    snake_list = Vec::new();
                    apple_list = Vec::new();
                    new_snake = Snake {
                        position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
                    };
                    snake_list.push(new_snake);
                    apple_count = 0;
                    last_input = 1;
                }
                if this_snake.position[0] > TILES_PER_ROW - 1 {
                    snake_list = Vec::new();
                    apple_list = Vec::new();
                    new_snake = Snake {
                        position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
                    };
                    snake_list.push(new_snake);
                    apple_count = 0;
                    last_input = 1;
                }
                if this_snake.position[1] < 0 {
                    snake_list = Vec::new();
                    apple_list = Vec::new();
                    new_snake = Snake {
                        position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
                    };
                    snake_list.push(new_snake);
                    apple_count = 0;
                    last_input = 1;
                }
                if this_snake.position[1] > TILES_PER_ROW - 1 {
                    snake_list = Vec::new();
                    apple_list = Vec::new();
                    new_snake = Snake {
                        position : [(TILES_PER_ROW / 2) - 1, (TILES_PER_ROW / 2)]
                    };
                    snake_list.push(new_snake);
                    apple_count = 0;
                    last_input = 1;
                }
            }

            let mut is_collision = false;

            for i in 0..apple_count {
                let index = i as usize;
                let list = apple_list.clone();
                let mut this_apple = list[index as usize];
                for j in 0..snake_list.len() {
                    let index = j as usize;
                    let list_snake = snake_list.clone();
                    let this_snake = list_snake[index as usize];
                    if this_apple.position == this_snake.position {
                        this_apple.collision = true;
                        apple_list.remove(i as usize);
                        apple_list.push(this_apple);
                    }
                }
            }
            for i in 0..apple_count {
                let index = i as usize;
                let list = apple_list.clone();
                let this_apple = list[index as usize];
                if this_apple.collision == true {
                    is_collision = true;
                    apple_list.remove(i as usize);
                    apple_count = apple_count - 1;
                }
            }

            if is_collision == false {
                snake_list.remove(0);
            }
            
            score = snake_list.len() as i64 - 1;
            
            draw_grid(); //draw all + grid
            let mut tiles_height = screen_height() as i32 / TILES_PER_ROW;
            if tiles_height * TILES_PER_ROW > screen_width() as i32 {
                tiles_height = screen_width() as i32 / TILES_PER_ROW;
            }
            let tiles_width =  tiles_height;
            if apple_count < MAX_APPLE_COUNT {
                let mut good = false;
                // create a new apple in the grid
                while good == false {
                    let x = gen_range(0, TILES_PER_ROW);
                    let y = gen_range(0, TILES_PER_ROW);
                    new_apple = Appel {
                        position : [x, y],
                        collision : false
                    };
                    let mut tester = true;
                    for i in 0..snake_list.len() {
                        let index = i as usize;
                        let list = snake_list.clone();
                        let this_snake = list[index as usize];
                        if this_snake.position == new_apple.position {
                            tester = false;
                        }
                        good = tester
                    }
                }

                apple_list.push(new_apple);
                //make the apples + 1
                apple_count += 1;
            }
            //draw all apples
            for i in 0..apple_count {
                let index = i as usize;
                let list = apple_list.clone();
                let this_apple = list[index as usize];
                draw_apple(this_apple)
            }
            for i in 0..snake_list.len() {
                let index = i as usize;
                let list = snake_list.clone();
                let this_snake = list[index as usize];
                draw_snake(this_snake)
            }
            previuse_input = last_input;
            previuse_snake = new_snake;
            draw_text(&score.to_string() as &str, ((tiles_width * (TILES_PER_ROW / 2) - ((tiles_height / 5) * (&score.to_string()).len() as i32)) - ((tiles_height / 2) * (&score.to_string()).len() as i32)) as f32 , ((tiles_height * 2) + (tiles_height) )  as f32, tiles_height as f32 * 5.0, WHITE);
            next_frame().await
        }
        
    }
}

fn draw_grid () { // function to draw grid
    //clear background and set to green
    clear_background(BLACK);
    //drawgrid
    let mut tiles_height = screen_height() as i32 / TILES_PER_ROW;
    if tiles_height * TILES_PER_ROW > screen_width() as i32 {
        tiles_height = screen_width() as i32 / TILES_PER_ROW;
    }
    let tiles_width =  tiles_height;

    for y in 0..TILES_PER_ROW { //generate the grid on screen
        for x in 0..TILES_PER_ROW {
            if (y as f32 / 2 as f32) != (y / 2) as f32 {
                if (x as f32 / 2 as f32) != (x / 2) as f32 {
                    draw_rectangle((x * tiles_width) as f32, (y * tiles_height) as f32, tiles_width as f32, tiles_height as f32, LIME)
                } else {
                    draw_rectangle((x * tiles_width) as f32, (y * tiles_height) as f32, tiles_width as f32, tiles_height as f32, DARKGREEN)
                }
            } else {
                if (x as f32 / 2 as f32) == (x / 2) as f32 {
                    draw_rectangle((x * tiles_width) as f32, (y * tiles_height) as f32, tiles_width as f32, tiles_height as f32, LIME)
                } else {
                    draw_rectangle((x * tiles_width) as f32, (y * tiles_height) as f32, tiles_width as f32, tiles_height as f32, DARKGREEN)
                }
            }
        }
    }
}

fn draw_apple(appel_draw : Appel) { // draws appel
    let mut tiles_height = screen_height() as i32 / TILES_PER_ROW;
    if tiles_height * TILES_PER_ROW > screen_width() as i32 {
        tiles_height = screen_width() as i32 / TILES_PER_ROW;
    }
    let tiles_width =  tiles_height;
    //draw things on top of it
    let x = appel_draw.position[0];
    let y =  appel_draw.position[1];
    draw_rectangle((x * tiles_width + 5 )as f32, (y * tiles_height + 5) as f32 , tiles_width as f32 - 10.0, tiles_height as f32 - 10.0, RED)
}

fn draw_snake(snake_draw : Snake) {
    let mut tiles_height = screen_height() as i32 / TILES_PER_ROW;
    if tiles_height * TILES_PER_ROW > screen_width() as i32 {
        tiles_height = screen_width() as i32 / TILES_PER_ROW;
    }
    let tiles_width =  tiles_height;
    //draw things on top of it
    let x = snake_draw.position[0];
    let y =  snake_draw.position[1];
    draw_rectangle((x * tiles_width )as f32, (y * tiles_height) as f32 , tiles_width as f32, tiles_height as f32, BLUE)
}

fn get_input(last : i32) -> i32 {
    let mut input : i32 = 5;
    // get player input and check if it is equal to a key
    // inputs : 
    // 1 = right
    // 2 = left
    // 3 = up
    // 4 = down
    // 5 = nothing
    if is_key_down(KeyCode::Right) == true && last != 2  && last != 1 {
        input = 1;
    } 
    if is_key_down(KeyCode::Left) == true && last != 1 && last != 2{
        input = 2;
    } 
    if is_key_down(KeyCode::Up) == true && last != 4  && last != 3{
        input = 3;
    } 
    if is_key_down(KeyCode::Down) == true && last != 3  && last != 4 {
        input = 4;
    } 

    if input == 5 { //if there is nothing input, return the latest one back -> this way we go the same direction
        return last as i32;
    } else {
        return input as i32;
    }
}