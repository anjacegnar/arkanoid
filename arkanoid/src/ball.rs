pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, dx: 1.0, dy: 1.0 }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}