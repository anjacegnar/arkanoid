mod game;
mod ball;
mod paddle;
mod brick;
mod utils;

use macroquad::prelude::*;
use game::Game;

#[macroquad::main("Arkanoid")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();
        game.update(dt);

        clear_background(BLACK);
        game.draw();
        next_frame().await;
    }
}