use ggez::{
    graphics::{
        self,
        Point2,
    },
};

use map::Map;
use sprite::{Sprite, SpriteComponent};

pub struct Entity {
    pub pos: Point2,
    pub dimensions: Point2,

    pub tile_x: usize,
    pub tile_y: usize,

    frame: usize,
}

impl Entity {
    pub fn new(dimensions: Point2) -> Self {
        Entity{
            pos: Point2::new(0.0, 0.0),
            dimensions,

            tile_x: 0,
            tile_y: 0,

            frame: 0,
        }
    }

    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
    }

    pub fn set_position(&mut self, pos: Point2) {
        self.pos = pos;
    }

    pub fn teleport(&mut self, tile_x: usize, tile_y: usize, map: &Map) {
        let pos = map.get_tile_foot(tile_x, tile_y);
        let x = pos.x - self.dimensions.x / 2.0;
        let y = pos.y - self.dimensions.y;

        self.set_position(Point2::new(x, y));
        self.tile_x = tile_x;
        self.tile_y = tile_y;
    }
}

impl SpriteComponent for Entity {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        if self.frame != sprite.frame() {
            sprite.set_frame(self.frame);
        }
    }
    fn draw_sprite_at(&self) -> graphics::Point2 {
        self.pos.clone()
    }
}
