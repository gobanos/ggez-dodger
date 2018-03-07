use constants::*;
use resources::Resources;

use ggez::{Context, GameResult};
use graphics::{self, Color, Rect, Vector2};
use rand::{thread_rng, Rng};
use rand::distributions::{Range, Sample};

pub struct Baddie {
    pub body: Rect,
    pub color: BaddieColor,
    pub face: BaddieFace,

    speed: Vector2,
}

impl Baddie {
    pub fn new() -> Baddie {
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

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.body.translate(self.speed);
        Ok(())
    }

    pub fn draw(&self, res: &Resources, ctx: &mut Context) -> GameResult<()> {
        use self::graphics::*;

        // get bg & infos
        let bg = &res.baddies_bg;
        let Rect { w: iw, h: ih, .. } = bg.get_dimensions();

        set_color(ctx, self.color.into())?;
        let Rect { w: bw, h: bh, .. } = self.body;

        let scale = Point2::new(bw / iw, bh / ih);

        let params = DrawParam {
            dest: self.body.point(),
            scale,
            ..Default::default()
        };

        draw_ex(ctx, bg, params)?;
        draw_ex(ctx, &res.baddies_faces[&self.face], params)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Rand)]
pub enum BaddieColor {
    Brown,
    Green,
    Blue,
    Yellow,
}

impl Into<Color> for BaddieColor {
    fn into(self) -> Color {
        match self {
            BaddieColor::Brown => Color::from_rgb_u32(0x58_29_26),
            BaddieColor::Green => Color::from_rgb_u32(0x05_82_1a),
            BaddieColor::Blue => Color::from_rgb_u32(0x24_5e_97),
            BaddieColor::Yellow => Color::from_rgb_u32(0x8c_97_2c),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Rand)]
pub enum BaddieFace {
    Bad,
    Happy,
    Horrified,
    Sad,
    Sick,
    Wink,
}
