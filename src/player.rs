use constants::*;
use baddies::{BaddieColor, BaddieFace};

use ggez::{Context, GameResult};
use graphics::{Point2, Rect, Vector2};

pub struct Player {
    pub position: Point2,
    pub speed: Vector2,
    pub fast_attenuation: bool,
    pub captured: Option<(BaddieColor, BaddieFace)>,
    pub score: u32,
}

impl Player {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.position.x = (self.position.x + self.speed.x)
            .min(WIDTH - RADIUS)
            .max(RADIUS);
        self.position.y = (self.position.y + self.speed.y).min(MAX_Y).max(0.0);

        if self.on_the_ground() {
            self.speed.y = 0.0;
        } else {
            self.speed.y += JUMP_ATTENUATION * if self.fast_attenuation {
                FAST_ATTENUATION
            } else {
                1.0
            };
        }

        Ok(())
    }

    pub fn on_the_ground(&self) -> bool {
        self.position.y >= MAX_Y
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.position.x - RADIUS,
            self.position.y - RADIUS,
            RADIUS * 2.0,
            RADIUS * 2.0,
        )
    }
}
