extern crate ggez;

use ggez::audio;
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

struct MainState;

fn draw_at(ctx: &mut Context, image: &graphics::Image, x: f32, y: f32) {
     let drawparams = graphics::DrawParam {
            dest: graphics::Point2::new(x, y),
            rotation: 0.0,
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, image, drawparams);
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let gHW = ctx.conf.window_mode.width / 2;
        let gHH = ctx.conf.window_mode.height / 2;

        let grass_image = graphics::Image::new(ctx, "/grass_tile.png")?;
        let iHW = grass_image.width() / 2;
        let iHH = grass_image.height() / 2;

        let gTilesPerRow = (2 * gHW) / (2 * iHW) + 1; 
        let gTilesPerCol = (2 * gHH) / (2 * iHH) + 1;

        for i in 0..gTilesPerRow {
            for j in 0..gTilesPerCol {
                draw_at(ctx, &grass_image, (i * 2 * iHW) as f32, (j * 2 * iHH) as f32);
            }
        }

        // let drawparams = graphics::DrawParam {
        //     dest: graphics::Point2::new((gHW - iHW) as f32, (gHH - iHH) as f32),
        //     rotation: 0.0,
        //     offset: graphics::Point2::new(0.5, 0.5),
        //     ..Default::default()
        // };
        // graphics::draw_ex(ctx, &grass_image, drawparams);

        // Then we flip the screen...
        graphics::present(ctx);

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();
        Ok(())
    }

}

fn main() {
    let mut cb = ContextBuilder::new("tilemap", "ggez")
        .window_setup(conf::WindowSetup::default().title("Tilemap!"))
        .window_mode(conf::WindowMode::default().dimensions(640, 480));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        // We need this re-assignment alas, see
        // https://aturon.github.io/ownership/builders.html
        // under "Consuming builders"
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let mut game = MainState{};
    let result = event::run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
