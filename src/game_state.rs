use crate::{GRID_X_SIZE, GRID_Y_SIZE};
use std::fs::read_to_string;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {Paused, Running, GameOver}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {Up, Down, Left, Right}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point(pub i32, pub i32);

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

pub struct GameContext {
    pub snake: Vec<Point>,
    pub snake_dir: Direction,
    pub food: Point,
    pub score: i32,
    pub state: GameState,
    pub highscore: Option<i32>,
    pub rng: rand::rngs::ThreadRng,
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut rng = rand::thread_rng();

        let highscore = match read_to_string("highscore.txt") {
            Ok(content) => content.trim().parse().ok(),
            Err(_) => None,
        };

        GameContext {
            snake: vec![Point(2, 0), Point(1, 0), Point(0, 0)],
            snake_dir: Direction::Right,
            food: Point(rng.gen_range(0..GRID_X_SIZE), rng.gen_range(0..GRID_Y_SIZE)),
            score: 0,
            state: GameState::Paused,
            highscore,
            rng,
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        match self.state {
            GameState::Running => self.update_running(),
            GameState::Paused => Ok(()),
            GameState::GameOver => Ok(())
        }
    }

    pub fn update_running(&mut self) -> Result<(), String> {
        let head_position = self.snake.first().unwrap();
        let next_head_position = match self.snake_dir {
            Direction::Up => *head_position + Point(0, -1),
            Direction::Down => *head_position + Point(0, 1),
            Direction::Right => *head_position + Point(1, 0),
            Direction::Left => *head_position + Point(-1, 0),
        };
        
        // Hitting a wall is game over
        if next_head_position.0 < 0
            || next_head_position.0 >= GRID_X_SIZE
            || next_head_position.1 < 0
            || next_head_position.1 >= GRID_Y_SIZE
        // Hitting the snake is game over
            || self.snake.contains(&next_head_position) 
        {
            if self.score > self.highscore.unwrap_or(0) {
                std::fs::write("highscore.txt", self.score.to_string()).unwrap();
            }
            self.state = GameState::GameOver;
            return Ok(());
        }

        // Moving the snake
        self.snake.insert(0, next_head_position);
        if next_head_position == self.food {
            self.score += 1;

            // If the snake is too big, we select a food position in the elements free
            if GRID_X_SIZE * GRID_Y_SIZE <= self.snake.len() as i32 + 20 {
                let mut available_points = Vec::new();
                for x in 0..GRID_X_SIZE {
                    for y in 0..GRID_Y_SIZE {
                        let point = Point(x, y);
                        if !self.snake.contains(&point) {
                            available_points.push(point);
                        }
                    }
                }
                self.food = available_points[self.rng.gen_range(0..available_points.len())];
            }

            // Select new food position, it needs to be outside the snake
            self.food = Point(self.rng.gen_range(0..GRID_X_SIZE), self.rng.gen_range(0..GRID_Y_SIZE));
            while self.food == next_head_position || self.snake.contains(&self.food) {
                self.food = Point(self.rng.gen_range(0..GRID_X_SIZE), self.rng.gen_range(0..GRID_Y_SIZE));
            }
            println!("Food at {:?}", self.food);
        } else {
            self.snake.pop();
        }
        
        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.snake_dir != Direction::Down && self.snake[1] != self.snake[0] + Point(0,-1) {
            self.snake_dir = Direction::Up;
        }
    }
    
    pub fn move_down(&mut self) {
        if self.snake_dir != Direction::Up && self.snake[1] != self.snake[0] + Point(0,1) {
            self.snake_dir = Direction::Down;
        }
    }
    
    pub fn move_right(&mut self) {
        if self.snake_dir != Direction::Left && self.snake[1] != self.snake[0] + Point(1,0) {
            self.snake_dir = Direction::Right;
        }
    }
    
    pub fn move_left(&mut self) {
        if self.snake_dir != Direction::Right && self.snake[1] != self.snake[0] + Point(-1,0) {
            self.snake_dir = Direction::Left;
        }
    }
    
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
            GameState::GameOver => {
                self.snake = vec![Point(2, 0), Point(1, 0), Point(0, 0)];
                self.snake_dir = Direction::Right;
                self.food = Point(self.rng.gen_range(0..GRID_X_SIZE), self.rng.gen_range(0..GRID_Y_SIZE));
                self.score = 0;
                self.highscore = read_to_string("highscore.txt")
                    .ok()
                    .and_then(|content| content.trim().parse().ok());
                GameState::Paused
            }
        }
    }
}