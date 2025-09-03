use arkanoid::{app, world::World, assets::Assets};

#[macroquad::main("Arkanoid")]
async fn main() {
    let assets = Assets::load().await;

    let world = World::new();

    app::run(world, &assets).await;
}