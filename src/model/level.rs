use crate::model::brick::{Brick, BrickKind};
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
use crate::constants::{BRICK_W, STEP_X, STEP_Y};

pub struct Level {
    pub brick_positions: Vec<(f32, f32)>,
}

impl Level {
    pub fn center_positions_horiz(mut pts: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
        if pts.is_empty() {
            return pts;
        }
        let min_x = pts.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
        let max_x = pts.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
        let cluster_w = (max_x - min_x) + BRICK_W;
        let desired_left = (screen_width() - cluster_w) * 0.5;
        let dx = desired_left - min_x;
        for p in pts.iter_mut() {
            p.0 += dx;
        }
        pts
    }

    pub fn heart_positions_centered() -> Vec<(f32, f32)> {
        Self::center_positions_horiz(Self::heart_positions())
    }

    /// Srček
    pub fn heart_positions() -> Vec<(f32, f32)> {
        let base_x = 35.0;
        let base_y = 40.0;
        vec![
            // 1. vrstica
            (base_x + 1.0 * STEP_X, base_y + 0.0 * STEP_Y),
            (base_x + 2.0 * STEP_X, base_y + 0.0 * STEP_Y),
            (base_x + 3.0 * STEP_X, base_y + 0.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 0.0 * STEP_Y),
            (base_x + 7.0 * STEP_X, base_y + 0.0 * STEP_Y),
            (base_x + 8.0 * STEP_X, base_y + 0.0 * STEP_Y),

            // 2. vrstica
            (base_x + 0.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 1.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 2.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 3.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 4.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 7.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 8.0 * STEP_X, base_y + 1.0 * STEP_Y),
            (base_x + 9.0 * STEP_X, base_y + 1.0 * STEP_Y),

            // 3. vrstica
            (base_x + 0.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 1.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 2.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 3.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 4.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 7.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 8.0 * STEP_X, base_y + 2.0 * STEP_Y),
            (base_x + 9.0 * STEP_X, base_y + 2.0 * STEP_Y),

            // 4. vrstica
            (base_x + 1.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 2.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 3.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 4.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 7.0 * STEP_X, base_y + 3.0 * STEP_Y),
            (base_x + 8.0 * STEP_X, base_y + 3.0 * STEP_Y),

            // 5. vrstica
            (base_x + 2.0 * STEP_X, base_y + 4.0 * STEP_Y),
            (base_x + 3.0 * STEP_X, base_y + 4.0 * STEP_Y),
            (base_x + 4.0 * STEP_X, base_y + 4.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 4.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 4.0 * STEP_Y),
            (base_x + 7.0 * STEP_X, base_y + 4.0 * STEP_Y),

            // 6. vrstica
            (base_x + 3.0 * STEP_X, base_y + 5.0 * STEP_Y),
            (base_x + 4.0 * STEP_X, base_y + 5.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 5.0 * STEP_Y),
            (base_x + 6.0 * STEP_X, base_y + 5.0 * STEP_Y),

            // 7. vrstica
            (base_x + 4.0 * STEP_X, base_y + 6.0 * STEP_Y),
            (base_x + 5.0 * STEP_X, base_y + 6.0 * STEP_Y),
        ]
    }

    pub fn spawn_bricks(&self) -> Vec<Brick> {
        let mut v = Vec::with_capacity(self.brick_positions.len());
        for (x, y) in &self.brick_positions {
            v.push(Brick::new_default(*x, *y));
        }
        v
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RandomLevelCfg {
    pub side_padding: f32,
    pub top_padding: f32,
    pub min_rows: usize,
    pub max_rows: usize,
    pub min_cols: usize,
    pub max_cols: usize,
    pub hole_pct: f32, // verjetnost "luknje" v mreži
    pub row_offset: f32,
}

impl Default for RandomLevelCfg {
    fn default() -> Self {
        Self {
            side_padding: 40.0,
            top_padding: 40.0,
            min_rows: 5,
            max_rows: 12,
            min_cols: 8,
            max_cols: 10,
            hole_pct: 0.12,
            row_offset: 32.0,
        }
    }
}

/// Ustvari naključna mesta brickov
pub fn random_positions(seed: Option<u64>, cfg: RandomLevelCfg) -> Vec<(f32, f32)> {
    if let Some(s) = seed {
        srand(s);
    } else { 
        srand((get_time() * 1_000_000.0) as u64);
    }

    let step_x = STEP_X;
    let step_y = STEP_Y;

    let rows = gen_range(cfg.min_rows as i32, (cfg.max_rows + 1) as i32) as usize;
    let cols = gen_range(cfg.min_cols as i32, (cfg.max_cols + 1) as i32) as usize;

    let mut out = Vec::with_capacity(rows * cols);

    for r in 0..rows {
        let offset = if r % 2 == 0 { 0.0 } else { cfg.row_offset };
        for c in 0..cols {
            if gen_range(0.0, 1.0) < cfg.hole_pct {
                continue;
            }

            let x = cfg.side_padding + offset + c as f32 * step_x;
            let y = cfg.top_padding + r as f32 * step_y;

            if x + step_x > screen_width() - cfg.side_padding {
                continue;
            }

            out.push((x, y));
        }
    }
    Level::center_positions_horiz(out)
}

pub fn assign_random_brick_kinds(bricks: &mut [Brick]) {
    let p_red: f32 = 0.60;
    let p_green: f32 = 0.25;
    let p_yellow: f32 = 0.10;
    let p_gray: f32 = 0.05;

    let total = p_gray + p_green + p_red + p_yellow;

    for b in bricks.iter_mut() {
        let r = gen_range(0.0f32, total);
        let kind = if r < p_gray {
            BrickKind::Gray
        } else if r < p_gray + p_green {
            BrickKind::Green
        } else if r < p_gray + p_green + p_red {
            BrickKind::Red
        } else {
            BrickKind::Yellow
        };
        b.set_kind(kind);
    }

    if !bricks.iter().any(|b| b.is_breakable()) {
        if let Some(b0) = bricks.get_mut(0) {
            b0.set_kind(BrickKind::Red);
        }
    }
}
