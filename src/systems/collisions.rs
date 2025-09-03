use macroquad::prelude::*;
use crate::world::World;
use crate::constants::SIDE_PADDING;
use crate::model::{ball::Ball, paddle::Paddle, brick::Brick};
use crate::systems::{progression, powerups};

pub fn resolve_collisions(world: &mut World) {
    let sw = screen_width();
    let sh = screen_height();

    let mut scored = 0;
    let mut spawn_at: Vec<Vec2> = Vec::new();

    for ball in &mut world.balls {
        collide_walls(ball, sw, sh, SIDE_PADDING);
        let _ = bounce_off_paddle(ball, &world.paddle, world.paddle.vx);

        for brick in world.bricks.iter_mut() {
            if hit_brick(ball, brick) {
                let was_destroyed = brick.on_hit();
                if brick.is_breakable() {
                    if was_destroyed {
                        scored += 100;
                        spawn_at.push(brick.center());
                    } else {
                        scored += 10;
                    }
                }
            }
        }
    }
    world.score += scored;

    // odstrani uničene bricke
    world.bricks.retain(|b| !b.is_destroyed());

    for pos in spawn_at {
        powerups::maybe_spawn_powerup(world, pos);
    }

    // odstrani žoge, ki so padle
    world.balls.retain(|b| b.pos.y - b.radius <= sh);

    let any_breakable_left = world
        .bricks
        .iter()
        .any(|b| b.is_breakable() && !b.is_destroyed());

    if !any_breakable_left {
        world.advance_level();
        return;
    }

    if world.balls.is_empty() {
        progression::life_lost(world);
    }
}

fn collide_walls(ball: &mut Ball, screen_w: f32, _screen_h: f32, side_pad: f32) {
    if ball.pos.x - ball.radius <= side_pad {
        ball.pos.x = side_pad + ball.radius + 0.1;
        ball.bounce_x();
    }
    if ball.pos.x + ball.radius >= screen_w - side_pad {
        ball.pos.x = screen_w - side_pad - ball.radius - 0.1;
        ball.bounce_x();
    }
    if ball.pos.y - ball.radius <= 0.0 {
        ball.pos.y = ball.radius + 0.1;
        ball.bounce_y();
    }
}

fn bounce_off_paddle(ball: &mut Ball, paddle: &Paddle, paddle_vx: f32) -> bool {
    let paddle_y = screen_height() - paddle.height;

    let intersects_x = ball.pos.x + ball.radius >= paddle.x
        && ball.pos.x - ball.radius <= paddle.x + paddle.width;
    let intersects_y = ball.pos.y + ball.radius >= paddle_y
        && ball.pos.y - ball.radius <= paddle_y + paddle.height;

    if !(intersects_x && intersects_y && ball.vel.y > 0.0) {
        return false;
    }

    let paddle_cx = paddle.x + paddle.width * 0.5;
    let hit_offset = ((ball.pos.x - paddle_cx) / (paddle.width * 0.5)).clamp(-1.0, 1.0);

    let max_angle_from_vertical = 60f32.to_radians();
    let theta = hit_offset * max_angle_from_vertical;

    let mut dir = Vec2::new(theta.sin(), -theta.cos());

    let paddle_influence = 0.25;
    if ball.speed > 1.0 {
        dir.x += (paddle_vx / ball.speed) * paddle_influence;
    } else {
        dir.x += paddle_vx * 0.002;
    }

    let min_up_frac = 0.25;
    if dir.y > -min_up_frac {
        dir.y = -min_up_frac;
    }

    let len = dir.length();
    ball.vel = if len > 0.0 { dir / len } else { Vec2::new(0.0, -1.0) };

    ball.pos.y = paddle_y - ball.radius - 0.1;
    true
}

fn collide_brick(ball: &Ball, brick: &Brick) -> bool {
    ball.pos.y - ball.radius <= brick.y + brick.height
        && ball.pos.y + ball.radius >= brick.y
        && ball.pos.x + ball.radius >= brick.x
        && ball.pos.x - ball.radius <= brick.x + brick.width
}

fn hit_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
    if brick.is_destroyed() {
        return false;
    }
    if !collide_brick(ball, brick) {
        return false;
    }

    let overlap_left = (ball.pos.x + ball.radius) - brick.x;
    let overlap_right = (brick.x + brick.width) - (ball.pos.x - ball.radius);
    let overlap_top = (ball.pos.y + ball.radius) - brick.y;
    let overlap_bottom = (brick.y + brick.height) - (ball.pos.y - ball.radius);

    let min_horiz = overlap_left.min(overlap_right);
    let min_vert  = overlap_top.min(overlap_bottom);

    if min_horiz < min_vert {
        if overlap_left < overlap_right {
            ball.pos.x = brick.x - ball.radius - 0.1;
        } else {
            ball.pos.x = brick.x + brick.width + ball.radius + 0.1;
        }
        ball.bounce_x();
    } else {
        if overlap_top < overlap_bottom {
            ball.pos.y = brick.y - ball.radius - 0.1;
        } else {
            ball.pos.y = brick.y + brick.height + ball.radius + 0.1;
        }
        ball.bounce_y();
    }
    true
}

