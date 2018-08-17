extern crate ggez;
extern crate rpg;
extern crate tiled;

use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

use rpg::map::Map;
use rpg::input::InputState;
use rpg::util::load_tile_map;

struct MainState<'a> {
    map: Map<'a>,

    input: InputState,
}

impl<'a> MainState<'a> {
    pub fn new(image: graphics::Image, tilemap: &'a tiled::Map) -> GameResult<MainState<'a>> {
        let map = Map::new(image, tilemap, 0, 0);
        Ok(MainState {
            map,
            input: InputState::default(),
        })
    }
}

impl<'a> EventHandler for MainState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const SPEED: f32 = 100.0;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.map.map_cam_x += self.input.xaxis * SPEED * seconds;
            self.map.map_cam_y += self.input.yaxis * SPEED * seconds;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let (x, y) = (-self.map.map_cam_x, -self.map.map_cam_y);
        self.map.setup_draw(ctx, Point2::new(x, y))?;
        // graphics::draw(ctx, &self.map, Point2::new(-self.map.map_cam_x, -self.map.map_cam_y), 0.0)?;
        graphics::draw(
            ctx,
            &self.map,
            Point2::new(self.map.map_x, self.map.map_y),
            0.0,
        )?;
        // println!("x: {:?}, y: {:?}\n", self.map.map_cam_x, self.map.map_cam_y);
        graphics::present(ctx);

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        // timer::yield_now();
        Ok(())
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.input.key_down(keycode);
        if keycode == Keycode::Escape {
            ctx.quit().unwrap()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.input.key_up(keycode);
    }
}

fn main() {
    let mut cb = ContextBuilder::new("tilemap", "ggez")
        .window_setup(conf::WindowSetup::default().title("Tilemap!"))
        .window_mode(conf::WindowMode::default().dimensions(256 * 2, 224 * 2));

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

    let mut image = graphics::Image::new(ctx, "/cave16x16.png").unwrap();
    let tilemap = load_tile_map(ctx, "/larger_map.tmx").unwrap();
    let mut game = MainState::new(image, &tilemap).unwrap();
    let result = event::run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
