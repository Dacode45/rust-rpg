use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2, Rect};
use ggez::{Context, GameResult};

use tiled;
use util;

#[derive(Debug)]
pub struct Map<'a> {
    // pixel location of left of map
    pub map_x: f32,
    // pixel location of top of map
    pub map_y: f32,

    pub map_cam_x: f32,
    pub map_cam_y: f32,

    pub map_def: &'a tiled::Map,
    pub sprite_batch: SpriteBatch,

    // layer index to use
    pub layer_index: usize,
    // tileset to use
    pub tile_set: usize,
    // width of map in tiles
    pub map_width: f32,
    // height of map in tiles
    pub map_height: f32,

    pub tile_width: f32,
    pub tile_height: f32,

    pub map_pixel_width: f32,
    pub map_pixel_height: f32,

    pub uvs: Vec<Rect>,
}

impl<'a> Map<'a> {
    pub fn new(
        image: graphics::Image,
        map_def: &'a tiled::Map,
        layer_index: usize,
        tile_set: usize,
    ) -> Self {
        Map {
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

    /// returns the bottom center of a tile
    pub fn get_tile_foot(&self, x: usize, y: usize) -> (f32, f32) {
        return (
            self.map_x + (self.tile_width * x as f32) + (self.tile_width / 2.0),
            self.map_y + (self.tile_height * y as f32) + self.tile_height
        )
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        self.map_cam_x = x;
        self.map_cam_y = y;
    }

    pub fn goto_tile(&mut self, x: f32, y: f32) {
        self.map_cam_x = x * self.tile_width + (self.tile_width / 2.0);
        self.map_cam_y = y * self.tile_height + (self.tile_height / 2.0);
    }

    pub fn point_to_tile(&self, x: f32, y: f32) -> (usize, usize) {
        let x = util::clamp(x, self.map_x, self.map_x + self.map_pixel_width - 1.0);
        let y = util::clamp(y, self.map_y, self.map_y + self.map_pixel_height - 1.0);

        let tile_x = ((x - self.map_x) / self.tile_width).floor();
        let tile_y = ((y - self.map_y) / self.tile_height).floor();

        (tile_x as usize, tile_y as usize)
    }

    pub fn setup_draw(&mut self, ctx: &mut Context, dest: Point2) -> GameResult<()> {
        let (w, h) = graphics::get_size(ctx);

        // ignore src until i figure out what to do with it
        let start_x = self.map_cam_x; // - self.tile_width; // + param.src.left() * self.map_pixel_width;
        let start_y = self.map_cam_y; // - self.tile_height; // + param.src.top() * self.map_pixel_height;
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
                let tile_index =
                    self.map_def.layers[self.layer_index].tiles[j as usize][i as usize];
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

pub fn generate_uvs(map: &tiled::Map, tileset_id: usize) -> Vec<Rect> {
    let tileset = &map.tilesets[tileset_id];
    let i_width = tileset.images[0].width as f32;
    let i_height = tileset.images[0].height as f32;
    let t_width = tileset.tile_width as f32;
    let t_height = tileset.tile_height as f32;

    return util::generate_uvs(i_width, i_height, t_width, t_height);
}
