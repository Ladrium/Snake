use crate::utils::draw_block;
use piston_window::{types::Color, Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    x: i32,
    y: i32,
}
pub struct Snake {
    body: LinkedList<Cell>,
    direction: Direction,
    popped: Cell,
}
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Cell> = LinkedList::new();
        body.push_back(Cell { x, y });
        Snake {
            body,
            direction: Direction::None,
            popped: Cell { x: 0, y: 0 },
        }
    }

    pub fn get_score(&self) -> usize {
        self.body.len()
    }

    fn head(&self) -> (i32, i32) {
        let position = self.body.front().unwrap();
        (position.x, position.y)
    }

    pub fn eat(&mut self) {
        println!("{:?}", self.popped);
        self.body.push_back(self.popped);
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for cell in &self.body {
            draw_block(SNAKE_COLOR, cell.x, cell.y, con, g);
        }
    }

    pub fn update_snake(&mut self) {
        let (x, y) = self.head();
        let new_head = match self.direction {
            Direction::Up => Cell { x, y: y - 1 },
            Direction::Down => Cell { x, y: y + 1 },
            Direction::Right => Cell { x: x + 1, y },
            Direction::Left => Cell { x: x - 1, y },
            Direction::None => Cell { x, y },
        };
        self.body.push_front(new_head);
        let popped = self.body.pop_back();
        self.popped = popped.unwrap();
    }

    pub fn change_direction(&mut self, d: Direction) {
        self.direction = d;
    }

    pub fn is_overlapped(&self, x: i32, y: i32) -> bool {
        for cell in &self.body {
            if cell.x == x && cell.y == y {
                return true;
            }
        }
        false
    }
}
