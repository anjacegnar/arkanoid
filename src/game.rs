use crate::{ball::Ball, paddle::Paddle, brick::Brick};
use macroquad::prelude::*;

pub struct Game {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(vec2(350.0, 300.0), vec2(200.0, -200.0)),
            paddle: Paddle::new(350.0, 12.0, 100.0),
            bricks: Brick::layout(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.ball.update(dt);
        self.ball.collide_walls(screen_width(), screen_height());
    
        for brick in &mut self.bricks {
            if !brick.destroyed && self.ball.collide_brick(brick) {
                brick.destroyed = true;
                self.ball.bounce_y();
            }
        }
    
        if self.ball.collide_paddle(&self.paddle) {
            self.ball.bounce_y();
        }
    
        self.paddle.update(dt);
    }    

    pub fn draw(&self) {
        // nariše bloke
        for brick in &self.bricks {
            if !brick.destroyed {
                draw_rectangle(brick.x, brick.y, brick.width, brick.height, GRAY);
            }
        }
        // nariše žogico
        draw_circle(self.ball.pos.x, self.ball.pos.y, self.ball.radius, WHITE);
        // nariše ploščico
        draw_rectangle(self.paddle.x, screen_height() - self.paddle.height,
                       self.paddle.width, self.paddle.height, BLUE);
    }
}
