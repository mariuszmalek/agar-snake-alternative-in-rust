use piston_window::*;
use rand::Rng;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::Snake;
use crate::enemy::Enemy;

const FPS: f64 = 10.0;
// const RESTART_TIME: f64 = 1.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0, width as i32),
        y: rng.gen_range(0, height as i32),
    }
}

pub struct Game {
    snake: Snake,
    enemy: Enemy,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            enemy: Enemy::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            over: false,
            paused: true,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    // pub fn toggle_game_state(&mut self) {
    //     if self.paused {
    //         self.start();
    //     } else {
    //         self.pause();
    //     }
    // }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        draw_block(&ctx, g, colors::FRUIT, &self.fruit);

        self.snake.draw(&ctx, g);
        self.enemy.draw(&ctx, g);

        if self.over {
            draw_overlay(&ctx, g, colors::OVERLAY, self.size)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // if self.over {
        // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            // self.check_colision() use snake.get_head_pos;
            self.waiting_time = 0.0;

            if !self.enemy.is_tail_overlapping() && !self.enemy.will_tail_overlapp() {
                self.enemy.update(self.size.0, self.size.1);
            } else {
                self.enemy = Enemy::new(calc_random_pos(self.size.0, self.size.1));
            }

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlapp() {
                self.snake.update(self.size.0, self.size.1);

                // End game if have colision with window
                if self.snake.get_head_pos().x == -1 || self.snake.get_head_pos().y == -1 {
                    self.over = true;
                }

                // When user bite a fruit
                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }

                // When user bite a enemy
                // if *self.snake.get_head_pos() == *self.enemy.get_head_pos() {
                //     self.snake.grow();
                //     self.snake.update(self.size.0, self.size.1);
                //     self.enemy = Enemy::new(calc_random_pos(self.size.0, self.size.1));
                //     self.calc_score();
                // }

                // When enemy bite a user
                // if *self.snake.get_head_pos() == *self.enemy.get_head_pos() {
                //     self.enemy.grow();
                //     self.over = true;
                // }

                // let snake_body = self.snake.get_body();
                // for pos in snake_body {
                //     // println!("{:?}", pos);
                //     if pos == *self.enemy.get_head_pos() {
                //         self.enemy.grow();
                //         self.over = true;
                //     }
                // }


            } else {
                self.over = true;
            }
        }
    }

    pub fn enemy_direction(&mut self) {

        let snake_position = self.snake.get_head_pos();
        let enemy_position = self.enemy.get_head_pos();
        let snake_next_position = self.snake.get_next_position();
        let body = self.snake.get_body();

        println!("{:?}", body);
        // println!("{:?}", snake_next_position);
        // println!("enemy: {:?}, snake: {:?}", enemy_position.x, snake_position.x);

        let x = snake_position.x > enemy_position.x;
        let y = snake_position.y > enemy_position.y;

        if x {
            println!("Go to right");
            self.enemy.set_dir(Direction::Right);
        }
        
        if !x {
            println!("Go to left");
            self.enemy.set_dir(Direction::Left);
        }

        if y {
            println!("Go to down");
            self.enemy.set_dir(Direction::Down);
        }
        
        if !y {
            println!("Go to up");
            self.enemy.set_dir(Direction::Up);
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        use keyboard::Key;

        // match key {
        //     Key::R => self.over = false, // temp solution -> replace current game state trough new one
        //     Key::Space => self.toggle_game_state(),
        //     _ => self.start(),
        // }

        match key {
            Key::R => self.restart(),
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            _ => {}
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calc_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32
    }

    fn restart(&mut self) {
        *self = Game::new(self.size.0, self.size.1);
        self.start();
    }

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}

// fn calc_not_overlapping_pos(pos_vec: Vec<Position>, width: u32, height: u32) {
//     let mut fruit_pos: Position = calc_random_pos(width, height);

//     loop {
//         // if snake_pos.y != fruit_pos.y {
//         //     break;
//         // }

//         for pos in pos_vec {
//             if
//         }

//         snake_pos = calc_random_pos(width, height);
//         fruit_pos = calc_random_pos(width, height);
//     }
// }