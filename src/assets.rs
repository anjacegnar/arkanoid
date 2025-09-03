use macroquad::prelude::*;
use crate::model::powerup::PowerUpType;

pub struct Assets {
    pub extend_paddle: Texture2D,
    pub extra_life: Texture2D,
    pub multiple_balls: Texture2D,
    pub slower_ball: Texture2D,
}

impl Assets {
    pub async fn load() -> Assets {
        let extend_paddle = load_texture("assets/extend_paddle.png").await.unwrap();
        extend_paddle.set_filter(FilterMode::Nearest);

        let extra_life = load_texture("assets/extra_life.png").await.unwrap();
        extra_life.set_filter(FilterMode::Nearest);

        let multiple_balls = load_texture("assets/multiple_balls.png").await.unwrap();
        multiple_balls.set_filter(FilterMode::Nearest);

        let slower_ball = load_texture("assets/slower_ball.png").await.unwrap();
        slower_ball.set_filter(FilterMode::Nearest);

        Assets { extend_paddle, extra_life, multiple_balls, slower_ball }
    }
    
    pub fn powerup_tex(&self, kind: PowerUpType) -> &Texture2D {
        match kind {
            PowerUpType::ExtendPaddle    => &self.extend_paddle,
            PowerUpType::ExtraLife       => &self.extra_life,
            PowerUpType::MultipleBalls   => &self.multiple_balls,
            PowerUpType::SlowerBall { .. } => &self.slower_ball,
        }
    }
}

