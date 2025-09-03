use macroquad::prelude::*;
use crate::model::ball::Ball;
use crate::model::paddle::Paddle;
use crate::model::powerup::PowerUp;
use crate::model::brick::Brick;
use crate::model::level::{Level, RandomLevelCfg, random_positions, assign_random_brick_kinds};

#[derive(Clone, Debug)]
pub struct World {
    pub balls: Vec<Ball>,
    pub paddle: Paddle,
    pub bricks: Vec<Brick>,
    pub powerups: Vec<PowerUp>,

    pub score: i32,
    pub lives: i32,
    pub current_level: usize,

    pub extend_time_left: f32,
    pub slow_time_left: f32,
    pub slow_factor: f32,

    pub base_paddle_width: f32,
}

impl World {
    pub fn new() -> Self {
        let paddle = Paddle::new_centered(screen_width());
        let mut w = Self {
            balls: vec![Ball::new_at(vec2(screen_width() * 0.5, screen_height() * 0.5))],
            paddle,
            bricks: Vec::new(),
            powerups: Vec::new(),
            score: 0,
            lives: 3,
            current_level: 0,
            extend_time_left: 0.0,
            slow_time_left: 0.0,
            slow_factor: 1.0,
            base_paddle_width: 0.0,
        };
        w.base_paddle_width = w.paddle.width;
        w.load_level();
        w
    }

    pub fn load_level(&mut self) {
        if self.current_level == 0 {
            let positions = Level::heart_positions_centered();
            self.bricks = Level { brick_positions: positions }.spawn_bricks();
        } else {
            let positions = random_positions(None, RandomLevelCfg::default());
            self.bricks = Level { brick_positions: positions }.spawn_bricks();
            assign_random_brick_kinds(&mut self.bricks);
        }
    }

    pub fn apply_extend_paddle(&mut self, duration: f32, scale: f32) {
        self.extend_time_left = self.extend_time_left.max(duration);
        self.paddle.width = self.base_paddle_width * scale;
    }

    pub fn apply_slow_ball(&mut self, factor: f32, duration: f32) {
        self.slow_factor = factor;
        self.slow_time_left = duration;
    }

    pub fn reset_after_life_loss(&mut self) {
        let w = screen_width();
        let h = screen_height();

        self.paddle.x  = w * 0.5 - self.paddle.width * 0.5;
        self.paddle.vx = 0.0;

        self.balls.clear();
        self.balls.push(Ball::new_at(vec2(w * 0.5, h * 0.5)));

        self.slow_factor = 1.0;
        self.slow_time_left = 0.0;
        self.extend_time_left = 0.0;
        self.paddle.width = self.base_paddle_width;
    }

    pub fn advance_level(&mut self) {
        self.current_level += 1;

        self.powerups.clear();
        self.slow_factor = 1.0;
        self.slow_time_left = 0.0;
        self.extend_time_left = 0.0;
        self.paddle.width = self.base_paddle_width;

        self.load_level();

        self.balls.clear();
        self.balls.push(crate::model::ball::Ball::new_at(macroquad::prelude::vec2(
            macroquad::prelude::screen_width() * 0.5,
            macroquad::prelude::screen_height() * 0.5,
        )));
    }
}
