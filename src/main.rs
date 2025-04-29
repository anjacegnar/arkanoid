mod game;
mod ball;
mod paddle;
mod brick;
mod utils;

fn main() {
    println!("Arkanoid game initialized.");
    let mut game = game::Game::new();
    game.run();
}