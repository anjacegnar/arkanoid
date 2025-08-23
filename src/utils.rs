use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerUpType {
    ExtendPaddle,
    ExtraLife,
    MultipleBalls,
}

pub struct PowerUp {
    pub pos: Vec2,
    pub kind: PowerUpType,
    pub active: bool,
}

impl PowerUp {
    pub fn new(pos: Vec2, kind: PowerUpType) -> Self {
        Self { pos, kind, active: true }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.y += 100.0 * dt;
    }

    pub fn draw(&self, texture: &Texture2D) {
        let size = Vec2::new(42.0, 32.0);
        draw_texture_ex(
            texture,
            self.pos.x - size.x / 2.0,
            self.pos.y - size.y / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
}

pub const MAX_BALLS: usize = 42;

pub fn rotate(v: Vec2, ang: f32) -> Vec2 {
    let (s, c) = ang.sin_cos();
    vec2(v.x * c - v.y * s, v.x * s + v.y * c)
}

#[derive(Clone, Debug)]
pub struct PowerUpRng {
    pub drop_chance: f32,
    pub w_extra: f32,
    pub w_extend: f32,
    pub w_multi: f32,
    pub w_multi_min: f32,
    pub w_multi_decay: f32,
}

impl PowerUpRng {
    pub fn new_default() -> Self {
        Self {
            drop_chance: 0.07,
            w_extra: 0.20,
            w_extend: 0.30,
            w_multi: 0.50,
            w_multi_min: 0.10,
            w_multi_decay: 0.50,
        }
    }

    pub fn maybe_spawn(&mut self, pos: Vec2) -> Option<PowerUp> {
        if macroquad::rand::gen_range(0.0, 1.0) >= self.drop_chance {
            return None;
        }

        let total = self.w_extra + self.w_extend + self.w_multi;
        let r = macroquad::rand::gen_range(0.0, total);

        let kind = if r < self.w_extra {
            PowerUpType::ExtraLife
        } else if r < self.w_extra + self.w_extend {
            PowerUpType::ExtendPaddle
        } else {
            PowerUpType::MultipleBalls
        };

        if let PowerUpType::MultipleBalls = kind {
            self.w_multi = (self.w_multi * self.w_multi_decay).max(self.w_multi_min);
        }

        Some(PowerUp::new(pos, kind))
    }
}