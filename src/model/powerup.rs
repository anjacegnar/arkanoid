use macroquad::prelude::Vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PowerUpType {
    ExtendPaddle,
    ExtraLife,
    MultipleBalls,
    SlowerBall { factor: f32, duration: f32 },
}

#[derive(Clone, Debug)]
pub struct PowerUp {
    pub pos: Vec2,
    pub vel: Vec2,
    pub kind: PowerUpType,
    pub radius: f32,
}

impl PowerUp {
    pub fn new(pos: Vec2, kind: PowerUpType) -> Self {
        Self {
            pos,
            vel: Vec2::new(0.0, 120.0),
            kind,
            radius: 8.0,
        }
    }
}