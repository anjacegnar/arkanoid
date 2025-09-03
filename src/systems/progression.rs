use crate::world::World;
use macroquad::prelude::*;

pub fn tick_effects(world: &mut World, dt: f32) {
    if world.extend_time_left > 0.0 {
        world.extend_time_left = (world.extend_time_left - dt).max(0.0);
        world.paddle.width = world.base_paddle_width * 1.4;
    } else {
        world.paddle.width = world.base_paddle_width;
    }

    if world.slow_time_left > 0.0 {
        world.slow_time_left = (world.slow_time_left - dt).max(0.0);
        if world.slow_time_left == 0.0 {
            world.slow_factor = 1.0;
        }
    }
}

pub fn life_lost(world: &mut World) {
    if world.lives > 0 {
        world.lives -= 1;
    }

    if world.lives == 0 {
        world.lives = 3;
        world.score = 0;
        world.current_level = 0;
        world.powerups.clear();
        world.extend_time_left = 0.0;
        world.slow_time_left = 0.0;
        world.slow_factor = 1.0;
        world.paddle.width = world.base_paddle_width;
        world.load_level();
    }

    world.reset_after_life_loss();
}
