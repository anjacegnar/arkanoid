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

    // Odbije žogico od ploščice
    pub fn bounce_off_paddle(&mut self, paddle: &Paddle, paddle_vx: f32) -> bool {
        let paddle_y = screen_height() - paddle.height;

        let intersects_x = self.pos.x + self.radius >= paddle.x
            && self.pos.x - self.radius <= paddle.x + paddle.width;
        let intersects_y = self.pos.y + self.radius >= paddle_y
            && self.pos.y - self.radius <= paddle_y + paddle.height;

        // obravnavamo samo, če žoga potuje navzdol
        if !(intersects_x && intersects_y && self.vel.y > 0.0) {
            return false;
        }

        // relativni zadetek po širini ploščice v [-1, 1]
        let paddle_cx = paddle.x + paddle.width * 0.5;
        let hit_offset = ((self.pos.x - paddle_cx) / (paddle.width * 0.5)).clamp(-1.0, 1.0);

        // največji odklon od navpičnice
        let max_angle_from_vertical = 60f32.to_radians();
        let theta = hit_offset * max_angle_from_vertical;

        let base_speed = self.vel.length().max(120.0);

        // nova smer, odvisna od kota
        let mut vx =  base_speed * theta.sin();
        let mut vy = -base_speed * theta.cos();

        // vpliv hitrosti ploščice
        let paddle_influence = 0.25;
        vx += paddle_vx * paddle_influence;

        let mut v = vec2(vx, vy);
        let len = v.length();
        if len > 0.0 {
            v *= base_speed / len;
        }

        let min_up_frac = 0.35;
        let min_up_v = base_speed * min_up_frac;
        if v.y > -min_up_v {
            v.y = -min_up_v;
        }

        self.vel = v;
        self.pos.y = paddle_y - self.radius - 0.1;
        true
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
