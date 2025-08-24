use crate::brick::*;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
pub struct Level {
    pub brick_positions: Vec<(f32, f32)>,
}

impl Level {
    /// Srček
    pub fn heart_positions() -> Vec<(f32, f32)> {
        vec![
            // 1. vrstica
            (35.0 + 1.0 * 65.0, 40.0), (35.0 + 2.0 * 65.0, 40.0), (35.0 + 3.0 * 65.0, 40.0),
            (35.0 + 6.0 * 65.0, 40.0), (35.0 + 7.0 * 65.0, 40.0), (35.0 + 8.0 * 65.0, 40.0),
            // 2. vrstica
            (35.0 + 0.0 * 65.0, 65.0), (35.0 + 1.0 * 65.0, 65.0), (35.0 + 2.0 * 65.0, 65.0),
            (35.0 + 3.0 * 65.0, 65.0), (35.0 + 4.0 * 65.0, 65.0), (35.0 + 5.0 * 65.0, 65.0),
            (35.0 + 6.0 * 65.0, 65.0), (35.0 + 7.0 * 65.0, 65.0), (35.0 + 8.0 * 65.0, 65.0),
            (35.0 + 9.0 * 65.0, 65.0),
            // 3. vrstica
            (35.0 + 0.0 * 65.0, 90.0), (35.0 + 1.0 * 65.0, 90.0), (35.0 + 2.0 * 65.0, 90.0),
            (35.0 + 3.0 * 65.0, 90.0), (35.0 + 4.0 * 65.0, 90.0), (35.0 + 5.0 * 65.0, 90.0),
            (35.0 + 6.0 * 65.0, 90.0), (35.0 + 7.0 * 65.0, 90.0), (35.0 + 8.0 * 65.0, 90.0),
            (35.0 + 9.0 * 65.0, 90.0),
            // 4. vrstica
            (35.0 + 1.0 * 65.0, 115.0), (35.0 + 2.0 * 65.0, 115.0), (35.0 + 3.0 * 65.0, 115.0),
            (35.0 + 4.0 * 65.0, 115.0), (35.0 + 5.0 * 65.0, 115.0), (35.0 + 6.0 * 65.0, 115.0),
            (35.0 + 7.0 * 65.0, 115.0), (35.0 + 8.0 * 65.0, 115.0),
            // 5. vrstica
            (35.0 + 2.0 * 65.0, 140.0), (35.0 + 3.0 * 65.0, 140.0),
            (35.0 + 4.0 * 65.0, 140.0), (35.0 + 5.0 * 65.0, 140.0),
            (35.0 + 6.0 * 65.0, 140.0), (35.0 + 7.0 * 65.0, 140.0),
            // 6. vrstica
            (35.0 + 3.0 * 65.0, 165.0), (35.0 + 4.0 * 65.0, 165.0),
            (35.0 + 5.0 * 65.0, 165.0), (35.0 + 6.0 * 65.0, 165.0),
            // 7. vrstica
            (35.0 + 4.0 * 65.0, 190.0), (35.0 + 5.0 * 65.0, 190.0),
        ]
    }

    pub fn spawn_bricks(&self) -> Vec<Brick> {
        self.brick_positions
            .iter()
            .map(|&(x, y)| Brick::new(x, y))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RandomLevelCfg {
    pub side_padding: f32, // odmik od levega/desnega roba
    pub top_padding: f32,          // odmik od zgoraj
    pub min_rows: usize,
    pub max_rows: usize,
    pub min_cols: usize,
    pub max_cols: usize,
    pub hole_pct: f32,     // verjetnost "luknje" v mreži
    pub row_offset: f32,   // horizontalni zamik vsake druge vrstice
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
    if let Some(s) = seed { srand(s); } else { srand((get_time() * 1_000_000.0) as u64); }

    let step_x = 65.0;
    let step_y = 25.0;

    let rows = gen_range(cfg.min_rows as i32, (cfg.max_rows + 1) as i32) as usize;
    let cols = gen_range(cfg.min_cols as i32, (cfg.max_cols + 1) as i32) as usize;

    let mut out = Vec::with_capacity(rows * cols);

    for r in 0..rows {
        let offset = if r % 2 == 0 { 0.0 } else { cfg.row_offset };
        for c in 0..cols {
            if gen_range(0.0, 1.0) < cfg.hole_pct { continue; }

            let x = cfg.side_padding + offset + c as f32 * step_x;
            let y = cfg.top_padding + r as f32 * step_y;

            if x + step_x > screen_width() - cfg.side_padding { continue; }

            out.push((x, y));
        }
    }

    out
}

pub fn assign_random_brick_kinds(bricks: &mut [Brick]) {
    let p_red:   f32 = 0.60;
    let p_green:  f32 = 0.25;
    let p_yellow:    f32 = 0.10;
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
}