mod game;
mod ball;
mod paddle;
mod brick;
mod utils;
mod level;

use macroquad::prelude::*;
use game::Game;
use crate::utils::*;

#[macroquad::main("Arkanoid")]
async fn main() {
    let extend_texture = load_texture("assets/extend_paddle.png")
    .await
    .unwrap();

    extend_texture.set_filter(FilterMode::Nearest);

    let life_texture = load_texture("assets/extra_life.png")
    .await
    .unwrap();

    life_texture.set_filter(FilterMode::Nearest);

    let mut game = Game::new();

    loop {
        game.update();

        clear_background(BLACK);
        game.draw(&extend_texture);
        for p in &game.powerups {
            let tex = match p.kind {
                PowerUpType::ExtendPaddle => &extend_texture,
                PowerUpType::ExtraLife    => &life_texture,
            };
            p.draw(tex);
        }
        next_frame().await;
    }
}