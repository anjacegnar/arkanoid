use macroquad::prelude::*;
use crate::world::World;

pub fn integrate(world: &mut World, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    world.paddle.x += world.paddle.vx * dt;
    if world.paddle.x < 0.0 {
        world.paddle.x = 0.0;
    }
    let max_x = screen_width() - world.paddle.width;
    if world.paddle.x > max_x {
        world.paddle.x = max_x;
    }

    let speed_scale = if world.slow_time_left > 0.0 {
        world.slow_factor.max(0.1)
    } else {
        1.0
    };

    for b in &mut world.balls {
        b.step(dt, speed_scale);
    }
}
