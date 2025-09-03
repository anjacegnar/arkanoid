use macroquad::prelude::*;
use crate::{world::World, constants::PADDLE_SPEED};

pub fn handle_input(world: &mut World) {
    let left  = is_key_down(KeyCode::Left)  || is_key_down(KeyCode::A);
    let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);

    world.paddle.vx = match (left, right) {
        (true, true) | (false, false) => 0.0,
        (true, false)  => -PADDLE_SPEED,
        (false, true)  =>  PADDLE_SPEED,
    };
}

