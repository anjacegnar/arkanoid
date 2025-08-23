use crate::{ball::Ball, paddle::Paddle, brick::Brick, utils::*, level::*};
use macroquad::prelude::*;

pub struct Game {
    pub balls: Vec<Ball>,
    pub paddle: Paddle,
    pub bricks: Vec<Brick>,
    pub powerups: Vec<PowerUp>,
    pub score: i32,
    pub lives: i32,
    pub extend_paddle_time_left: f32,
    pub current_level: usize,
    pub pu_rng: PowerUpRng,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            balls: vec![Ball::new()],
            paddle: Paddle::new(),
            bricks: Vec::new(),
            powerups: Vec::new(),
            score: 0,
            lives: 3,
            extend_paddle_time_left: 0.0,
            current_level: 0,
            pu_rng: PowerUpRng::new_default(),
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

        for ball in &mut self.balls {
            ball.update();
            let _ = ball.bounce_off_paddle(&self.paddle, self.paddle.vx);
            ball.collide_walls(screen_width());
        }

        let mut bricks_to_remove: Vec<usize> = Vec::new();
        let mut powerup_spawns: Vec<Vec2> = Vec::new();

        for (i, brick) in self.bricks.iter_mut().enumerate() {
            let mut hit = false;

            for ball in &mut self.balls {
                if ball.hit_brick(brick) {
                    hit = true;
                }
            }

            if hit {
                bricks_to_remove.push(i);
                self.score += 100;

                powerup_spawns.push(brick.center());
            }
        }

        bricks_to_remove.sort_unstable();
        bricks_to_remove.dedup();
        for idx in bricks_to_remove.into_iter().rev() {
            self.bricks.swap_remove(idx);
        }

        for pos in powerup_spawns {
            if let Some(pu) = self.pu_rng.maybe_spawn(pos) {
                self.powerups.push(pu);
            }
        }

        for pu in &mut self.powerups {
            pu.update(dt);

        }
        
        let mut taken_idxs: Vec<usize> = Vec::new();
        let mut effects: Vec<PowerUpType> = Vec::new();

        for (i, pu) in self.powerups.iter().enumerate() {
            if pu.active
                && pu.pos.y >= screen_height() - self.paddle.height
                && pu.pos.x >= self.paddle.x
                && pu.pos.x <= self.paddle.x + self.paddle.width
            {
                taken_idxs.push(i);
            }
        }

        taken_idxs.sort_unstable();
        taken_idxs.dedup();
        for i in taken_idxs.into_iter().rev() {
            let p = self.powerups.swap_remove(i);
            effects.push(p.kind);
        }

        for kind in effects {
            match kind {
                PowerUpType::ExtendPaddle => {
                    if self.extend_paddle_time_left <= 0.0 {
                        self.paddle.width *= 1.5;
                    }
                    self.extend_paddle_time_left = 10.0;
                }
                PowerUpType::ExtraLife => {
                    self.lives += 1;
                }
                PowerUpType::MultipleBalls => {
                    self.apply_multiple_balls(); 
                }
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

            for b in &mut self.balls {
                b.speed = (b.speed * 1.05).min(600.0);
            }

            self.load_level();
        }


        let h = screen_height();
        self.balls.retain(|b| b.pos.y - b.radius <= h);

        if self.balls.is_empty() {
            self.lives -= 1;
            if self.lives > 0 {
                self.balls.push(Ball::new());
                self.paddle.reset();
            } else {
                self.score = 0;
                self.lives = 3;
                self.current_level = 0;
                self.balls.clear();
                self.balls.push(Ball::new());
                self.paddle.reset();
                self.load_level();
            }
        }
    }

    fn apply_multiple_balls(&mut self) {
        if self.balls.len() >= MAX_BALLS { 
            return; 
        }

        let originals = self.balls.clone();

        let angle: f32 = 14.0_f32.to_radians();

        for b in &originals {
            if self.balls.len() + 2 > MAX_BALLS { 
                break; 
            }

            let mut b1 = b.clone();
            let mut b2 = b.clone();

            b1.vel = rotate(b.vel, -angle);
            b2.vel = rotate(b.vel,  angle);

            self.balls.push(b1);
            self.balls.push(b2);
        }
    }

    pub fn draw(&self, extend_texture: &Texture2D, life_texture: &Texture2D, balls_texture: &Texture2D) {
        clear_background(BLACK);

        for brick in &self.bricks {
            brick.draw();
        }

        for ball in &self.balls {
            ball.draw();
        }

        self.paddle.draw();

        for p in &self.powerups {
            let tex = match p.kind {
                PowerUpType::ExtendPaddle => &extend_texture,
                PowerUpType::ExtraLife => &life_texture,
                PowerUpType::MultipleBalls => &balls_texture,
            };
            p.draw(tex);
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
