use macroquad::prelude::*;
use crate::world::World;
use crate::assets::Assets;
use crate::model::{
    ball::Ball,
    paddle::Paddle,
    brick::{Brick, BrickKind},
    powerup::PowerUpType,
};

fn brick_color(kind: BrickKind) -> Color {
    match kind {
        BrickKind::Gray   => GRAY,
        BrickKind::Green  => GREEN,
        BrickKind::Red    => RED,
        BrickKind::Yellow => YELLOW,
    }
}

fn draw_brick(b: &Brick) {
    if !b.is_destroyed() {
        draw_rectangle(b.x, b.y, b.width, b.height, brick_color(b.kind));
    }
}

fn draw_paddle(p: &Paddle) {
    let y = screen_height() - p.height;
    draw_rectangle(p.x, y, p.width, p.height, BLUE);
}

fn draw_ball(b: &Ball) {
    draw_circle(b.pos.x, b.pos.y, b.radius, WHITE);
}

pub fn draw_with_assets(world: &World, assets: &Assets) {
    clear_background(BLACK);

    for b in &world.bricks {
        draw_brick(b);
    }
    draw_paddle(&world.paddle);

    for b in &world.balls {
        draw_ball(b);
    }

    for p in &world.powerups {
        let tex = match p.kind {
            PowerUpType::ExtendPaddle    => &assets.extend_paddle,
            PowerUpType::ExtraLife       => &assets.extra_life,
            PowerUpType::MultipleBalls   => &assets.multiple_balls,
            PowerUpType::SlowerBall { .. } => &assets.slower_ball,
        };
        let size = Vec2::new(42.0, 32.0);
        draw_texture_ex(
            &tex,
            p.pos.x - size.x * 0.5,
            p.pos.y - size.y * 0.5,
            WHITE,
            DrawTextureParams { dest_size: Some(size), ..Default::default() },
        );
    }

    draw_hud(world);
}

fn draw_hud(world: &World) {
    let mut y = 28.0;
    let x = 16.0;

    draw_text(&format!("Score: {}", world.score), x, y, 28.0, WHITE); y += 26.0;
    draw_text(&format!("Lives: {}", world.lives), x, y, 28.0, WHITE); y += 26.0;
    draw_text(&format!("Level: {}", world.current_level + 1), x, y, 28.0, WHITE); y += 26.0;

    if world.extend_time_left > 0.0 {
        y += 22.0;
        draw_text(&format!("Extend {:.1}s", world.extend_time_left.max(0.0)), x, y, 24.0, GREEN);
    }
    if world.slow_time_left > 0.0 {
        y += 22.0;
        draw_text(
            &format!("Slow x{:.2} ({:.1}s)", world.slow_factor, world.slow_time_left.max(0.0)),
            x, y, 24.0, SKYBLUE
        );
    }
}
