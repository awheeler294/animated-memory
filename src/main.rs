extern crate good_web_game as ggez;

use std::env;
use std::path;

use game::GameManager;
use getrandom::register_custom_getrandom;
use good_web_game::GameResult;

mod color_scheme;
mod game;
mod menu;
mod screen;

use color_scheme::{ColorPalette, TweenableColor};
use screen::{SCREEN_WIDTH, SCREEN_HEIGHT};

fn fallback_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Ok(())
}

register_custom_getrandom!(fallback_getrandom);


fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let conf = ggez::conf::Conf::default()
        .window_width(SCREEN_WIDTH)
        .window_height(SCREEN_HEIGHT)
        .physical_root_dir(Some(resource_dir));

    let game_manager = GameManager::new();

    ggez::start(
        conf,
        |mut _ctx, mut _gctx| Box::new(game_manager),
    )

}
