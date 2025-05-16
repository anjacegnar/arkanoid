use crate::{ball::Ball, paddle::Paddle, brick::Brick, utils::{PowerUp, PowerUpType}, level::Level};
use macroquad::prelude::*;

pub struct Game {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
    powerups: Vec<PowerUp>,
    extend_paddle_time_left: f32,
    levels: Vec<Level>,
    current_level: usize,
}

impl Game {
    pub fn new() -> Self {
        let levels = Level::all();
        let current_level = 0;
        let bricks = levels[current_level].spawn_bricks();

        Self {
            ball: Ball::new(vec2(400.0, 300.0), vec2(200.0, -200.0)),
            paddle: Paddle::new(350.0, 20.0, 100.0),
            bricks,
            powerups: Vec::new(),
            extend_paddle_time_left: 0.0,
            levels,
            current_level,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.ball.update(dt);
        self.ball.collide_walls(screen_width(), screen_height());
    
        for brick in &mut self.bricks {
            if !brick.destroyed && self.ball.collide_brick(brick) {
                brick.destroyed = true;

                if rand::gen_range(0, 100) < 7 {
                    self.powerups.push(PowerUp::new(brick.center(), PowerUpType::ExtendPaddle));
                }
                
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

        if self.bricks.iter().all(|b| b.destroyed) {
            self.next_level();
        }
    
        if self.ball.vel.y > 0.0 && self.ball.collide_paddle(&self.paddle) {
            self.ball.bounce_y();
            let paddle_y = screen_height() - self.paddle.height;
            self.ball.pos.y = paddle_y - self.ball.radius;
        }
    
        self.paddle.update(dt);

        for pu in &mut self.powerups {
            pu.update(dt);
            if pu.active 
                    && pu.pos.y >= screen_height() - self.paddle.height
                    && pu.pos.x >= self.paddle.x
                    && pu.pos.x <= self.paddle.x + self.paddle.width
            {
                if self.extend_paddle_time_left <= 0.0 {
                    self.paddle.width *= 1.5;
                }
                self.extend_paddle_time_left = 10.0;
                pu.active = false;
            }
        }
        self.powerups.retain(|pu| pu.active && pu.pos.y <= screen_height());
        
        if self.extend_paddle_time_left > 0.0 {
            self.extend_paddle_time_left -= dt;
            if self.extend_paddle_time_left <= 0.0 {
                self.paddle.width /= 1.5;
                self.extend_paddle_time_left = 0.0;
            }
        }
    }    

    pub fn draw(&self, extend_texture: &Texture2D) {
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
        for pu in &self.powerups {
            let size = Vec2::new(42.0, 32.0);
            draw_texture_ex(
                extend_texture,
                pu.pos.x - size.x / 2.0,
                pu.pos.y - size.y / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(size),
                    ..Default::default()
                },
            );
        }
    }

    fn load_level(&mut self) {
        let lvl = &self.levels[self.current_level];
        self.bricks = lvl.brick_positions
            .iter()
            .map(|&(x, y)| Brick::new(x, y))
            .collect();
    }

    fn next_level(&mut self) {
        self.current_level += 1;
        if self.current_level < self.levels.len() {
            self.load_level();
        } else {
            // game-over ali victory
        }
    }

}
