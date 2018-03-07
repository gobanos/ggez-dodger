#![windows_subsystem = "windows"]

extern crate flexi_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate rand;
#[macro_use]
extern crate rand_derive;

mod constants;
mod actions;
mod player;
mod baddies;
mod resources;
mod game;

use game::MainState;

use std::{env, path};

use ggez::{conf, event, graphics, Context};

use flexi_logger::Logger;

pub fn main() {
    Logger::with_env_or_str("ggez_dodger=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("dodger", "gobanos", c).unwrap();

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
