use macroquad::prelude::*;

pub struct Paddle {
    pub x: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub vx: f32,
}

impl Paddle {
    pub fn new() -> Self {
        let width = 100.0;
        let height = 20.0;
        Self {
            x: screen_width() / 2.0 - width / 2.0,
            width,
            height,
            speed: 500.0,
            vx: 0.0,
        }
    }

    // Posodobi položaj
    pub fn update(&mut self) {
        let dt = get_frame_time();
        let prev_x = self.x; 

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
        let max_x = screen_width() - self.width;
        if self.x > max_x {
            self.x = max_x;
        }

        self.vx = (self.x - prev_x) / dt;
    }

    // Nariše ploščico
    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            screen_height() - self.height,
            self.width,
            self.height,
            BLUE);
    }

    pub fn reset(&mut self) {
        self.x = screen_width() / 2.0 - self.width / 2.0;
        self.vx = 0.0;
    }
}
