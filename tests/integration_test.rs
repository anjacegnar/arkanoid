use arkanoid::ball::Ball;
use macroquad::prelude::Vec2;

#[test]
fn ball_moves_correctly() {
    let start = Vec2::new(0.0, 0.0);
    let speed = Vec2::new(1.0, 1.0);

    let mut ball = Ball::new(start, speed);

    ball.update(1.0);

    assert_eq!(ball.pos.x, 1.0);
    assert_eq!(ball.pos.y, 1.0);

    assert_eq!(ball.vel, speed);
}
