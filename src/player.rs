use constants::*;
use baddies::{Baddie, BaddieColor, BaddieFace};
use actions::{MoveDirection, PlayerAction};
use resources::Resources;

use ggez::{Context, GameResult};
use graphics::{self, Point2, Rect, Vector2};

pub struct Player {
    position: Point2,
    speed: Vector2,
    captured: Option<(BaddieColor, BaddieFace)>,
    score: u32,
    fast_attenuation: bool,
    current_direction: MoveDirection,
}

impl Player {
    pub fn new(position: Point2) -> Player {
        Player {
            position,
            speed: Vector2::new(0.0, 0.0),
            captured: None,
            score: 0,
            fast_attenuation: false,
            current_direction: MoveDirection::Stop,
        }
    }

    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let wanted = self.wanted_speed();

        let damping = if self.on_the_ground() { PLAYER_DAMPING } else { FLYING_DAMPING };
        if self.speed.x > wanted {
            self.speed.x = wanted.max(self.speed.x - damping);
        } else {
            self.speed.x = wanted.min(self.speed.x + damping);
        }

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

    pub fn draw(&self, res: &Resources, ctx: &mut Context) -> GameResult<()> {
        use self::graphics::*;

        // draw player
        if let Some((color, face)) = self.captured {
            set_color(ctx, color.into())?;

            let img = &res.baddies_faces[&face];
            let Rect { w: iw, h: ih, .. } = img.get_dimensions();

            let scale = Point2::new(RADIUS * 2.0 / iw, RADIUS * 2.0 / ih);

            let params = DrawParam {
                dest: self.rect().point(),
                scale,
                ..Default::default()
            };

            circle(ctx, DrawMode::Fill, self.position, RADIUS, 0.1)?;
            draw_ex(ctx, img, params)?;
        } else {
            set_color(ctx, Color::from_rgb(255, 255, 255))?;
            circle(ctx, DrawMode::Fill, self.position, RADIUS, 0.1)?;
        }

        Ok(())
    }

    pub fn draw_ui(&self, res: &Resources, ctx: &mut Context) -> GameResult<()> {
        use self::graphics::*;

        // draw score
        set_color(ctx, Color::from_rgb(255, 255, 255))?;
        let text = Text::new(ctx, &format!("SCORE: {}", self.score), &res.font)?;
        draw(ctx, &text, Point2::new(10.0, 10.0), 0.0)?;

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

    pub fn process_action(&mut self, action: PlayerAction) -> GameResult<()> {
        use self::PlayerAction::*;

        match action {
            Move(dir) => self.current_direction = dir,
            Jump => {
                self.speed.y = -JUMP_HEIGHT;
                self.fast_attenuation = false;
            }
            Dump => self.fast_attenuation = true,
            StopDump => self.fast_attenuation = false,
        }

        Ok(())
    }

    pub fn collides(&mut self, baddie: &Baddie) {
        self.captured = if let Some((c, f)) = self.captured.take() {
            if c == baddie.color || f == baddie.face {
                self.score += 1;
                Some((baddie.color, baddie.face))
            } else {
                self.score = 0;
                None
            }
        } else {
            Some((baddie.color, baddie.face))
        };
    }

    pub fn overlaps(&self, rect: &Rect) -> bool {
        let radius = RADIUS - TOLERANCE;

        let dx = self.position.x - rect.x.max(self.position.x.min(rect.x + rect.w));
        let dy = self.position.y - rect.y.max(self.position.y.min(rect.y + rect.h));
        (dx * dx + dy * dy) < (radius * radius)
    }

    fn wanted_speed(&self) -> f32 {
        use self::MoveDirection::*;

        match self.current_direction {
            Stop => 0.0,
            Left => -PLAYER_SPEED,
            Right => PLAYER_SPEED,
        }
    }
}
