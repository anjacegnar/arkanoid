use crate::{ball::Ball, paddle::Paddle, brick::Brick, utils::*, level::*};
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Game {
    pub ball: Ball,
    pub paddle: Paddle,
    pub bricks: Vec<Brick>,
    pub powerups: Vec<PowerUp>,
    pub score: i32,
    pub lives: i32,
    pub extend_paddle_time_left: f32,
    pub current_level: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            ball: Ball::new(),
            paddle: Paddle::new(),
            bricks: Vec::new(),
            powerups: Vec::new(),
            score: 0,
            lives: 3,
            extend_paddle_time_left: 0.0,
            current_level: 0,
        };
        game.load_level();
        game
    }

    fn load_level(&mut self) {
        if self.current_level == 0 {
        // prvi level - srček
            self.bricks = Level { brick_positions: Level::heart_positions() }.spawn_bricks();
        } else {
            // naslednji leveli so random
            let positions = random_positions(None, RandomLevelCfg::default());
            self.bricks = Level { brick_positions: positions }.spawn_bricks();
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.paddle.update();
        self.ball.update();

        let _ = self.ball.bounce_off_paddle(&self.paddle, self.paddle.vx);

        self.ball.collide_walls(screen_width());

        let mut spawn_pu_at: Option<Vec2> = None;
        for brick in &mut self.bricks {
            if self.ball.hit_brick(brick) {
                self.score += 10;
                if gen_range(0, 100) < 7 {
                    spawn_pu_at = Some(brick.center());
                }
                break;
            }
        }
        if let Some(p) = spawn_pu_at {
            let kind = if gen_range(0, 100) < 20 {
                PowerUpType::ExtraLife
            } else {
                PowerUpType::ExtendPaddle
            };
            self.powerups.push(PowerUp::new(p, kind));
        }

        self.bricks.retain(|b| !b.destroyed);

        for pu in &mut self.powerups {
            pu.update(dt);
            if pu.active
                && pu.pos.y >= screen_height() - self.paddle.height
                && pu.pos.x >= self.paddle.x
                && pu.pos.x <= self.paddle.x + self.paddle.width
            {
                match pu.kind {
                    PowerUpType::ExtendPaddle => {
                        if self.extend_paddle_time_left <= 0.0 {
                            self.paddle.width *= 1.5;
                        }
                        self.extend_paddle_time_left = 10.0;
                    }
                    PowerUpType::ExtraLife => {
                        self.lives += 1;
                    }
                }
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

        if self.bricks.is_empty() {
            self.current_level += 1;

            self.ball.speed = (self.ball.speed * 1.05).min(600.0);

            self.load_level();
        }


        if self.ball.pos.y - self.ball.radius > screen_height() {
            self.lives -= 1;
            if self.lives > 0 {
                self.ball.reset();
                self.paddle.reset();
            } else {
                self.score = 0;
                self.lives = 3;
                self.current_level = 0;
                self.ball.reset();
                self.paddle.reset();
                self.load_level();
            }
        }
    }


    pub fn draw(&self, extend_texture: &Texture2D) {
        clear_background(BLACK);
        for brick in &self.bricks {
            brick.draw();
        }
        self.ball.draw();
        self.paddle.draw();
        for pu in &self.powerups {
            pu.draw(extend_texture);
        }

        let x = 15.0;
        let mut y = 30.0;
        draw_text(&format!("Score: {}", self.score), x, y, 30.0, WHITE);
        y += 30.0;
        draw_text(&format!("Lives: {}", self.lives), x, y, 30.0, WHITE);
        y += 30.0;
        draw_text(&format!("Level: {}", self.current_level + 1), x, y, 30.0, WHITE);
    }

}
