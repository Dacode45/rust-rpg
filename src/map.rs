use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2, Rect};
use ggez::{Context, GameResult};

use std::collections::HashMap;

use sprite::{Sprite, SpriteComponent};
use tiled;
use util;
use entity;

#[derive(Debug)]
struct IndexPair(usize, usize);

#[derive(Debug)]
pub struct Map {
    // pixel location of top left of map
    pos: Point2,
    camera: Rect,

    layers: Vec<tiled::Layer>,
    tilesets: Vec<tiled::Tileset>,

    // layer index to use
    layer_index: usize,
    // tileset to use
    tile_set: usize,

    dimensions: Point2,
    tile_dimensions: Point2,
    pixel_dimensions: Point2,

    // gid of tileset with blocking layer
    blocking_tile: Option<u32>,

    entities: Vec<HashMap<usize, entity::Entity>>,
    entityid_to_index: HashMap<String, IndexPair>,
}

impl Map {
    pub fn new(map_def: &tiled::Map, layer_index: usize, tile_set: usize) -> Self {
        let zero = Point2::new(0.0, 0.0);
        let layers = map_def.layers.clone();
        let tilesets = map_def.tilesets.clone();
        let mut blocking_tile = None;

        let dimensions = Point2::new(map_def.width as f32, map_def.height as f32);
        let tile_dimensions = Point2::new(
                map_def.tilesets[tile_set].tile_width as f32,
                map_def.tilesets[tile_set].tile_height as f32,
            );

        let pixel_dimensions = Point2::new(
                (map_def.tilesets[tile_set].tile_width * map_def.width) as f32,
                (map_def.tilesets[tile_set].tile_height * map_def.height) as f32,
            );

        for tileset in tilesets.iter() {
            if tileset.name == "collision_graphic" {
                blocking_tile = Some(tileset.first_gid);
                break;
            }
        }

        // initialize entities
        let entities = Vec::new();

        Map {
            pos: zero.clone(),
            camera: Rect::new(0.0, 0.0, 1.0, 1.0),

            layers,
            tilesets,
            layer_index,
            tile_set,
            blocking_tile,
            
            dimensions,
            tile_dimensions,
            pixel_dimensions,

            entities,
            entityid_to_index: HashMap::new(),
        }
    }

    pub fn coord_to_index(&self, x: usize, y:usize) -> usize {
        x + y * self.dimensions.x as usize
    }

    pub fn get_tile(&self, x: usize, y: usize, layer: usize) -> usize {
        self.layers[layer].tiles[y][x] as usize
    }

    pub fn get_tile_foot(&self, x: usize, y: usize) -> graphics::Point2 {
        let x = self.pos.x + (self.tile_dimensions.x * x as f32) + self.tile_dimensions.x / 2.0;
        let y = self.pos.y + (self.tile_dimensions.y * y as f32) + self.tile_dimensions.y;
        Point2::new(x, y)
    }

    // Sets the camera position to point
    pub fn goto(&mut self, pos: graphics::Point2) {
        self.camera.x = pos.x;
        self.camera.y = pos.y;
    }

    // Sets the camera position to tile
    pub fn goto_tile(&mut self, x: usize, y: usize) {
        let x = (self.tile_dimensions.x * x as f32) + (0.5 * self.tile_dimensions.x);
        let y = (self.tile_dimensions.y * y as f32) + (0.5 * self.tile_dimensions.y);
        self.goto(Point2::new(x, y));
    }

    pub fn is_blocked(&self, layer: usize, tile_x: usize, tile_y: usize) -> bool {
        match self.blocking_tile {
            None => false,
            Some(blocking_tile) => {
                let tile = self.get_tile(tile_x, tile_y, layer + 2);
                tile == blocking_tile as usize
            }
        }
    }

    pub fn layer_count(&self) -> usize {
        assert!(
            self.layers.len() % 3 == 0,
            "Number of layers hosuld be a factor of 3"
        );
        self.layers.len() / 3
    }

    /// converts world pixel coordinates to tile in map
    pub fn point_to_tile(&self, x: f32, y: f32) -> (usize, usize) {
        let x = util::clamp(x, self.pos.x, self.pos.y + self.pixel_dimensions.x - 1.0);
        let y = util::clamp(y, self.pos.y, self.pos.y + self.pixel_dimensions.y - 1.0);

        let tile_x = ((x - self.pos.x) / self.tile_dimensions.x).floor();
        let tile_y = ((y - self.pos.y) / self.tile_dimensions.y).floor();

        (tile_x as usize, tile_y as usize)
    }

    // overrides the layer tile
    pub fn write_tile(&mut self, x: usize, y: usize, layer: usize, tile: u32, detail: Option<u32>, collision: bool) {
        let layer = layer * 3;
        self.layers[layer].tiles[y][x] = tile;
        if let Some(d) = detail {
            self.layers[layer + 1].tiles[y][x] = d;
        }
        if collision {
            if let Some(blocking_tile) = self.blocking_tile {
                self.layers[layer + 2].tiles[y][x] = blocking_tile;
            }
        }

    }

    pub fn tile_draw_params(
        &self,
        uvs: &Vec<Rect>,
        tile_x: usize,
        tile_y: usize,
        tile: usize,
    ) -> graphics::DrawParam {
        let x: f32 = self.pos.x + self.tile_dimensions.x * tile_x as f32;
        let y: f32 = self.pos.y + self.tile_dimensions.y * tile_y as f32;

        // subtract 1 because tiled indexes by 1
        let uv = uvs[tile - 1];
        // println!("wh: {} {}", uv.left() * self.map_pixel_width, uv.right() * self.map_pixel_width);

        let mut params = graphics::DrawParam::default();
        params.src = uv;
        params.dest = Point2::new(-self.camera.left() + x, -self.camera.top() + y);
        // TODO: Figure out reason for this hack
        // have to scale otherwise it looks like tearing
        params.scale = Point2::new(1.1, 1.1);
        params
    }

    /// entity functions

    pub fn entity(&self, x: usize, y:usize, layer:usize) -> Option<&entity::Entity> {
        if layer >= self.entities.len() {
            return None
        }
        let map = &self.entities[layer];
        match map.get(&self.coord_to_index(x, y)) {
            Some(entity) => Some(&entity),
            None => None
        }
    }

    pub fn entity_mut<'a>(&'a mut self, x: usize, y:usize, layer:usize) -> Option<&'a mut entity::Entity> {
        if layer >= self.entities.len() {
            return None
        }
        let index = self.coord_to_index(x, y);
        match self.entities[layer].get_mut(&index) {
            Some(e) => Some(e),
            None => None
        }
    }

    pub fn entities_of_layer(&self, layer: usize) -> Option<Vec<&entity::Entity>> {
        if layer >= self.entities.len() {
            return None
        }
        let map =& self.entities[layer];
        let entities: Vec<&entity::Entity> = map.values().collect();
        Some(entities)
    }

    pub fn add_entity(&mut self, e : entity::Entity) -> &mut entity::Entity {
        while e.layer() >= self.entities.len() {
            self.entities.push(HashMap::new());
        }

        let index = self.coord_to_index(e.tile_x(), e.tile_y());
        self.entityid_to_index.insert(e.id.clone(), IndexPair(e.layer, index));
        
        let layer = &mut self.entities[e.layer()];

        assert!(layer.get(&index).is_none());
        layer.insert(index, e);
        layer.get_mut(&index).unwrap()

    }

    pub fn remove_entity(&mut self, e: &entity::Entity) -> entity::Entity {
        let index = self.coord_to_index(e.tile_x(), e.tile_y());
        self.entityid_to_index.remove(&e.id).unwrap();

        let layer = &mut self.entities[e.layer()];

        layer.remove(&index).unwrap()
    }

    pub fn set_tile_pos(&mut self, e : &entity::Entity, tile_x: usize, tile_y: usize, layer: usize) {
        let mut e = self.remove_entity(e);
        assert!(self.entity(tile_x, tile_y, layer).is_none(), "tile must be free before enity moves to it");

        e.tile_x = tile_x;
        e.tile_y = tile_y;
        e.layer = layer;

        let pos = self.get_tile_foot(tile_x, tile_y);
        let dimensions = self.dimensions.clone();
        let e = self.add_entity(e);
        e.set_pos(Point2::new(pos.x, pos.y - dimensions.y));
    }

}

impl SpriteComponent for Map {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        // layers are made of 3 sections
        // want the index to point to a given section
        let layer_index = self.layer_index * 3;

        let (tile_left, tile_top) = self.point_to_tile(self.camera.left(), self.camera.top());
        let (tile_right, tile_bottom) =
            self.point_to_tile(self.camera.right(), self.camera.bottom());

        sprite.sprite_batch.clear();
        for j in tile_top..=(tile_bottom) {
            for i in tile_left..=(tile_right) {
                // Get actual tile layer
                let tile = self.get_tile(i, j, layer_index);
                if tile > 0 {
                    sprite
                        .sprite_batch
                        .add(self.tile_draw_params(&sprite.uvs, i, j, tile));
                }

                // Get decoration layer tiles
                let tile = self.get_tile(i, j, layer_index + 1);
                if tile > 0 {
                    sprite
                        .sprite_batch
                        .add(self.tile_draw_params(&sprite.uvs, i, j, tile));
                }
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
