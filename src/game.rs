use crate::{
    snake::{Direction, Snake},
    utils::draw_block,
};
use piston_window::{types::Color, *};
use rand::{thread_rng, Rng};

const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 0.0];
const LINE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub struct Game {
    time: f64,
    snake: Snake,
    entity_size: f64,

    // Food
    food: (bool, i32, i32),

    // Game Size
    pub width: i32,
    pub height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32, entity_size: f64) -> Game {
        Game {
            time: 0.0,
            snake: Snake::new(9, 9),
            entity_size,
            food: (false, 0, 0),
            width,
            height,
        }
    }
    pub fn key_down(&mut self, key: Key) {
        match key {
            Key::Up | Key::W => self.snake.change_direction(Direction::Up),
            Key::Down | Key::S => self.snake.change_direction(Direction::Down),
            Key::Left | Key::A => self.snake.change_direction(Direction::Left),
            Key::Right | Key::D => self.snake.change_direction(Direction::Right),
            _ => {}
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(0..self.width / self.entity_size as i32);
        let mut y = rng.gen_range(0..self.height / self.entity_size as i32);
        while self.snake.is_overlapped(x, y) {
            x = rng.gen_range(0..self.width / self.entity_size as i32);
            y = rng.gen_range(0..self.height / self.entity_size as i32);
        }

        self.food = (true, x, y);
    }

    pub fn draw_stats(&mut self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        let score = self.snake.get_score();
        let score_text = format!("Score: {}", score);

        text(
            [1.0, 1.0, 1.0, 1.0],
            20,
            &score_text,
            glyphs,
            context.transform.trans(10.0, 30.0),
            graphics,
        )
        .unwrap();
    }

    pub fn draw(&mut self, context: &Context, graphics: &mut G2d) {
        clear(BACKGROUND_COLOR, graphics);
        self.snake.draw(context, graphics);

        if self.food.0 {
            draw_block(FOOD_COLOR, self.food.1, self.food.2, context, graphics);
        } else {
            self.add_food();
        }
        for i in 0..self.width / self.entity_size as i32 + 1 {
            line_from_to(
                LINE_COLOR,
                1.0,
                [i as f64 * self.entity_size + 100.0, 100.0],
                [
                    i as f64 * self.entity_size + 100.0,
                    (self.height + 100) as f64,
                ],
                context.transform,
                graphics,
            );
        }
        for i in 0..(self.height / self.entity_size as i32) + 1 {
            line_from_to(
                LINE_COLOR,
                1.0,
                [100.0, i as f64 * self.entity_size + 100.0],
                [
                    (self.width + 100) as f64,
                    i as f64 * self.entity_size + 100.0,
                ],
                context.transform,
                graphics,
            );
        }
    }

    pub fn update(&mut self, time: f64) {
        self.time += time;

        if self.time > 0.1 {
            self.time = 0.0;
            self.snake.update_snake();
        }
        if self.food.0 {
            if self.snake.is_overlapped(self.food.1, self.food.2) {
                self.food.0 = false;
                self.snake.eat();
            }
        }
    }
}
