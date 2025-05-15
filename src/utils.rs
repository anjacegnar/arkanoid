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

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 8.0, YELLOW);
    }
}
