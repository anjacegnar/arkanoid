use macroquad::prelude::*;

pub enum PowerUpType {
    ExtendPaddle,
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
