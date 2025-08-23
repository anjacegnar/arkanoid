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

    extend_texture.set_filter(FilterMode::Nearest);

    let life_texture = load_texture("assets/extra_life.png")
    .await
    .unwrap();

    life_texture.set_filter(FilterMode::Nearest);

    let balls_texture = load_texture("assets/multiple_balls.png")
    .await
    .unwrap();

    balls_texture.set_filter(FilterMode::Nearest);

    let mut game = Game::new();

    loop {
        game.update();

        clear_background(BLACK);
        game.draw(&extend_texture, &life_texture, &balls_texture);
        next_frame().await;
    }
}