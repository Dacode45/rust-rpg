extern crate ggez;
extern crate tiled;

use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::conf;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::timer;
use ggez::filesystem;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{Point2, Rect};
use std::env;
use std::path;


fn draw_at(ctx: &mut Context, image: &graphics::Image, x: f32, y: f32) {
     let drawparams = graphics::DrawParam {
            dest: graphics::Point2::new(x, y),
            rotation: 0.0,
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, image, drawparams).unwrap();
}
#[derive(Debug)]
struct Map<'a> {
    // pixel location of left of map
    map_x: f32,
    // pixel location of top of map
    map_y: f32,

    map_cam_x: f32,
    map_cam_y: f32,

    map_def: &'a tiled::Map,
    sprite_batch: SpriteBatch,
    
    // layer index to use
    layer_index: usize,
    // tileset to use
    tile_set: usize,
    // width of map in tiles
    map_width: f32,
    // height of map in tiles
    map_height: f32,

    tile_width: f32,
    tile_height: f32,

    map_pixel_width: f32,
    map_pixel_height: f32,

    uvs: Vec<Rect>,    
}

impl<'a> Map<'a> {
    pub fn new(image: graphics::Image, map_def: &'a tiled::Map, layer_index: usize, tile_set: usize) -> Self {
        Map{
            map_x: 0.0,
            map_y: 0.0,

            map_cam_x: 0.0,
            map_cam_y: 0.0,

            map_def,
            sprite_batch: SpriteBatch::new(image),

            layer_index,
            tile_set,

            map_width: map_def.width as f32,
            map_height: map_def.height as f32,

            tile_width: map_def.tilesets[tile_set].tile_width as f32,
            tile_height: map_def.tilesets[tile_set].tile_height as f32,

            map_pixel_width: (map_def.tilesets[tile_set].tile_width * map_def.width) as f32,
            map_pixel_height: (map_def.tilesets[tile_set].tile_height * map_def.height) as f32,

            uvs: generate_uvs(map_def, tile_set),
        }
    }

    pub fn point_to_tile(&self, x: f32, y:f32) -> (usize, usize) {
        let x = clamp(x, self.map_x, self.map_x + self.map_pixel_width - 1.0);
        let y = clamp(y, self.map_y, self.map_y + self.map_pixel_height - 1.0);

        let tile_x = ((x - self.map_x) / self.tile_width).floor();
        let tile_y = ((y - self.map_y) / self.tile_height).floor();

        (tile_x as usize, tile_y as usize)
    }

    fn setup_draw(&mut self, ctx: &mut Context, dest: Point2) -> GameResult<()> {
            let (w, h) = graphics::get_size(ctx);

            // ignore src until i figure out what to do with it 
            let start_x = self.map_cam_x;// - self.tile_width; // + param.src.left() * self.map_pixel_width;
            let start_y = self.map_cam_y;// - self.tile_height; // + param.src.top() * self.map_pixel_height;
            let end_x = start_x + w as f32; // + param.src.right() * self.map_pixel_width;
            let end_y = start_y + h as f32; // + param.src.bottom() * self.map_pixel_height;

        // println!("{}, {}, {}, {}", start_x, start_y, end_x, end_y);

        let (tile_left, tile_top) = self.point_to_tile(start_x, start_y);
        let (tile_right, tile_bottom) = self.point_to_tile(end_x, end_y);

        self.sprite_batch.clear();
        for j in tile_top..=(tile_bottom) {
            for i in tile_left..=(tile_right) {
                let x: f32 = self.map_x + self.tile_width * i as f32;
                let y: f32 = self.map_y + self.tile_height * j as f32;
                let tile_index = self.map_def.layers[self.layer_index].tiles[j as usize][i as usize];
                let uv = self.uvs[tile_index as usize - 1];
                // println!("wh: {} {}", uv.left() * self.map_pixel_width, uv.right() * self.map_pixel_width);

                let mut params = graphics::DrawParam::default();
                params.src = uv;
                params.dest = Point2::new(dest.x + x, dest.y + y);
                // TODO: Figure out reason for this hack
                // have to scale otherwise it looks like tearing
                params.scale = Point2::new(1.1, 1.1);
                self.sprite_batch.add(params);
                // if let Err(err) = graphics::draw_ex(ctx, self.image, params) {
                //     return Err(err)
                // }
            }
        }

            Ok(())
        }
}

impl<'a> graphics::Drawable for Map<'a> {
    fn draw_ex(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult<()> {
        self.sprite_batch.draw_ex(ctx, param)
    }
    fn set_blend_mode(&mut self, _: Option<graphics::BlendMode>) {}
    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}


#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

fn generate_uvs(map: &tiled::Map, tileset_id: usize) -> Vec<Rect> {
    let tileset = &map.tilesets[tileset_id];
        let i_width = tileset.images[0].width as f32;
        let i_height = tileset.images[0].height as f32;
        let t_width = tileset.tile_width as f32;
        let t_height = tileset.tile_height as f32;
        
        let width = t_width / i_width;
        let height = t_height / i_height;
        let cols = i_width / t_width;
        let rows = i_height / t_height;

        // println!("wh: {} {}", width, height);

        let mut ux: f32 = 0.0;
        let mut uy: f32 = 0.0;

        let mut uvs = Vec::new();

        for _ in 0..(rows as u32) {
            for _ in 0..(cols as u32) {
                uvs.push(
                    Rect::new(ux, uy, width, height)
                );
                ux += width;
            }
            ux = 0.0;
            uy += height;
        }
        return uvs
    
}

fn load_tile_map(ctx: &mut Context, tilemap_src: &str) -> GameResult<tiled::Map> {
    let tilemap_file = ctx.filesystem.open(tilemap_src)?;    
    match tiled::parse(tilemap_file) {
                Ok(map) => Ok(map),
                Err(_) => Err(ggez::GameError::from(String::from("tiled error")))
            }

}

#[derive(Debug)]
struct InputState {
    xaxis: f32,
    yaxis: f32,
    fire: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            fire: false,
        }
    }
}

struct MainState<'a> {
    map: Map<'a>,

    input: InputState,

}

impl<'a> MainState<'a> {
    pub fn new(image: graphics::Image, tilemap: &'a tiled::Map) -> GameResult<MainState<'a>> {
        let map = Map::new(image, tilemap, 0, 0);
        Ok(MainState{
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
        self.map.setup_draw(ctx, Point2::new(x,y))?;
        // graphics::draw(ctx, &self.map, Point2::new(-self.map.map_cam_x, -self.map.map_cam_y), 0.0)?;
        graphics::draw(ctx, &self.map, Point2::new(0.0, 0.0), 0.0)?;
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
        match keycode {
            Keycode::Up => {
                self.input.yaxis = -1.0;
            }
            Keycode::Down => {
                self.input.yaxis = 1.0;
            }
            Keycode::Left => {
                self.input.xaxis = -1.0;
            }
            Keycode::Right => {
                self.input.xaxis = 1.0;
            }
            Keycode::Space => {
                self.input.fire = true;
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => {
                self.input.yaxis = 0.0;
            }
            Keycode::Down => {
                self.input.yaxis = 0.0;
            }
            Keycode::Left => {
                self.input.xaxis = 0.0;
            }
            Keycode::Right => {
                self.input.xaxis = 0.0;
            }
            Keycode::Space => {
                self.input.fire = false;
            }
            _ => (), // Do nothing
        }
    }

}

fn main() {
    let mut cb = ContextBuilder::new("tilemap", "ggez")
        .window_setup(conf::WindowSetup::default().title("Tilemap!"))
        .window_mode(conf::WindowMode::default().dimensions(256*2, 224*2));

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
