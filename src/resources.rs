use baddies::BaddieFace;
use ggez::{Context, GameResult};
use ggez::graphics::{Font, Image, Text};
use std::collections::HashMap;

// Struct containing the hard-coded resources of the game
pub struct Resources {
    pub baddies_bg: Image,
    pub baddies_faces: HashMap<BaddieFace, Image>,
    pub life: Image,
    pub font: Font,
    pub pause: Text,
    pub waiting: Text,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        let mut baddies_faces = HashMap::new();
        baddies_faces.insert(BaddieFace::Bad, Image::new(ctx, "/bad.png")?);
        baddies_faces.insert(BaddieFace::Happy, Image::new(ctx, "/happy.png")?);
        baddies_faces.insert(BaddieFace::Horrified, Image::new(ctx, "/horrified.png")?);
        baddies_faces.insert(BaddieFace::Sad, Image::new(ctx, "/sad.png")?);
        baddies_faces.insert(BaddieFace::Sick, Image::new(ctx, "/sick.png")?);
        baddies_faces.insert(BaddieFace::Wink, Image::new(ctx, "/wink.png")?);

        let font = Font::new(ctx, "/DejaVuSerif.ttf", 25)?;

        Ok(Resources {
            baddies_bg: Image::new(ctx, "/white.png")?,
            baddies_faces,
            life: Image::new(ctx, "/life.png")?,
            pause: Text::new(ctx, "PAUSED", &font)?,
            waiting: Text::new(ctx, "WAITING FOR A PLAYER", &font)?,
            font,
        })
    }
}
