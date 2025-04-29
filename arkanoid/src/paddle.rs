pub struct Paddle {
    pub x: f32,
    pub width: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self { x: 0.0, width: 25.0 }
    }

    pub fn move_left(&mut self) {
        self.x -= 1.0;
    }

    pub fn move_right(&mut self) {
        self.x += 1.0;
    }
}