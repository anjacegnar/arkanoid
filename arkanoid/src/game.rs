use crate::{ball::Ball, paddle::Paddle, brick::Brick};

pub struct Game {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            paddle: Paddle::new(),
            bricks: Brick::layout(),
        }
    }

    pub fn run(&mut self) {
        // igra
        println!("Game loop started.");
    }
}
