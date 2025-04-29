pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub destroyed: bool,
}

impl Brick {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, destroyed: false }
    }

    pub fn layout() -> Vec<Brick> {
        let mut bricks = Vec::new();
        for i in 0..5 {
            for j in 0..10 {
                bricks.push(Brick::new(j as f32 * 10.0, i as f32 * 5.0));
            }
        }
        bricks
    }
}
