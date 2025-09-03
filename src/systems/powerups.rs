use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::world::World;
use crate::model::powerup::{PowerUp, PowerUpType};
use crate::utils::{rotate, MAX_BALLS};

pub fn maybe_spawn_powerup(world: &mut World, spawn_pos: Vec2) {
    let r = gen_range(0.0, 1.0);
    if r < 0.07 {
        let kind = if r < 0.01 {
            PowerUpType::ExtraLife
        } else if r < 0.04 {
            PowerUpType::ExtendPaddle
        } else if r < 0.06 {
            PowerUpType::MultipleBalls
        } else {
            PowerUpType::SlowerBall { factor: 0.6, duration: 10.0 }
        };
        world.powerups.push(PowerUp::new(spawn_pos, kind));
    }
}

pub fn update_powerups(world: &mut World, dt: f32) {
    for p in &mut world.powerups {
        p.pos += p.vel * dt;
    }
    let h = screen_height();
    world.powerups.retain(|p| p.pos.y - p.radius <= h);
}

pub fn collect_and_apply(world: &mut World) {
    let mut to_apply: Vec<PowerUpType> = Vec::new();

    world.powerups.retain(|p| {
        if overlaps_with_paddle(p.pos, p.radius, &world.paddle) {
            to_apply.push(p.kind);
            false
        } else {
            true
        }
    });

    for kind in to_apply {
        apply_powerup(world, kind);
    }
}

fn apply_powerup(world: &mut World, kind: PowerUpType) {
    match kind {
        PowerUpType::ExtraLife => {
            world.lives += 1;
        }
        PowerUpType::ExtendPaddle => {
            world.extend_time_left = world.extend_time_left.max(10.0);
            world.paddle.width = world.base_paddle_width * 1.4;
        }
        PowerUpType::MultipleBalls => {
            if world.balls.len() < MAX_BALLS {
                let mut to_add = Vec::new();
                let mut remaining = MAX_BALLS.saturating_sub(world.balls.len());
                if remaining == 0 {
                    return;
                }

                for b in &world.balls {
                    if remaining == 0 {
                        break;
                    }

                    let base_deg = gen_range(10.0_f32, 18.0_f32);
                    let angles = [ -base_deg.to_radians(), base_deg.to_radians() ];

                    for &ang in &angles {
                        if remaining == 0 {
                            break;
                        }
                        let mut nb = b.clone();
                        let nv = rotate(nb.vel, ang);
                        nb.vel = if nv.length_squared() > 0.0 { nv.normalize() } else { nb.vel };
                        to_add.push(nb);
                        remaining -= 1;
                    }
                }
                world.balls.extend(to_add);
            }
        }
        PowerUpType::SlowerBall { factor, duration } => {
            world.slow_factor = factor;
            world.slow_time_left = duration;
        }
    }
}

fn overlaps_with_paddle(center: Vec2, radius: f32, paddle: &crate::model::paddle::Paddle) -> bool {
    let y = screen_height() - paddle.height;
    let (px, py, pw, ph) = (paddle.x, y, paddle.width, paddle.height);

    let cx = center.x.clamp(px, px + pw);
    let cy = center.y.clamp(py, py + ph);

    let dx = center.x - cx;
    let dy = center.y - cy;

    dx * dx + dy * dy <= radius * radius
}
