use actions::{Entity, MoveDirection, PlayerAction};
use baddies::{Baddie, BaddieColor, BaddieFace};
use constants::*;
use ggez::{Context, GameResult};
use graphics::{self, Point2, Rect, Vector2};
use resources::Resources;

#[derive(Debug, Copy, Clone)]
pub struct PlayerBody {
    position: Point2,
    speed: Vector2,
    shielded: bool,
}

impl PlayerBody {
    fn new(position: Point2) -> PlayerBody {
        PlayerBody {
            position,
            speed: Vector2::new(0.0, 0.0),
            shielded: false,
        }
    }

    pub fn radius(&self) -> f32 {
        RADIUS - TOLERANCE + if self.shielded { 5.0 } else { 0.0 }
    }

    pub fn on_the_ground(&self) -> bool {
        self.position.y >= MAX_Y
    }
}

#[derive(Debug)]
pub struct Player {
    body: PlayerBody,
    captured: Option<(BaddieColor, BaddieFace)>,
    score: u32,
    life: i32,
    fast_attenuation: bool,
    current_direction: Option<MoveDirection>,
    index: u8,
}

impl Player {
    pub fn new(index: u8, position: Point2) -> Player {
        Player {
            body: PlayerBody::new(position),
            captured: None,
            score: 0,
            life: START_PLAYER_LIFE,
            fast_attenuation: false,
            current_direction: None,
            index,
        }
    }

    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let wanted = self.wanted_speed();

        let body = &mut self.body;

        let damping = if body.on_the_ground() {
            PLAYER_DAMPING
        } else {
            FLYING_DAMPING
        };
        if body.speed.x > wanted {
            body.speed.x = wanted.max(body.speed.x - damping);
        } else {
            body.speed.x = wanted.min(body.speed.x + damping);
        }

        body.position.x = (body.position.x + body.speed.x)
            .min(WIDTH - RADIUS)
            .max(RADIUS);
        body.position.y = (body.position.y + body.speed.y).min(MAX_Y).max(0.0);

        if body.on_the_ground() {
            body.speed.y = 0.0;
        } else {
            body.speed.y += JUMP_ATTENUATION * if self.fast_attenuation {
                FAST_ATTENUATION
            } else {
                1.0
            };
        }

        Ok(())
    }

    pub fn draw(&self, res: &Resources, ctx: &mut Context) -> GameResult<()> {
        use self::graphics::*;

        let body = &self.body;

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

            circle(ctx, DrawMode::Fill, body.position, RADIUS, 0.1)?;
            draw_ex(ctx, img, params)?;
        } else {
            set_color(ctx, Color::from_rgb(255, 255, 255))?;
            circle(ctx, DrawMode::Fill, body.position, RADIUS, 0.1)?;
        }

        if body.shielded {
            circle(ctx, DrawMode::Line(1.0), body.position, RADIUS + 5.0, 0.1)?;
        }

        Ok(())
    }

    pub fn draw_ui(&self, res: &Resources, nb_players: usize, ctx: &mut Context) -> GameResult<()> {
        use self::graphics::*;

        let col = f32::from(self.index % 2);
        let line = f32::from(self.index / 2);

        let max_width = WIDTH / nb_players.min(2) as f32;
        let start_x = max_width * col;
        let start_y = (LIFE_IMAGE_SIZE + UI_MARGIN) * line;

        // draw thumb
        let radius = LIFE_IMAGE_SIZE / 2.0;
        let pos = Point2::new(start_x + UI_MARGIN + radius, start_y + UI_MARGIN + radius);

        if let Some((color, face)) = self.captured {
            set_color(ctx, color.into())?;

            let img = &res.baddies_faces[&face];
            let Rect { w: iw, h: ih, .. } = img.get_dimensions();

            let scale = Point2::new(LIFE_IMAGE_SIZE / iw, LIFE_IMAGE_SIZE / ih);

            let params = DrawParam {
                dest: Point2::new(start_x + UI_MARGIN, start_y + UI_MARGIN),
                scale,
                ..Default::default()
            };

            circle(ctx, DrawMode::Fill, pos, radius, 0.1)?;
            draw_ex(ctx, img, params)?;
        } else {
            set_color(ctx, Color::from_rgb(255, 255, 255))?;
            circle(ctx, DrawMode::Fill, pos, radius, 0.1)?;
        }

        // draw score
        set_color(ctx, Color::from_rgb(255, 255, 255))?;
        let text = Text::new(
            ctx,
            &format!("PLAYER {}: {}", self.index + 1, self.score),
            &res.font,
        )?;
        draw(
            ctx,
            &text,
            Point2::new(
                start_x + UI_MARGIN * 2.0 + LIFE_IMAGE_SIZE,
                start_y + UI_MARGIN,
            ),
            0.0,
        )?;

        // draw lifes
        (0..self.life).for_each(|i| {
            let i = (i + 1) as f32;
            let x = start_x + max_width - i * (LIFE_IMAGE_SIZE + UI_MARGIN);
            draw(ctx, &res.life, Point2::new(x, start_y + UI_MARGIN), 0.0)
                .expect("Failed to draw a heart");
        });

        Ok(())
    }

    pub fn on_the_ground(&self) -> bool {
        self.body.on_the_ground()
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.body.position.x - RADIUS,
            self.body.position.y - RADIUS,
            RADIUS * 2.0,
            RADIUS * 2.0,
        )
    }

    pub fn body(&self) -> PlayerBody {
        self.body
    }

    pub fn process_action(&mut self, action: PlayerAction) -> GameResult<()> {
        use self::PlayerAction::*;

        match action {
            Move(dir) => self.current_direction = dir,
            Jump => {
                self.body.speed.y = -JUMP_HEIGHT;
                self.fast_attenuation = false;
            }
            Dump(dump) => self.fast_attenuation = dump,
            Shield(shield) => self.body.shielded = shield,
            Collides(Entity::Baddie(baddie)) => self.collides_with_baddie(&baddie),
            Collides(Entity::Player(other)) => self.collides_with_player(&other),
        }

        Ok(())
    }

    pub fn collides_with_baddie(&mut self, baddie: &Baddie) {
        if self.body.shielded {
            return;
        }

        let Rect { x, y, w, h } = baddie.body;
        let pos = Point2::new(x + w / 2.0, y + h / 2.0);

        self.captured = if let Some((c, f)) = self.captured.take() {
            if c == baddie.color || f == baddie.face {
                self.score += 1;
                Some((baddie.color, baddie.face))
            } else {
                self.score = self.score.saturating_sub(1);

                let mut dir = self.body.position - pos;
                if self.on_the_ground() {
                    dir.y = 0.0;
                }
                let dir = dir.normalize() * (w / 5.0);

                self.body.speed += dir;
                self.life -= 1;
                None
            }
        } else {
            Some((baddie.color, baddie.face))
        };
    }

    pub fn collides_with_player(&mut self, other: &PlayerBody) {
        // swap speed
        self.body.speed = other.speed;

        let my_radius = self.body.radius();
        let their_radius = other.radius();

        let diff = self.body.position - other.position;

        let dist = (diff.x * diff.x + diff.y * diff.y).sqrt();
        let factor = (1.0 - dist / (my_radius + their_radius)) / if other.shielded { 1.0 } else { 2.0 };

        self.body.speed.x += diff.x * factor;
        self.body.speed.y += diff.y * factor;
    }

    pub fn overlaps(&self, rect: &Rect) -> bool {
        let body = &self.body;
        let radius = body.radius();

        let dx = body.position.x - rect.x.max(body.position.x.min(rect.x + rect.w));
        let dy = body.position.y - rect.y.max(body.position.y.min(rect.y + rect.h));
        (dx * dx + dy * dy) < (radius * radius)
    }

    pub fn overlaps_player(&self, other: &PlayerBody) -> bool {
        let body = &self.body;
        let my_radius = body.radius();
        let their_radius = other.radius();

        let diff = body.position - other.position;

        let square_dist = diff.x * diff.x + diff.y * diff.y;
        (my_radius + their_radius) * (my_radius + their_radius) > square_dist
    }

    fn wanted_speed(&self) -> f32 {
        use self::MoveDirection::*;

        match self.current_direction {
            None => 0.0,
            Some(Left) => -PLAYER_SPEED,
            Some(Right) => PLAYER_SPEED,
        }
    }
}
