use arkanoid::ball::Ball;
use macroquad::prelude::Vec2;

#[test]
fn ball_moves_correctly() {
    let mut ball = Ball {
        pos: Vec2::new(0.0, 0.0),
        vel: Vec2::new(1.0, 1.0),
        radius: 10.0,
        speed: 1.0,
    };

    let dt = 1.0;
    ball.pos += ball.vel * ball.speed * dt;

    let eps = 1e-6;
    assert!((ball.pos.x - 1.0).abs() < eps);
    assert!((ball.pos.y - 1.0).abs() < eps);
    assert!((ball.vel.x - 1.0).abs() < eps && (ball.vel.y - 1.0).abs() < eps);
}
