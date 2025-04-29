use arkanoid::ball::Ball;

#[test]
fn ball_moves_correctly() {
    let mut ball = Ball::new();
    ball.update();
    assert_eq!(ball.x, 1.0);
    assert_eq!(ball.y, 1.0);
}
