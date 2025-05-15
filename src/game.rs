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
            ball: Ball::new(vec2(400.0, 300.0), vec2(200.0, -200.0)),
            paddle: Paddle::new(350.0, 20.0, 100.0),
            bricks: Brick::layout(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.ball.update(dt);
        self.ball.collide_walls(screen_width(), screen_height());
    
        for brick in &mut self.bricks {
            if !brick.destroyed && self.ball.collide_brick(brick) {
                brick.destroyed = true;
                
                let cx = self.ball.pos.x;
                let cy = self.ball.pos.y;
                let r = self.ball.radius;
                let bx = brick.x;
                let by = brick.y;
                let bw = brick.width;
                let bh = brick.height;

                let overlap_left = (cx + r) - bx;
                let overlap_right = (bx + bw) - (cx - r);
                let overlap_top = (cy + r) - by;
                let overlap_bottom = (by + bh) - (cy - r);

                let min_overlap = overlap_left
                    .min(overlap_right)
                    .min(overlap_top)
                    .min(overlap_bottom);

                if min_overlap == overlap_left || min_overlap == overlap_right {
                    self.ball.bounce_x();
                } else {
                    self.ball.bounce_y();
                }
    
                break;
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
