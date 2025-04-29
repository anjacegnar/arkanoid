use macroquad::prelude::*;
use crate::paddle::Paddle;
use crate::brick::Brick;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
            radius: 7.0,
        }
    }

    // Posodobi pozicijo glede na hitrost in čas
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    pub fn collide_walls(&mut self, screen_w: f32, screen_h: f32) {
        // Odbije žogico od sten
    }

    pub fn collide_paddle(&self, paddle: &Paddle) -> bool {
        false
        // Preveri trk s ploščico
    }

    pub fn collide_brick(&self, brick: &Brick) -> bool {
        false
        // Preveri trk z enim blokom
    }

    // Obrne vertikalno komponento hitrosti
    pub fn bounce_y(&mut self) {
        self.vel.y = -self.vel.y;
    }

    // Nariše žogico
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
