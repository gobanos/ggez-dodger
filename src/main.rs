#![windows_subsystem = "windows"]

extern crate flexi_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate rand;
#[macro_use]
extern crate rand_derive;

use ggez::*;
use ggez::graphics::{Color, DrawMode, Point2, Rect, Vector2};

use flexi_logger::Logger;

use rand::{thread_rng, Rng};
use rand::distributions::{Range, Sample};

use std::collections::HashMap;
use std::{env, path};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const RADIUS: f32 = 32.0;
const GROUND_HEIGHT: f32 = 100.0;

const JUMP_HEIGHT: f32 = 25.0;
const JUMP_DEDUP: f32 = 1.5;
const FAST_DEDUP: f32 = 3.0;
const PLAYER_SPEED: f32 = 10.0;

const MAX_Y: f32 = HEIGHT - GROUND_HEIGHT - RADIUS;

struct Player {
    position: Point2,
    speed: Vector2,
    fast_dedup: bool,
    captured: Option<(BaddieColor, BaddieFace)>,
    score: u32,
}

impl Player {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.position.x = (self.position.x + self.speed.x)
            .min(WIDTH - RADIUS)
            .max(RADIUS);
        self.position.y = (self.position.y + self.speed.y).min(MAX_Y).max(0.0);

        if self.on_the_ground() {
            self.speed.y = 0.0;
        } else {
            self.speed.y += JUMP_DEDUP * if self.fast_dedup { FAST_DEDUP } else { 1.0 };
        }

        Ok(())
    }

    fn on_the_ground(&self) -> bool {
        self.position.y >= MAX_Y
    }

    fn rect(&self) -> Rect {
        Rect::new(
            self.position.x - RADIUS,
            self.position.y - RADIUS,
            RADIUS * 2.0,
            RADIUS * 2.0,
        )
    }
}

struct Baddie {
    body: Rect,
    speed: Vector2,
    color: BaddieColor,
    face: BaddieFace,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Rand)]
enum BaddieColor {
    Brown,
    Green,
    Blue,
    Yellow,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Rand)]
enum BaddieFace {
    Bad,
    Happy,
    Horrified,
    Sad,
    Sick,
    Wink,
}

impl Baddie {
    fn new() -> Baddie {
        let mut rng = thread_rng();

        let size = Range::new(20.0, 50.0).sample(&mut rng);
        let x = Range::new(0.0, WIDTH - size).sample(&mut rng);

        Baddie {
            body: Rect::new(x, -size, size, size),
            speed: Vector2::new(0.0, Range::new(1.5, 3.0).sample(&mut rng)),
            color: rng.gen(),
            face: rng.gen(),
        }
    }
}

struct Resources {
    baddies_colors: HashMap<BaddieColor, graphics::Image>,
    baddies_faces: HashMap<BaddieFace, graphics::Image>,
    font: graphics::Font,
}

impl Resources {
    fn new(ctx: &mut Context) -> GameResult<Resources> {
        let mut baddies_colors = HashMap::new();
        baddies_colors.insert(BaddieColor::Brown, graphics::Image::new(ctx, "/brown.png")?);
        baddies_colors.insert(BaddieColor::Green, graphics::Image::new(ctx, "/green.png")?);
        baddies_colors.insert(BaddieColor::Blue, graphics::Image::new(ctx, "/blue.png")?);
        baddies_colors.insert(
            BaddieColor::Yellow,
            graphics::Image::new(ctx, "/yellow.png")?,
        );

        let mut baddies_faces = HashMap::new();
        baddies_faces.insert(BaddieFace::Bad, graphics::Image::new(ctx, "/bad.png")?);
        baddies_faces.insert(BaddieFace::Happy, graphics::Image::new(ctx, "/happy.png")?);
        baddies_faces.insert(
            BaddieFace::Horrified,
            graphics::Image::new(ctx, "/horrified.png")?,
        );
        baddies_faces.insert(BaddieFace::Sad, graphics::Image::new(ctx, "/sad.png")?);
        baddies_faces.insert(BaddieFace::Sick, graphics::Image::new(ctx, "/sick.png")?);
        baddies_faces.insert(BaddieFace::Wink, graphics::Image::new(ctx, "/wink.png")?);

        Ok(Resources {
            baddies_colors,
            baddies_faces,
            font: graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?,
        })
    }
}

struct MainState {
    player: Player,
    timer: u32,
    baddies: Vec<Baddie>,
    resources: Resources,
    paused: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let player = Player {
            position: Point2::new(WIDTH / 2.0, MAX_Y),
            speed: Vector2::new(0.0, 0.0),
            fast_dedup: false,
            captured: None,
            score: 0,
        };

        let s = MainState {
            player,
            baddies: Vec::new(),
            timer: 0,
            resources: Resources::new(ctx)?,
            paused: false,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.paused {
            self.player.update(ctx)?;

            if self.timer % 10 == 0 {
                self.baddies.push(Baddie::new());
            }

            self.baddies.retain(|b| b.body.y < HEIGHT);

            let player_rect = self.player.rect();

            // used in place of drain_filter...
            let mut i = 0;
            while i != self.baddies.len() {
                if player_rect.overlaps(&self.baddies[i].body) {
                    let baddie = self.baddies.swap_remove(i);

                    self.player.captured = if let Some((c, f)) = self.player.captured.take() {
                        if c == baddie.color || f == baddie.face {
                            self.player.score += 1;
                            Some((baddie.color, baddie.face))
                        } else {
                            self.player.score = 0;
                            None
                        }
                    } else {
                        Some((baddie.color, baddie.face))
                    };
                } else {
                    i += 1;
                }
            }

            self.timer += 1;

            self.baddies
                .iter_mut()
                .for_each(|b| b.body.translate(b.speed));
        }

        Ok(())
    }

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::sleep_until_next_frame()`
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // draw player
        graphics::set_color(ctx, Color::from_rgb(255, 255, 255))?;

        if let Some((color, face)) = self.player.captured {
            let img = &self.resources.baddies_colors[&color];

            let Rect { w: iw, h: ih, .. } = img.get_dimensions();

            let scale = Point2::new(RADIUS * 2.0 / iw, RADIUS * 2.0 / ih);

            let params = graphics::DrawParam {
                dest: self.player.rect().point(),
                scale,
                ..Default::default()
            };

            graphics::draw_ex(ctx, img, params)?;
            graphics::draw_ex(ctx, &self.resources.baddies_faces[&face], params)?;
        } else {
            graphics::circle(ctx, DrawMode::Fill, self.player.position, RADIUS, 0.1)?;
        }

        // draw baddies
        graphics::set_color(ctx, Color::from_rgb(255, 255, 255))?;
        for baddie in &self.baddies {
            let img = &self.resources.baddies_colors[&baddie.color];

            let Rect { w: bw, h: bh, .. } = baddie.body;
            let Rect { w: iw, h: ih, .. } = img.get_dimensions();

            let scale = Point2::new(bw / iw, bh / ih);

            let params = graphics::DrawParam {
                dest: baddie.body.point(),
                scale,
                ..Default::default()
            };

            graphics::draw_ex(ctx, img, params)?;
            graphics::draw_ex(ctx, &self.resources.baddies_faces[&baddie.face], params)?;
        }

        // draw ground
        graphics::set_color(ctx, Color::from_rgb(0, 0, 0))?;
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(0.0, HEIGHT - GROUND_HEIGHT, WIDTH, GROUND_HEIGHT),
        )?;

        // draw score
        graphics::set_color(ctx, Color::from_rgb(255, 255, 255))?;
        let text = graphics::Text::new(
            ctx,
            &format!("SCORE: {}", self.player.score),
            &self.resources.font,
        )?;
        graphics::draw(ctx, &text, Point2::new(10.0, 10.0), 0.0)?;

        // draw paused
        if self.paused {
            graphics::set_color(ctx, Color::from_rgb(255, 255, 255))?;
            let text_pause = graphics::Text::new(ctx, "PAUSED", &self.resources.font)?;
            let pos_x =
                ctx.conf.window_mode.width / 2 - self.resources.font.get_width("PAUSED") as u32 / 2;
            let pos_y =
                ctx.conf.window_mode.height / 2 - self.resources.font.get_height() as u32 / 2;

            graphics::draw(
                ctx,
                &text_pause,
                Point2::new(pos_x as f32, pos_y as f32),
                0.,
            )?;
        }

        graphics::present(ctx);
        Ok(())
    }

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: i32,
        y: i32,
    ) {
        debug!("mouse_button_down_event - {:?}: ({},{})", button, x, y);
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: i32,
        y: i32,
    ) {
        debug!("mouse_button_up_event - {:?}: ({},{})", button, x, y);
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: event::MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        debug!(
            "mouse_motion_event - [STATE]: ({},{})/({},{})",
            x, y, xrel, yrel
        );
    }

    /// The mousewheel was clicked.
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        debug!("mouse_wheel_event - ({},{})", x, y);
    }

    /// A keyboard button was pressed.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::Keycode,
        keymod: event::Mod,
        repeat: bool,
    ) {
        debug!(
            "key_down_event - {:?} ({:?}): {}",
            keycode,
            keymod,
            if repeat { "repeated" } else { "first" }
        );

        if keycode == event::Keycode::Escape {
            ctx.quit().expect("Should never fail");
        }
    }

    /// A keyboard button was released.
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::Keycode,
        keymod: event::Mod,
        repeat: bool,
    ) {
        debug!(
            "key_up_event - {:?} ({:?}): {}",
            keycode,
            keymod,
            if repeat { "repeated" } else { "first" }
        );
    }

    /// A controller button was pressed; instance_id identifies which controller.
    fn controller_button_down_event(
        &mut self,
        _ctx: &mut Context,
        btn: event::Button,
        instance_id: i32,
    ) {
        debug!("controller_button_down_event - {:?} ({})", btn, instance_id);

        match btn {
            event::Button::DPadLeft => self.player.speed.x = -PLAYER_SPEED,
            event::Button::DPadRight => self.player.speed.x = PLAYER_SPEED,
            event::Button::DPadDown if !self.player.on_the_ground() => {
                self.player.fast_dedup = true
            }
            event::Button::B if self.player.on_the_ground() => {
                self.player.speed.y = -JUMP_HEIGHT;
                self.player.fast_dedup = false;
            }
            event::Button::Start => {
                self.paused = !self.paused;
            }
            _ => (),
        }
    }
    /// A controller button was released.
    fn controller_button_up_event(
        &mut self,
        _ctx: &mut Context,
        btn: event::Button,
        instance_id: i32,
    ) {
        debug!("controller_button_up_event - {:?} ({})", btn, instance_id);

        match btn {
            event::Button::DPadLeft | event::Button::DPadRight => {
                self.player.speed.x = 0.0;
            }
            event::Button::DPadDown => self.player.fast_dedup = false,
            _ => (),
        }
    }
    /// A controller axis moved.
    fn controller_axis_event(
        &mut self,
        _ctx: &mut Context,
        axis: event::Axis,
        value: i16,
        instance_id: i32,
    ) {
        debug!(
            "controller_axis_event - {:?}[{}] ({})",
            axis, value, instance_id
        );
    }

    /// Called when the window is shown or hidden.
    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        debug!("focus_event - {}", if gained { "gained" } else { "loose" });
    }
}

pub fn main() {
    Logger::with_env_or_str("ggez_dodger=warn, gfx_device_gl=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    info!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
