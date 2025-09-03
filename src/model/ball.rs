use macroquad::prelude::Vec2;

#[derive(Clone, Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub speed: f32,
}

impl Ball {
    pub fn new_at(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::new(1.0, -1.0).normalize(),
            radius: 8.0,
            speed: 400.0,
        }
    }

    pub fn with_dir(mut self, dir: Vec2) -> Self {
        if dir.length_squared() > 0.0 {
            self.vel = dir.normalize();
        }
        self
    }

    pub fn step(&mut self, dt: f32, speed_scale: f32) {
        self.pos += self.vel * (self.speed * speed_scale * dt);
    }

    pub fn bounce_x(&mut self) {
        self.vel.x = -self.vel.x;
        self.vel = self.vel.normalize();
    }

    pub fn bounce_y(&mut self) {
        self.vel.y = -self.vel.y;
        self.vel = self.vel.normalize();
    }
}