use macroquad::prelude::*;

pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub destroyed: bool,
}

impl Brick {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 60.0,
            height: 20.0,
            destroyed: false,
        }
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }

    // Postavi mrežo 5×10 blokov
    pub fn layout() -> Vec<Brick> {
        let mut bricks = Vec::new();
        for row in 0..5 {
            for col in 0..10 {
                let x = col as f32 * (60.0 + 5.0) + 35.0;
                let y = row as f32 * (20.0 + 5.0) + 40.0;
                bricks.push(Brick::new(x, y));
            }
        }
        bricks
    }

    // Nariše blok, če še ni uničen
    pub fn draw(&self) {
        if !self.destroyed {
            draw_rectangle(self.x, self.y, self.width, self.height, GRAY);
        }
    }
}
