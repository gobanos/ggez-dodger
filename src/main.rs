extern crate ggez;
extern crate rand;

use ggez::*;
use ggez::graphics::{DrawMode, Point2, Rect, Vector2};

struct Baddie {
    body: Rect,
    speed: Vector2,
}

impl Baddie {
    fn new() -> Baddie {
        let x = rand::random::<f32>() * 800.0;
        Baddie {
            body: Rect::new(x, -10.0, 10.0, 10.0),
            speed: Vector2::new(0.0, 1.0),
        }
    }
}

struct MainState {
    pos_x: f32,
    timer: u32,
    baddies: Vec<Baddie>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0, baddies: Vec::new(), timer: 0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;

        if self.timer % 10 == 0 {
            self.baddies.push(Baddie::new());
        }

        self.baddies.retain(|b| b.body.y < 600.0);

        self.timer += 1;

        self.baddies.iter_mut().for_each(|b| b.body.translate(b.speed));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 380.0),
                         100.0,
                         0.1)?;

        for baddie in &self.baddies {
            graphics::rectangle(ctx,
                                DrawMode::Fill,
                                baddie.body)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}