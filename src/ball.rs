use macroquad::prelude::*;
use crate::paddle::Paddle;
use crate::brick::Brick;

#[derive(Clone)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            vel: vec2(1.0, -1.0).normalize(),
            radius: 8.0,
            speed: 300.0,
        }
    }

    // Posodobi pozicijo glede na hitrost in čas
    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.pos += self.vel * self.speed * dt;
    }

    // Odbije žogico od sten
    pub fn collide_walls(&mut self, screen_w: f32) {
        // leva
        if self.pos.x - self.radius <= 0.0 {
            self.pos.x = self.radius + 0.1;
            self.vel.x = -self.vel.x;
        }
        // desna
        if self.pos.x + self.radius >= screen_w {
            self.pos.x = screen_w - self.radius - 0.1;
            self.vel.x = -self.vel.x;
        }
        // zgornja
        if self.pos.y - self.radius <= 0.0 {
            self.pos.y = self.radius + 0.1;
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

        // nova smer, odvisna od kota
        let mut dir = vec2(theta.sin(), -theta.cos());

        // vpliv hitrosti ploščice
        let paddle_influence = 0.25;
        if self.speed > 1.0 {
            dir.x += (paddle_vx / self.speed) * paddle_influence;
        } else {
            dir.x += paddle_vx * 0.002;
        }

        // prepreči, da bi se žogica odbila navzdol
        let min_up_frac = 0.25;
        if dir.y > -min_up_frac {
            dir.y = -min_up_frac;
        }

        let len = dir.length();
        if len > 0.0 {
            dir /= len;
        } else {
            dir = vec2(0.0, -1.0);
        }

        self.vel = dir;

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

    // Obrne horizontalno komponento hitrosti
    pub fn bounce_x(&mut self) {
        self.vel.x = -self.vel.x;
        self.vel = self.vel.normalize();
    }

    // Obrne vertikalno komponento hitrosti
    pub fn bounce_y(&mut self) {
        self.vel.y = -self.vel.y;
        self.vel = self.vel.normalize();
    }

    /// Trki z bricki
    pub fn hit_brick(&mut self, brick: &mut Brick) -> bool {
        if brick.destroyed {
            return false;
        }
        if !self.collide_brick(brick) {
            return false;
        }

        let overlap_left   = (self.pos.x + self.radius) - brick.x;
        let overlap_right  = (brick.x + brick.width) - (self.pos.x - self.radius);
        let overlap_top    = (self.pos.y + self.radius) - brick.y;
        let overlap_bottom = (brick.y + brick.height) - (self.pos.y - self.radius);

        let min_horiz = overlap_left.min(overlap_right);
        let min_vert  = overlap_top.min(overlap_bottom);

        if min_horiz < min_vert {
            if overlap_left < overlap_right {
                self.pos.x = brick.x - self.radius - 0.1;
            } else {
                self.pos.x = brick.x + brick.width + self.radius + 0.1;
            }
            self.bounce_x();
        } else {
            if overlap_top < overlap_bottom {
                self.pos.y = brick.y - self.radius - 0.1;
            } else {
                self.pos.y = brick.y + brick.height + self.radius + 0.1;
            }
            self.bounce_y();
        }

        true
    }

    // Nariše žogico
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
