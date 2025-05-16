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

    // Odbije žogico od sten
    pub fn collide_walls(&mut self, screen_w: f32, screen_h: f32) {
        // leva in desna
        if self.pos.x - self.radius <= 0.0 || self.pos.x + self.radius >= screen_w {
            self.vel.x = -self.vel.x;
        }
        // zgornja
        if self.pos.y - self.radius <= 0.0 {
            self.vel.y = -self.vel.y;
        }
    }

    // Preveri trk s ploščico
    pub fn collide_paddle(&self, paddle: &Paddle) -> bool {
        let paddle_y = screen_height() - paddle.height;
        let within_x = self.pos.x + self.radius >= paddle.x
            && self.pos.x - self.radius <= paddle.x + paddle.width;
        let within_y = self.pos.y + self.radius >= paddle_y
            && self.pos.y - self.radius <= paddle_y + paddle.height;
        within_x && within_y
    }

    // Preveri trk z enim blokom
    pub fn collide_brick(&self, brick: &Brick) -> bool {
        self.pos.y - self.radius <= brick.y + brick.height
            && self.pos.y + self.radius >= brick.y
            && self.pos.x + self.radius >= brick.x
            && self.pos.x - self.radius <= brick.x + brick.width
    }

    // Obrne vertikalno komponento hitrosti
    pub fn bounce_y(&mut self) {
        self.vel.y = -self.vel.y;
    }

    // Obrne horizontalno komponento hitrosti
    pub fn bounce_x(&mut self) {
        self.vel.x = -self.vel.x;
    }

    // Nariše žogico
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
