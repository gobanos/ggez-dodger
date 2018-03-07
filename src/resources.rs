use baddies::BaddieFace;

use std::collections::HashMap;

use ggez::{Context, GameResult};
use ggez::graphics::{Font, Image};

pub struct Resources {
    pub baddies_bg: Image,
    pub baddies_faces: HashMap<BaddieFace, Image>,
    pub font: Font,
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

        Ok(Resources {
            baddies_bg: Image::new(ctx, "/white.png")?,
            baddies_faces,
            font: Font::new(ctx, "/DejaVuSerif.ttf", 48)?,
        })
    }
}
