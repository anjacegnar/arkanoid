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

    // Posodobi položaj
    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.x -= self.speed * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.x += self.speed * dt;
        }

        // omeji ploščico znotraj vidnega polja
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.width > screen_width() {
            self.x = screen_width() - self.width;
        }
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
