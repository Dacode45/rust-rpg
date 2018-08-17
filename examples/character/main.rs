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

use rpg::map::Map;
use rpg::input::InputState;
use rpg::util::{self, load_tile_map};

struct Player {
    pub x: f32,
    pub y: f32,
    
    sprite: Sprite,
}

impl Player {
    pub fn new(sprite: Sprite) -> Self {
        return Player{
            x: 0.0,
            y: 0.0,

            sprite,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn teleport(&mut self, tile_x: usize, tile_y: usize, map: &Map) {
        let (x, y) = map.get_tile_foot(tile_x, tile_y);
        let x = x - self.sprite.width / 2.0;
        let y = y - self.sprite.height;

        self.set_position(x, y);
    }
}

impl graphics::Drawable for Player {
    fn draw_ex(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult<()> {
        self.sprite.draw_ex(ctx, param)
    }
    fn set_blend_mode(&mut self, _: Option<graphics::BlendMode>) {}
    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

struct Sprite {
    pub width: f32,
    pub height: f32,

    pub sprite_batch: SpriteBatch,
    pub uvs: Vec<Rect>,
    frame: usize,
}

impl Sprite {
    pub fn new(image: graphics::Image, width: f32, height: f32) -> Self {
        let uvs = util::generate_uvs(image.width() as f32, image.height() as f32, width, height);
        Sprite {
            width,
            height,

            sprite_batch: SpriteBatch::new(image),
            uvs,
            frame: 0,
        }
    }

    

    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
        self.sprite_batch.clear();
        
        let mut param = graphics::DrawParam::default();
    
        param.src = self.uvs[frame];
        self.sprite_batch.add(param);
    }

    
}

impl graphics::Drawable for Sprite {
    fn draw_ex(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult<()> {
        self.sprite_batch.draw_ex(ctx, param)
    }
    fn set_blend_mode(&mut self, _: Option<graphics::BlendMode>) {}
    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

struct MainState<'a> {
    map: Map<'a>,
    player: Player,

    hero_tile_x: usize,
    hero_tile_y: usize,

    input: InputState,
}

impl<'a> MainState<'a> {
    pub fn new(map: Map<'a>, mut player: Player) -> GameResult<MainState<'a>> {
        let hero_tile_x = 10;
        let hero_tile_y = 2;
        
        player.teleport(hero_tile_x, hero_tile_y, &map);
        player.sprite.set_frame(8);

        Ok(MainState {
            map,
            player,

            hero_tile_x,
            hero_tile_y,

            input: InputState::default(),
        })
    }
}

impl<'a> EventHandler for MainState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        
        while timer::check_update_time(ctx, DESIRED_FPS) {
            // println!("pos {:?} {}", self.input.just_pressed_xaxis, self.hero_tile_x);
            // xaxis
            match self.input.just_pressed_xaxis {
                Some(xaxis) if self.hero_tile_x as f32 + xaxis > 0.0 && self.hero_tile_x as f32 + xaxis < self.map.map_width => {
                    let mut x = self.hero_tile_x as i32;
                    x += xaxis as i32;
                    self.hero_tile_x = x as usize;
                }
                _ => ()
            }
            // yaxis
            match self.input.just_pressed_yaxis {
                Some(yaxis) if self.hero_tile_y as f32 + yaxis > 0.0 && self.hero_tile_y as f32 + yaxis < self.map.map_width => {
                    let mut x = self.hero_tile_y as i32;
                    x += yaxis as i32;
                    self.hero_tile_y = x as usize;
                }
                _ => ()
            }
            self.player.teleport(self.hero_tile_x, self.hero_tile_y, &self.map);
            self.input.advance();
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

        // draw player
        graphics::draw(ctx, &self.player, Point2::new(self.player.x, self.player.y), 0.0);
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

    let mut image = graphics::Image::new(ctx, "/character/rpg_indoor.png").unwrap();
    let tilemap = load_tile_map(ctx, "/character/small_room.tmx").unwrap();
    let map = Map::new(image, &tilemap, 0, 0);

    let mut p_image = graphics::Image::new(ctx, "/character/walk_cycle.png").unwrap();
    let p_sprite = Sprite::new(p_image, 16.0, 24.0);
    let player = Player::new(p_sprite);

    let mut game = MainState::new(map, player).unwrap();
    let result = event::run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
