use macroquad::prelude::*;

pub struct Paddle {
    pub x: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, height: f32, width: f32) -> Self {
        Self {
            x,
            width,
            height,
            speed: 500.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
    }

    // Nariše ploščico
    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            screen_height() - self.height,
            self.width,
            self.height,
            BLUE,
        );
    }
}
