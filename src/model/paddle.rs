#[derive(Clone, Debug)]
pub struct Paddle {
    pub x: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub vx: f32,
}

impl Paddle {
    pub fn new_centered(screen_width: f32) -> Self {
        let width = 100.0;
        let height = 20.0;
        Self {
            x: screen_width / 2.0 - width / 2.0,
            width,
            height,
            speed: 700.0,
            vx: 0.0,
        }
    }

    pub fn clamp_to(&mut self, window_w: f32) {
        let max_x = window_w - self.width;
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x > max_x {
            self.x = max_x;
        }
    }

    pub fn aabb_at(&self, y: f32) -> (f32, f32, f32, f32) {
        (self.x, y, self.width, self.height)
    }
}

