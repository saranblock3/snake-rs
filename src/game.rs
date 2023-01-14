use crate::snake::Snake;
use crate::direction::Direction;
use crate::food::Food;
use std::{thread, time::{Duration, Instant}};
use crossterm::event::{poll, Event, read, KeyEvent, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use rand::prelude::*;
use colored::Colorize;

const INTERVAL: u64 = 200;

pub struct Game {
    score: i32,
    snake: Snake,
    food: Food,
    playing: bool,
    game_width: i32,
    game_height: i32,
}

impl Game {
    pub fn new(game_width: i32, game_height: i32) -> Game {
        let mut game = Game {
            score: 0,
            snake: Snake::new(game_width/2, game_height/2),
            food: Food::new(0, 0),
            playing: true,
            game_width: game_width,
            game_height: game_height,
        };
        let (x, y) = game.generate_point_outside_snake();
        game.food = Food::new(x, y);
        game
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        print!("\x1B[2J\x1B[1;1H");
        while self.playing {
            let now = Instant::now();
            let interval = Duration::from_millis(INTERVAL);
            self.draw();
            if now.elapsed() < interval {
                self.handle_key_press(interval - now.elapsed());
            }
            if now.elapsed() < interval {
                thread::sleep(interval - now.elapsed());
            }
            self.handle_eating();
            self.update();
            print!("\x1B[2J\x1B[1;1H");
            if self.has_collided_wall() || self.has_collided_self() {
                self.playing = false;
            }
        }
        disable_raw_mode().unwrap();
    }

    fn update(&mut self) {
        self.snake.slither();
    }

    fn draw(&self) {
        let mut board = String::from("");
        for _ in 0..self.game_width+2 {board += "██";}
        board += "\r\n";
        for i in 0..self.game_height {
            for j in 0..self.game_width {
                if j == 0 {
                    board += "██";
                }
				if self.snake.contains_point(j, i) {
					board += &format!("{}", "██".green());
				} else if j == self.food.postion.0 && i == self.food.postion.1 {
                    board += &format!("{}", "██".red());
                } else {
                    board += "  ";
                }
                if j == self.game_width-1 {
                    board += "██";
                }
            }
            board += "\r\n";
        }
        for _ in 0..self.game_width+2 {board += "██"}
        board += "\r\n";
        print!("{}", board);
        print!("\r\n");
        self.print_score();
    }

    fn print_score(&self) {
        let temp = self.score.to_string();
        let digits = temp.chars();


        let zero_level0 =   "  ██████";
        let zero_level1 =   "  ██  ██";
        let zero_level2 =   "  ██  ██";
        let zero_level3 =   "  ██  ██";
        let zero_level4 =   "  ██████";

        let one_level0 =    "      ██";
        let one_level1 =    "      ██";
        let one_level2 =    "      ██";
        let one_level3 =    "      ██";
        let one_level4 =    "      ██";

        let two_level0 =    "  ██████";
        let two_level1 =    "      ██";
        let two_level2 =    "  ██████";
        let two_level3 =    "  ██    ";
        let two_level4 =    "  ██████";

        let three_level0 =  "  ██████";
        let three_level1 =  "      ██";
        let three_level2 =  "  ██████";
        let three_level3 =  "      ██";
        let three_level4 =  "  ██████";

        let four_level0 =   "  ██  ██";
        let four_level1 =   "  ██  ██";
        let four_level2 =   "  ██████";
        let four_level3 =   "      ██";
        let four_level4 =   "      ██";

        let five_level0 =   "  ██████";
        let five_level1 =   "  ██    ";
        let five_level2 =   "  ██████";
        let five_level3 =   "      ██";
        let five_level4 =   "  ██████";

        let six_level0 =    "  ██████";
        let six_level1 =    "  ██    ";
        let six_level2 =    "  ██████";
        let six_level3 =    "  ██  ██";
        let six_level4 =    "  ██████";

        let seven_level0 =  "  ██████";
        let seven_level1 =  "      ██";
        let seven_level2 =  "      ██";
        let seven_level3 =  "      ██";
        let seven_level4 =  "      ██";

        let eight_level0 =  "  ██████";
        let eight_level1 =  "  ██  ██";
        let eight_level2 =  "  ██████";
        let eight_level3 =  "  ██  ██";
        let eight_level4 =  "  ██████";

        let nine_level0 =   "  ██████";
        let nine_level1 =   "  ██  ██";
        let nine_level2 =   "  ██████";
        let nine_level3 =   "      ██";
        let nine_level4 =   "  ██████";

        let mut level0 = String::from("");
        let mut level1 = String::from("");
        let mut level2 = String::from("");
        let mut level3 = String::from("");
        let mut level4 = String::from("");

        for digit in digits {
            match digit {
                '0' => {
                    level0 += zero_level0;
                    level1 += zero_level1;
                    level2 += zero_level2;
                    level3 += zero_level3;
                    level4 += zero_level4;
                },
                '1' => {
                    level0 += one_level0;
                    level1 += one_level1;
                    level2 += one_level2;
                    level3 += one_level3;
                    level4 += one_level4;
                },
                '2' => {
                    level0 += two_level0;
                    level1 += two_level1;
                    level2 += two_level2;
                    level3 += two_level3;
                    level4 += two_level4;
                },
                '3' => {
                    level0 += three_level0;
                    level1 += three_level1;
                    level2 += three_level2;
                    level3 += three_level3;
                    level4 += three_level4;
                },
                '4' => {
                    level0 += four_level0;
                    level1 += four_level1;
                    level2 += four_level2;
                    level3 += four_level3;
                    level4 += four_level4;
                },
                '5' => {
                    level0 += five_level0;
                    level1 += five_level1;
                    level2 += five_level2;
                    level3 += five_level3;
                    level4 += five_level4;
                },
                '6' => {
                    level0 += six_level0;
                    level1 += six_level1;
                    level2 += six_level2;
                    level3 += six_level3;
                    level4 += six_level4;
                },
                '7' => {
                    level0 += seven_level0;
                    level1 += seven_level1;
                    level2 += seven_level2;
                    level3 += seven_level3;
                    level4 += seven_level4;
                },
                '8' => {
                    level0 += eight_level0;
                    level1 += eight_level1;
                    level2 += eight_level2;
                    level3 += eight_level3;
                    level4 += eight_level4;
                },
                '9' => {
                    level0 += nine_level0;
                    level1 += nine_level1;
                    level2 += nine_level2;
                    level3 += nine_level3;
                    level4 += nine_level4;
                }
                _ => (),
            }
        }

        // print!("
        //     ██████  ██████  ██████  ██████  ██████      {}\r\n
        //     ██      ██      ██  ██  ██  ██  ██      ██  {}\r\n
        //     ██████  ██      ██  ██  ██████  ██████      {}\r\n
        //         ██  ██      ██  ██  ██ ██   ██      ██  {}\r\n 
        //     ██████  ██████  ██████  ██  ██  ██████      {}\r\n
        // ", level0, level1, level2, level3, level4);

        print!("██████  ██████  ██████  ██████  ██████    {}\r\n", level0);
        print!("██      ██      ██  ██  ██  ██  ██      ██{}\r\n", level1);
        print!("██████  ██      ██  ██  ██████  ██████    {}\r\n", level2);
        print!("    ██  ██      ██  ██  ██ ██   ██      ██{}\r\n", level3);
        print!("██████  ██████  ██████  ██  ██  ██████    {}\r\n", level4);
    }

    fn has_collided_wall(&self) -> bool {
        if self.snake.head.0 < 0 {return true}
        if self.snake.head.1 < 0 {return true}
        if self.snake.head.0 >= self.game_width {return true}
        if self.snake.head.1 >= self.game_height {return true}
        false
    }

    fn has_collided_self(&self) -> bool {
        return self.snake.tail_contains_point(self.snake.head.0, self.snake.head.1);
    }

    fn generate_point_outside_snake(&self) -> (i32, i32) {
       let mut rng = rand::thread_rng(); 
       let mut x = rng.gen_range(0..self.game_width);
       let mut y = rng.gen_range(0..self.game_height);
       while self.snake.contains_point(x, y) {
        x = rng.gen_range(0..self.game_width);
        y = rng.gen_range(0..self.game_height);
       }
       return (x, y);
    }

    fn replace_food(&mut self) {
        let (x, y) = self.generate_point_outside_snake();
        self.food = Food::new(x, y);
    }

    fn handle_eating(&mut self) {
        if self.snake.head.0 == self.food.postion.0 && self.snake.head.1 == self.food.postion.1 {
            self.snake.set_growing(true);
            self.replace_food();
            self.score += 1;
        }
    }

    fn handle_key_press(&mut self, wait_for: Duration) -> Option<KeyCode> {
        let key_event = self.wait_for_key_press(wait_for)?;


        match key_event.code {
            KeyCode::Up => self.snake.set_direction(Direction::Up),
            KeyCode::Right => self.snake.set_direction(Direction::Right),
            KeyCode::Down => self.snake.set_direction(Direction::Down),
            KeyCode::Left => self.snake.set_direction(Direction::Left),
            _ => (),
        }

        None
    }

    fn wait_for_key_press(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?; 
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }
        None
    }
}
