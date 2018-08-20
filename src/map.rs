use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2, Rect};
use ggez::{Context, GameResult};

use sprite::{Sprite, SpriteComponent};
use tiled;
use util;

#[derive(Debug)]
pub struct Map {
    // pixel location of top left of map
    pub pos: Point2,
    pub camera: Rect,

    pub layers: Vec<tiled::Layer>,
    pub tilesets: Vec<tiled::Tileset>,
    // layer index to use
    pub layer_index: usize,
    // tileset to use
    pub tile_set: usize,

    pub dimensions: Point2,

    pub tile_dimensions: Point2,

    pub pixel_dimensions: Point2,
}

impl Map {
    pub fn new(map_def: &tiled::Map, layer_index: usize, tile_set: usize) -> Self {
        let layers = map_def.layers.clone();
        let tilesets = map_def.tilesets.clone();

        let zero = Point2::new(0.0, 0.0);
        Map {
            pos: zero.clone(),
            camera: Rect::new(0.0, 0.0, 1.0, 1.0),

            layers,
            tilesets,
            layer_index,
            tile_set,

            dimensions: Point2::new(map_def.width as f32, map_def.height as f32),

            tile_dimensions: Point2::new(
                map_def.tilesets[tile_set].tile_width as f32,
                map_def.tilesets[tile_set].tile_height as f32,
            ),

            pixel_dimensions: Point2::new(
                (map_def.tilesets[tile_set].tile_width * map_def.width) as f32,
                (map_def.tilesets[tile_set].tile_height * map_def.height) as f32,
            ),
        }
    }

    /// returns the bottom center of a tile
    pub fn get_tile_foot(&self, x: usize, y: usize) -> graphics::Point2 {
        let x = self.pos.x + (self.tile_dimensions.x * x as f32) + self.tile_dimensions.x / 2.0;
        let y = self.pos.y + (self.tile_dimensions.y * y as f32) + self.tile_dimensions.y;
        Point2::new(x, y)
    }

    pub fn goto(&mut self, pos: graphics::Point2) {
        self.camera.x = pos.x;
        self.camera.y = pos.y;
    }

    pub fn goto_tile(&mut self, x: usize, y: usize) {
        let x = (self.tile_dimensions.x * x as f32) + (0.5 * self.tile_dimensions.x);
        let y = (self.tile_dimensions.y * y as f32) + (0.5 * self.tile_dimensions.y);
        self.goto(Point2::new(x, y));
    }

    /// converts world pixel coordinates to tile in map
    pub fn point_to_tile(&self, x: f32, y: f32) -> (usize, usize) {
        let x = util::clamp(x, self.pos.x, self.pos.y + self.pixel_dimensions.x - 1.0);
        let y = util::clamp(y, self.pos.y, self.pos.y + self.pixel_dimensions.y - 1.0);

        let tile_x = ((x - self.pos.x) / self.tile_dimensions.x).floor();
        let tile_y = ((y - self.pos.y) / self.tile_dimensions.y).floor();

        (tile_x as usize, tile_y as usize)
    }
}

impl SpriteComponent for Map {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        let (tile_left, tile_top) = self.point_to_tile(self.camera.left(), self.camera.top());
        let (tile_right, tile_bottom) =
            self.point_to_tile(self.camera.right(), self.camera.bottom());

        sprite.sprite_batch.clear();
        for j in tile_top..=(tile_bottom) {
            for i in tile_left..=(tile_right) {
                let x: f32 = self.pos.x + self.tile_dimensions.x * i as f32;
                let y: f32 = self.pos.y + self.tile_dimensions.y * j as f32;
                let tile_index = self.layers[self.layer_index].tiles[j as usize][i as usize];
                let uv = sprite.uvs[tile_index as usize - 1];
                // println!("wh: {} {}", uv.left() * self.map_pixel_width, uv.right() * self.map_pixel_width);

                let mut params = graphics::DrawParam::default();
                params.src = uv;
                params.dest = Point2::new(-self.camera.left() + x, -self.camera.top() + y);
                // TODO: Figure out reason for this hack
                // have to scale otherwise it looks like tearing
                params.scale = Point2::new(1.1, 1.1);
                sprite.sprite_batch.add(params);
                // if let Err(err) = graphics::draw_ex(ctx, self.image, params) {
                //     return Err(err)
                // }
            }
        }
    }

    /// Where to draw the sprite map at
    fn draw_sprite_at(&self) -> graphics::Point2 {
        self.pos.clone()
    }
}

// uvs from tiled generates uvs for a map tileset
pub fn uvs_from_tiled(map: &tiled::Map, tileset_id: usize) -> Vec<Rect> {
    let tileset = &map.tilesets[tileset_id];
    let i_width = tileset.images[0].width as f32;
    let i_height = tileset.images[0].height as f32;
    let t_width = tileset.tile_width as f32;
    let t_height = tileset.tile_height as f32;

    return util::generate_uvs(i_width, i_height, t_width, t_height);
}
