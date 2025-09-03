use macroquad::prelude::Vec2;
use crate::constants::{BRICK_W, BRICK_H};

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
            BrickKind::Red    => 1,
            BrickKind::Green  => 3,
            BrickKind::Yellow => 5,
            BrickKind::Gray   => i32::MAX,
        }
    }
    pub fn is_unbreakable(self) -> bool {
        matches!(self, BrickKind::Gray)
    }
}

#[derive(Clone, Debug)]
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
    pub fn new(x: f32, y: f32, width: f32, height: f32, kind: BrickKind) -> Self {
        Self {
            x, 
            y, 
            width, 
            height,
            destroyed: false,
            kind,
            hp: kind.hit_points(),
        }
    }

    pub fn new_default(x: f32, y: f32) -> Self {
        Self::new(x, y, BRICK_W, BRICK_H, BrickKind::Red)
    }

    pub fn set_kind(&mut self, kind: BrickKind) {
        self.kind = kind;
        self.hp = kind.hit_points();
        self.destroyed = false;
    }

    pub fn on_hit(&mut self) -> bool {
        if !self.kind.is_unbreakable() && !self.destroyed {
            if self.hp > 0 { 
                self.hp -= 1; 
            }
            if self.hp <= 0 {
                self.destroyed = true;
                return true;
            }
        }
        false
    }

    pub fn is_breakable(&self) -> bool { 
        !self.kind.is_unbreakable() 
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed || (self.is_breakable() && self.hp <= 0)
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2.0, 
            self.y + self.height / 2.0
        )
    }

    pub fn aabb(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
