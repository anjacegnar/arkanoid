use macroquad::prelude::*;
use crate::{
    world::World,
    assets::Assets,
    systems::{input, physics, collisions, powerups, progression},
    render::world_draw,
};

pub async fn run(mut world: World, assets: &Assets) {
    loop {
        let dt = get_frame_time();

        input::handle_input(&mut world);
        physics::integrate(&mut world, dt);
        collisions::resolve_collisions(&mut world);
        powerups::update_powerups(&mut world, dt);
        powerups::collect_and_apply(&mut world);
        progression::tick_effects(&mut world, dt);

        world_draw::draw_with_assets(&world, assets);
        next_frame().await;
    }
}
