mod game;
mod ball;
mod paddle;
mod brick;
mod utils;
mod level;

use macroquad::prelude::*;
use game::Game;

#[macroquad::main("Arkanoid")]
async fn main() {
    let extend_texture = load_texture("assets/extend_paddle.png")
        .await
        .unwrap();

    let mut game = Game::new();

    loop {
        let dt = get_frame_time();
        game.update(dt);

        clear_background(BLACK);
        game.draw(&extend_texture);
        next_frame().await;
    }
}