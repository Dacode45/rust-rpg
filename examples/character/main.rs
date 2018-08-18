extern crate ggez;
extern crate rpg;
extern crate tiled;

use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2, Rect};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};


use std::env;
use std::path;

use rpg::map::{Map, uvs_from_tiled};
use rpg::input::InputState;
use rpg::util::{self, load_tile_map};
use rpg::sprite::Sprite;
use rpg::entity::Entity;

struct MainState {
    map_sprite: Sprite,
    player_sprite: Sprite,
    map: Map,
    player: Entity,

    input: InputState,
}

impl MainState {
    pub fn new(map_sprite: Sprite, player_sprite: Sprite, mut map: Map, mut player: Entity) -> GameResult<MainState> {
        let camera = Rect::new(0.0, 0.0, map.pixel_dimensions.x, map.pixel_dimensions.y);
        map.camera = camera;
        player.teleport(10, 2, &map);
        
        Ok(MainState {
            map_sprite,
            player_sprite,

            map,
            player,

            input: InputState::default(),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        
        while timer::check_update_time(ctx, DESIRED_FPS) {
            // println!("pos {:?} {}", self.input.just_pressed_xaxis, self.hero_tile_x);
            // xaxis
            if let Some(xaxis) = self.input.just_pressed_xaxis {
                let next = self.player.tile_x as f32 + xaxis;
                if next >= 0.0 && next < self.map.dimensions.x {
                    self.player.tile_x = next as usize;
                }
            }
            if let Some(yaxis) = self.input.just_pressed_yaxis {
                let next = self.player.tile_y as f32 + yaxis;
                if next >= 0.0 && next < self.map.dimensions.y {
                    self.player.tile_y = next as usize;
                }
            }
            
            let (tx, ty) = (self.player.tile_x, self.player.tile_y);
            self.player.teleport(tx, ty, &self.map);
            self.input.advance();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        {
            let s = self.map_sprite.with_context(&self.map);
            graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0);
        }
        {
            let s = self.player_sprite.with_context(&self.player);
            graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0);
        }
        graphics::present(ctx);

        timer::yield_now();
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

    let mut image = graphics::Image::new(ctx, "/character/rpg_indoor.png").unwrap();
    let tilemap = load_tile_map(ctx, "/character/small_room.tmx").unwrap();
    let mut sprite = Sprite::new(image, 0.0, 0.0);
    sprite.uvs = uvs_from_tiled(&tilemap, 0);
    let map = Map::new(&tilemap, 0, 0);

    let mut p_image = graphics::Image::new(ctx, "/character/walk_cycle.png").unwrap();
    let p_sprite = Sprite::new(p_image, 16.0, 24.0);
    let mut player = Entity::new(Point2::new(16.0, 24.0));
    player.set_frame(9);

    let mut game = MainState::new(sprite, p_sprite, map, player).unwrap();
    let result = event::run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
