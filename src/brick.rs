use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BrickKind {
    Red,
    Green,
    Yellow,
    Gray,
}

impl BrickKind {
    pub fn hit_points(self) -> i32 {
        match self {
            BrickKind::Red   => 1,
            BrickKind::Green  => 3,
            BrickKind::Yellow    => 5,
            BrickKind::Gray => i32::MAX,
        }
    }
    pub fn color(self) -> Color {
        match self {
            BrickKind::Gray   => GRAY,
            BrickKind::Green  => GREEN,
            BrickKind::Red    => RED,
            BrickKind::Yellow => YELLOW,
        }
    }
    pub fn is_unbreakable(self) -> bool {
        matches!(self, BrickKind::Gray)
    }
}
pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub destroyed: bool,
    pub kind: BrickKind,
    pub hp: i32,
}

impl Brick {
    pub fn new(x: f32, y: f32) -> Self {
        let kind = BrickKind::Red;
        Self {
            x,
            y,
            width: 60.0,
            height: 20.0,
            destroyed: false,
            kind,
            hp: kind.hit_points(),
        }
    }

    pub fn set_kind(&mut self, kind: BrickKind) {
        self.kind = kind;
        self.hp = kind.hit_points();
        self.destroyed = false;
    }

    pub fn on_hit(&mut self) {
        if !self.kind.is_unbreakable() && self.hp > 0 {
            self.hp -= 1;
            if self.hp <= 0 {
                self.destroyed = true;
            }
        }
    }

    pub fn is_breakable(&self) -> bool {
        !self.kind.is_unbreakable()
    }

    pub fn is_destroyed(&self) -> bool {
        self.is_breakable() && (self.hp <= 0 || self.destroyed)
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }

    // Nariše blok, če še ni uničen
    pub fn draw(&self) {
        if !self.destroyed {
            draw_rectangle(self.x, self.y, self.width, self.height, self.kind.color());
        }
    }
}
